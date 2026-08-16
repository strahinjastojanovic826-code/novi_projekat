use std::ptr;
use std::sync::atomic::{AtomicIsize, AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub type Task = Box<dyn FnOnce() + Send + 'static>;

// =============================================================================
// 1. CHASE-LEV LOCK-FREE WORK-STEALING DEQUE
// =============================================================================

pub struct WorkStealingQueue<const CAPACITY: usize> {
    buffer: [AtomicPtr<Task>; CAPACITY],
    head: AtomicIsize, // Vrh reda (Top - mesto odakle kradljivci kradu - FIFO)
    tail: AtomicIsize, // Dno reda (Bottom - mesto gde vlasnik gura/skida - LIFO)
}

impl<const CAPACITY: usize> WorkStealingQueue<CAPACITY> {
    pub fn new() -> Self {
        let buffer = std::array::from_fn(|_| AtomicPtr::new(ptr::null_mut()));
        Self {
            buffer,
            head: AtomicIsize::new(0),
            tail: AtomicIsize::new(0),
        }
    }

    /// Vlasnik gura zadatak na dno reda (Push Bottom - LIFO)
    pub fn push(&self, task: Task) -> Result<(), Task> {
        let t = self.tail.load(Ordering::Relaxed);
        let h = self.head.load(Ordering::Acquire);

        if t - h >= CAPACITY as isize {
            return Err(task); // Red je pun!
        }

        let task_ptr = Box::into_raw(Box::new(task));
        let index = (t as usize) % CAPACITY;

        self.buffer[index].store(task_ptr, Ordering::Release);
        self.tail.store(t + 1, Ordering::Release);
        Ok(())
    }

    /// Vlasnik skida svoj najnoviji zadatak sa dna (Pop Bottom - LIFO)
    pub fn pop(&self) -> Option<Task> {
        let t = self.tail.load(Ordering::Relaxed) - 1;
        self.tail.store(t, Ordering::Relaxed);
        
        let h = self.head.load(Ordering::Acquire);

        if t >= h {
            let index = (t as usize) % CAPACITY;
            let task_ptr = self.buffer[index].swap(ptr::null_mut(), Ordering::Acquire);
            
            if !task_ptr.is_null() {
                return Some(unsafe { *Box::from_raw(task_ptr) });
            }
        }

        // Ako je red prazan ili je nastala trka
        self.tail.store(h, Ordering::Relaxed);
        None
    }

    /// Druga nit (Kradljivac) krade najstariji zadatak sa vrha (Steal Top - FIFO)
    pub fn steal(&self) -> Option<Task> {
        loop {
            let h = self.head.load(Ordering::Acquire);
            let t = self.tail.load(Ordering::Acquire);

            if h >= t {
                return None; // Red je prazan, nema šta da se ukrade!
            }

            let index = (h as usize) % CAPACITY;
            let task_ptr = self.buffer[index].swap(ptr::null_mut(), Ordering::Acquire);

            if !task_ptr.is_null() {
                // Pokušaj pomeranja head pointer-a preko CAS-a
                if self
                    .head
                    .compare_exchange_weak(
                        h,
                        h + 1,
                        Ordering::Release,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    return Some(unsafe { *Box::from_raw(task_ptr) });
                } else {
                    // Vraćamo pokazivač ako je druga nit bila brža
                    self.buffer[index].store(task_ptr, Ordering::Release);
                }
            }
        }
    }
}

impl<const CAPACITY: usize> Drop for WorkStealingQueue<CAPACITY> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

unsafe impl<const CAPACITY: usize> Send for WorkStealingQueue<CAPACITY> {}
unsafe impl<const CAPACITY: usize> Sync for WorkStealingQueue<CAPACITY> {}

// =============================================================================
// 2. QUANTUM WORK-STEALING SCHEDULER ENGINE
// =============================================================================

pub struct TaskStealingScheduler<const THREADS: usize, const QUEUE_CAP: usize> {
    queues: Arc<[WorkStealingQueue<QUEUE_CAP>; THREADS]>,
    stolen_count: Arc<AtomicUsize>,
    executed_count: Arc<AtomicUsize>,
}

impl<const THREADS: usize, const QUEUE_CAP: usize> TaskStealingScheduler<THREADS, QUEUE_CAP> {
    pub fn new() -> Self {
        let queues = Arc::new(std::array::from_fn(|_| WorkStealingQueue::new()));
        Self {
            queues,
            stolen_count: Arc::new(AtomicUsize::new(0)),
            executed_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Dodaje zadatak specifičnom radniku
    pub fn spawn_on(&self, worker_id: usize, task: Task) -> Result<(), Task> {
        self.queues[worker_id % THREADS].push(task)
    }

    /// Pokreće sve radničke niti i balansira opterećenje krađom
    pub fn run_all(&self) {
        let mut handles = vec![];

        for id in 0..THREADS {
            let queues_clone = Arc::clone(&self.queues);
            let stolen_counter = Arc::clone(&self.stolen_count);
            let executed_counter = Arc::clone(&self.executed_count);

            handles.push(thread::spawn(move || {
                let mut local_executed = 0;

                loop {
                    // 1. Prvo probaj da uzmeš sopstveni zadatak sa dna (Pop - LIFO)
                    let task = queues_clone[id].pop().or_else(|| {
                        // 2. Ako nemaš svoj, pokušaj da ukradeš sa vrha tuđeg reda (Steal - FIFO)
                        for other_id in 0..THREADS {
                            if other_id == id {
                                continue;
                            }
                            if let Some(stolen_task) = queues_clone[other_id].steal() {
                                stolen_counter.fetch_add(1, Ordering::Relaxed);
                                return Some(stolen_task);
                            }
                        }
                        None
                    });

                    // Ako ima zadatka, izvrši ga
                    if let Some(t) = task {
                        t();
                        local_executed += 1;
                    } else {
                        // Nema zadataka više ni kod koga
                        break;
                    }
                }

                executed_counter.fetch_add(local_executed, Ordering::Relaxed);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn total_stolen(&self) -> usize {
        self.stolen_count.load(Ordering::Relaxed)
    }

    pub fn total_executed(&self) -> usize {
        self.executed_count.load(Ordering::Relaxed)
    }
}