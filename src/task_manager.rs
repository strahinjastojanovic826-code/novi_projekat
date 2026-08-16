use crate::domain::QuquatVal;
use crate::driver::WinQuantumDriver;
use eframe::egui;
use crate::vfs::ProcessFDTable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessStatus {
    Running,
    Paused,
}

#[derive(Debug, Clone)]
pub struct VirtualTask {
    pub pid: u64,
    pub name: String,
    pub priority: u8,
    pub state: String,
}

#[derive(Clone)]
pub struct VirtualProcess {
    pub pid: u32,
    pub name: String,
    pub status: ProcessStatus,
    pub priority: u8,
    pub allocated_ququats: usize, // RAM (Kvati)
    pub storage_mb: usize,        // Virtuelni Disk (MB)
    pub start_index: usize,
    pub total_ticks: u64,
    pub fd_table: ProcessFDTable,
}

impl VirtualProcess {
    /// Izračunava trenutnu CPU potrošnju procesa u %
    pub fn current_cpu_usage(&self) -> f32 {
        if self.status == ProcessStatus::Running {
            (self.priority as f32 * 4.5) + (self.allocated_ququats as f32 * 1.2)
        } else {
            0.0
        }
    }

    /// Izračunava trenutnu GPU potrošnju u % (kvantna jezgra)
    pub fn current_gpu_usage(&self) -> f32 {
        if self.status == ProcessStatus::Running {
            (self.allocated_ququats as f32 * 5.5) + (self.priority as f32 * 2.0)
        } else {
            0.0
        }
    }
}

pub struct VirtualScheduler {
    pub processes: Vec<VirtualProcess>,
    pub tasks: Vec<VirtualTask>,
    next_pid: u32,
    new_name: String,
    new_ququats: usize,
    new_storage: usize,
    new_priority: u8,
    tick_counter: u64,
    pub total_storage_mb: usize, // Ukupan kapacitet virtuelnog diska (npr. 256 MB)
    pub total_ticks: u64,
}

impl VirtualScheduler {
    pub fn new(driver: &WinQuantumDriver) -> Self {
        let mut scheduler: VirtualScheduler = Self {
            processes: Vec::new(),
            tasks: Vec::new(),
            next_pid: 1000,
            new_name: "Quantum_FFT".to_string(),
            new_ququats: 4,
            new_storage: 32,
            new_priority: 3,
            tick_counter: 0,
            total_storage_mb: 256,
            total_ticks: 0,
        };

        //Zasto je ovaj deo ovakav 
        //Pojma nemam

        // Inicijalni procesi sa RAM i Storage alokacijom
        scheduler.spawn("Shor_Factoring", 8, 64, 5, driver);
        scheduler.spawn("Grover_Search", 4, 32, 3, driver);
        scheduler.spawn("Quantum_Key_Dist", 2, 16, 2, driver);

        scheduler.tasks.push(VirtualTask {
        pid: 1,
        name: "Kernel_Init".to_string(),
        priority: 1,
        state: "Running".to_string(),
    });

        scheduler
    }

    pub fn total_allocated_ququats(&self) -> usize {
        self.processes.iter().map(|p| p.allocated_ququats).sum()
    }

    pub fn total_allocated_storage(&self) -> usize {
        self.processes.iter().map(|p| p.storage_mb).sum()
    }

    pub fn global_cpu_usage(&self) -> f32 {
        let sum: f32 = self.processes.iter().map(|p| p.current_cpu_usage()).sum();
        sum.min(100.0)
    }

    pub fn global_gpu_usage(&self) -> f32 {
        let sum: f32 = self.processes.iter().map(|p| p.current_gpu_usage()).sum();
        sum.min(100.0)
    }

    pub fn spawn(&mut self, name: &str, ququats: usize, storage_mb: usize, priority: u8, driver: &WinQuantumDriver) -> bool {
        let allocated_ram = self.total_allocated_ququats();
        let allocated_disk = self.total_allocated_storage();

        if allocated_ram + ququats > 32 || allocated_disk + storage_mb > self.total_storage_mb {
            return false; // Nema dovoljno RAM ili Storage resursa
        }

        let pid = self.next_pid;
        self.next_pid += 1;
        let start_index = allocated_ram;

        for i in start_index..(start_index + ququats) {
            driver.set_ququat(i, QuquatVal::Q01);
        }

        self.processes.push(VirtualProcess {
            pid,
            name: name.to_string(),
            status: ProcessStatus::Running,
            priority,
            allocated_ququats: ququats,
            storage_mb,
            start_index,
            total_ticks: 0,
            fd_table: ProcessFDTable::new(16),
        });

        true
    }

    pub fn kill(&mut self, pid: u32, driver: &WinQuantumDriver) {
        if let Some(pos) = self.processes.iter().position(|p| p.pid == pid) {
            let proc = self.processes.remove(pos);
            for i in proc.start_index..(proc.start_index + proc.allocated_ququats) {
                driver.set_ququat(i, QuquatVal::Q00);
            }
            self.defragment(driver);
        }
    }

    pub fn toggle_pause(&mut self, pid: u32) {
        if let Some(proc) = self.processes.iter_mut().find(|p| p.pid == pid) {
            proc.status = match proc.status {
                ProcessStatus::Running => ProcessStatus::Paused,
                ProcessStatus::Paused => ProcessStatus::Running,
            };
        }
    }

    fn defragment(&mut self, driver: &WinQuantumDriver) {
        let mut current = 0;
        for proc in &mut self.processes {
            if proc.start_index != current {
                for i in 0..proc.allocated_ququats {
                    let val = driver.get_ququat(proc.start_index + i);
                    driver.set_ququat(proc.start_index + i, QuquatVal::Q00);
                    driver.set_ququat(current + i, val);
                }
                proc.start_index = current;
            }
            current += proc.allocated_ququats;
        }
    }

    pub fn tick(&mut self, driver: &WinQuantumDriver) {
        self.tick_counter += 1;
        self.total_ticks += 1;
        for proc in &mut self.processes {
            if proc.status == ProcessStatus::Running {
                proc.total_ticks += 1;
                let speed_divider = (6 - proc.priority as u64) * 10;
                if self.tick_counter % speed_divider == 0 {
                    for i in proc.start_index..(proc.start_index + proc.allocated_ququats) {
                        let current = driver.get_ququat(i);
                        driver.set_ququat(i, current.next());
                    }
                }
            }
        }
    }

    pub fn render_ui(&mut self, ui: &mut egui::Ui, driver: &WinQuantumDriver) {
        let total_ram = self.total_allocated_ququats();
        let total_disk = self.total_allocated_storage();
        let cpu = self.global_cpu_usage();
        let gpu = self.global_gpu_usage();
        let ram_pct = (total_ram as f32 / 32.0) * 100.0;
        let disk_pct = (total_disk as f32 / self.total_storage_mb as f32) * 100.0;

        ui.heading("📊 Task Manager - Sistemski Resursi");
        ui.separator();

        // =======================================================
        // METRIKA ZAUZEĆA: CPU, GPU, RAM, STORAGE
        // =======================================================
        ui.group(|ui| {
            ui.label(egui::RichText::new("⚡ PERFORMANCE & RESOURCE MONITOR").strong());
            ui.separator();

            ui.columns(4, |cols| {
                // 1. CPU
                cols[0].vertical(|ui| {
                    ui.label(egui::RichText::new("💻 CPU").strong());
                    ui.label(format!("{:.1}%", cpu));
                    ui.add(egui::ProgressBar::new(cpu / 100.0));
                });

                // 2. GPU
                cols[1].vertical(|ui| {
                    ui.label(egui::RichText::new("🎨 GPU (Compute)").strong());
                    ui.label(format!("{:.1}%", gpu));
                    ui.add(egui::ProgressBar::new(gpu / 100.0));
                });

                // 3. RAM (Kvati)
                cols[2].vertical(|ui| {
                    ui.label(egui::RichText::new("🧠 RAM (Kvati)").strong());
                    ui.label(format!("{} / 32 ({:.0}%)", total_ram, ram_pct));
                    ui.add(egui::ProgressBar::new(ram_pct / 100.0));
                });

                // 4. STORAGE
                cols[3].vertical(|ui| {
                    ui.label(egui::RichText::new("💾 Storage (Q-Drive)").strong());
                    ui.label(format!("{} / {} MB", total_disk, self.total_storage_mb));
                    ui.add(egui::ProgressBar::new(disk_pct / 100.0));
                });
            });
        });

        ui.add_space(10.0);

        // Pokretanje procesa sa parametrima za RAM i Storage
        ui.group(|ui| {
            ui.label(egui::RichText::new("➕ Pokreni Novi Proces").strong());
            ui.horizontal(|ui| {
                ui.label("Ime:");
                ui.text_edit_singleline(&mut self.new_name);
                ui.label("RAM (Kvati):");
                ui.add(egui::DragValue::new(&mut self.new_ququats).clamp_range(1..=(32 - total_ram)));
                ui.label("Storage (MB):");
                ui.add(egui::DragValue::new(&mut self.new_storage).clamp_range(8..=(self.total_storage_mb - total_disk)));
                ui.label("Prio (1-5):");
                ui.add(egui::DragValue::new(&mut self.new_priority).clamp_range(1..=5));

                if ui.button("Lansiraj").clicked() {
                    self.spawn(&self.new_name.clone(), self.new_ququats, self.new_storage, self.new_priority, driver);
                }
            });
        });

        ui.add_space(10.0);

        // Tabela procesa sa detaljnim prikazom potrošnje resursa po procesu
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("virt_resource_grid")
                .striped(true)
                .spacing([12.0, 8.0])
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("PID").strong());
                    ui.label(egui::RichText::new("Proces").strong());
                    ui.label(egui::RichText::new("Status").strong());
                    ui.label(egui::RichText::new("CPU %").strong());
                    ui.label(egui::RichText::new("GPU %").strong());
                    ui.label(egui::RichText::new("RAM (Kvati)").strong());
                    ui.label(egui::RichText::new("Storage").strong());
                    ui.label(egui::RichText::new("Akcija").strong());
                    ui.end_row();

                    let mut to_kill = None;
                    let mut to_toggle = None;

                    for proc in &self.processes {
                        ui.label(format!("{}", proc.pid));
                        ui.label(&proc.name);

                        let status_text = match proc.status {
                            ProcessStatus::Running => egui::RichText::new("RUNNING").color(egui::Color32::GREEN),
                            ProcessStatus::Paused => egui::RichText::new("PAUSED").color(egui::Color32::YELLOW),
                        };
                        ui.label(status_text);

                        ui.label(format!("{:.1}%", proc.current_cpu_usage()));
                        ui.label(format!("{:.1}%", proc.current_gpu_usage()));
                        ui.label(format!("{} Q", proc.allocated_ququats));
                        ui.label(format!("{} MB", proc.storage_mb));

                        ui.horizontal(|ui| {
                            let btn_label = if proc.status == ProcessStatus::Running { "Pauziraj" } else { "Nastavi" };
                            if ui.button(btn_label).clicked() {
                                to_toggle = Some(proc.pid);
                            }
                            if ui.button(egui::RichText::new("Ubij").color(egui::Color32::RED)).clicked() {
                                to_kill = Some(proc.pid);
                            }
                        });
                        ui.end_row();
                    }

                    if let Some(pid) = to_toggle {
                        self.toggle_pause(pid);
                    }
                    if let Some(pid) = to_kill {
                        self.kill(pid, driver);
                    }
                });
        });
    }
}