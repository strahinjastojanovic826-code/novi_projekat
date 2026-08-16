use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

// =============================================================================
// 1. LOCK-FREE TREIBER STACK (Atomska stitasta struktura za proizvoljne tipove)
// =============================================================================

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
    len: AtomicUsize,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            len: AtomicUsize::new(0),
        }
    }

    /// Guranje elementa na vrh steka bez zaključavanja (Lock-Free Push)
    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            // Učitavamo trenutni vrh steka
            let current_head = self.head.load(Ordering::Relaxed);
            
            // Postavljamo novom čvoru da pokazuje na trenutni vrh
            unsafe {
                (*new_node).next = current_head;
            }

            // Atomska CAS (Compare-And-Swap) operacija:
            // Ako je `self.head` i dalje jednaka `current_head`, zameni je sa `new_node`.
            if self
                .head
                .compare_exchange_weak(
                    current_head,
                    new_node,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                self.len.fetch_add(1, Ordering::Relaxed);
                break;
            }
            // Ako je druga nit izmenila `head` u međuvremenu, petlja se ponavlja bez blokiranja!
        }
    }

    /// Skidanje elementa sa vrha steka bez zaključavanja (Lock-Free Pop)
    pub fn pop(&self) -> Option<T> {
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            if current_head.is_null() {
                return None;
            }

            let next_node = unsafe { (*current_head).next };

            // Atomska zamena vrha
            if self
                .head
                .compare_exchange_weak(
                    current_head,
                    next_node,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                self.len.fetch_sub(1, Ordering::Relaxed);
                let boxed_node = unsafe { Box::from_raw(current_head) };
                return Some(boxed_node.data);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::Relaxed)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Sync> Sync for LockFreeStack<T> {}

// =============================================================================
// 2. LOCK-FREE ATOMIC RING BUFFER (Kružni bafer za IPC i prekide)
// =============================================================================

pub struct LockFreeRingBuffer<T, const N: usize> {
    buffer: [AtomicPtr<T>; N],
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T, const N: usize> LockFreeRingBuffer<T, N> {
    pub fn new() -> Self {
        // Inicijalizujemo niz praznih atomskih pokazivača
        let buffer = std::array::from_fn(|_| AtomicPtr::new(ptr::null_mut()));

        Self {
            buffer,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Upisuje podatak na kraj reda ako ima mesta
    pub fn enqueue(&self, item: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        // Provera da li je bafer pun
        if tail.wrapping_sub(head) >= N {
            return Err(item);
        }

        let item_ptr = Box::into_raw(Box::new(item));
        let index = tail % N;

        self.buffer[index].store(item_ptr, Ordering::Release);
        self.tail.fetch_add(1, Ordering::Release);

        Ok(())
    }

    /// Izvlači podatak sa početka reda
    pub fn dequeue(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        // Provera da li je bafer prazan
        if head == tail {
            return None;
        }

        let index = head % N;
        let item_ptr = self.buffer[index].swap(ptr::null_mut(), Ordering::Acquire);

        if item_ptr.is_null() {
            return None;
        }

        self.head.fetch_add(1, Ordering::Release);
        let item = unsafe { *Box::from_raw(item_ptr) };
        Some(item)
    }

    pub fn len(&self) -> usize {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Relaxed);
        tail.wrapping_sub(head)
    }
}

impl<T, const N: usize> Drop for LockFreeRingBuffer<T, N> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

unsafe impl<T: Send, const N: usize> Send for LockFreeRingBuffer<T, N> {}
unsafe impl<T: Sync, const N: usize> Sync for LockFreeRingBuffer<T, N> {}

// =============================================================================
// 3. LOCK-FREE FIFO QUEUE (Michael-Scott MPMC Queue Algoritam)
// =============================================================================

struct QueueNode<T> {
    data: Option<T>,
    next: AtomicPtr<QueueNode<T>>,
}

pub struct LockFreeQueue<T> {
    head: AtomicPtr<QueueNode<T>>,
    tail: AtomicPtr<QueueNode<T>>,
}

impl<T> LockFreeQueue<T> {
    pub fn new() -> Self {
        // Dummy čvor koji sprečava trke između praznog head-a i tail-a
        let dummy = Box::into_raw(Box::new(QueueNode {
            data: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    /// Dodavanje na kraj reda (FIFO Push / Enqueue) bez brave
    pub fn enqueue(&self, data: T) {
        let new_node = Box::into_raw(Box::new(QueueNode {
            data: Some(data),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            // Provera konzistentnosti tail pokazivača
            if tail == self.tail.load(Ordering::Relaxed) {
                if next.is_null() {
                    // Pokušaj kačenja novog čvora na kraj liste preko CAS-a
                    if unsafe {
                        (*tail)
                            .next
                            .compare_exchange_weak(
                                ptr::null_mut(),
                                new_node,
                                Ordering::Release,
                                Ordering::Relaxed,
                            )
                            .is_ok()
                    } {
                        // Uspeh! Pokušavamo da pomerimo tail na novi čvor
                        let _ = self.tail.compare_exchange_weak(
                            tail,
                            new_node,
                            Ordering::Release,
                            Ordering::Relaxed,
                        );
                        return;
                    }
                } else {
                    // Tail je zaostajao! Pomažemo drugoj niti da ga pomeri unapred
                    let _ = self.tail.compare_exchange_weak(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                }
            }
        }
    }

    /// Uzimanje sa početka reda (FIFO Pop / Dequeue) bez brave
    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == self.head.load(Ordering::Relaxed) {
                if head == tail {
                    if next.is_null() {
                        return None; // Red je prazan
                    }
                    // Tail zaostaje, pomažemo da se pomeri
                    let _ = self.tail.compare_exchange_weak(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                } else {
                    if next.is_null() {
                        continue;
                    }

                    // Uzimamo podatak iz čvora pre samog zamene head-a
                    let data = unsafe { (*next).data.take() };

                    // Pokušaj pomeranja head-a na sledeći čvor
                    if self
                        .head
                        .compare_exchange_weak(
                            head,
                            next,
                            Ordering::Release,
                            Ordering::Relaxed,
                        )
                        .is_ok()
                    {
                        // Oslobađamo stari dummy čvor iz memorije
                        unsafe {
                            let _ = Box::from_raw(head);
                        }
                        return data;
                    }
                }
            }
        }
    }
}

impl<T> Drop for LockFreeQueue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        let head = self.head.load(Ordering::Relaxed);
        if !head.is_null() {
            unsafe {
                let _ = Box::from_raw(head);
            }
        }
    }
}

unsafe impl<T: Send> Send for LockFreeQueue<T> {}
unsafe impl<T: Sync> Sync for LockFreeQueue<T> {}