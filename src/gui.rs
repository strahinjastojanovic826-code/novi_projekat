use eframe::egui;
use crate::domain::QuquatVal;
use crate::driver::WinQuantumDriver;
use crate::task_manager::VirtualScheduler;
use crate::vfs::{QuquatVFS, NodeType, TOTAL_DISK_BLOCKS};
use crate::audio::QuantumAudioEngine;
use crate::vm::{QuantumVM, Instruction, Opcode};
use crate::network::{NetworkEngine, QuquatPacket, PacketType};
use crate::db::QuquatDB;
use crate::terminal::{TerminalEngine, LineType, command::CommandDispatcher};
use crate::container::{HypervisorEngine, ContainerStatus};
use crate::video::{VideoEngine, PlaybackState};
use crate::image_engine::QuantumImageEngine;
use crate::compiler::QuantumScriptCompiler;
use crate::search_engine::QuantumSearchEngine;
use crate::crypto::QuantumCryptoEngine;
use crate::nvram::QuantumNvramEngine;
use crate::boot_manager::QuantumBootEngine;
use crate::streaming::QuantumEventStreamEngine;
use crate::hft::{orderbook::OrderSide, QuantumHftEngine};
use crate::gis::{spatial::GeoPoint, QuantumSpatialEngine};
use crate::tsdb::{metrics::AggregationFunc, QuantumTimeSeriesEngine};
use crate::linter::{rules::LintLevel, QuantumCodeLinterEngine};
use crate::pathfinding::{graph::CellType, QuantumPathfinderEngine};
use crate::compression::QuantumArchiverEngine;
use crate::virtual_input::QuantumVirtualInputEngine;
use crate::gop_vga::QuantumGopVgaEngine;
use crate::protocol_drivers::QuantumProtocolDrivers;
use crate::efi_fat32::QuantumEfiFatEngine;
use crate::telemetry::{ring_buffer::LogLevel, QuantumTelemetryEngine};
use crate::shell_automation::QuantumShellEngine;
use crate::pkg_loader::QuantumPkgEngine;
use crate::ipc::{signals::Signal, QuantumIpcEngine};

pub struct QuantumOSWinApp {
    driver: WinQuantumDriver,
    scheduler: VirtualScheduler,
    vfs: QuquatVFS,
    pub audio: QuantumAudioEngine,
    pub vm: QuantumVM,
    pub net: NetworkEngine,
    pub db: QuquatDB,
    pub terminal: TerminalEngine, 
    pub container_engine: HypervisorEngine,
    pub video_engine: VideoEngine,
    pub img_engine: QuantumImageEngine,
    pub compiler: QuantumScriptCompiler,
    pub search_engine: QuantumSearchEngine,
    pub crypto_engine: QuantumCryptoEngine,
    pub nvram_engine: QuantumNvramEngine,
    pub boot_engine: QuantumBootEngine,
    pub stream_engine: QuantumEventStreamEngine,
    pub hft_engine: QuantumHftEngine,
    pub gis_engine: QuantumSpatialEngine,
    pub tsdb_engine: QuantumTimeSeriesEngine,
    pub linter_engine: QuantumCodeLinterEngine,
    pub pathfinder_engine: QuantumPathfinderEngine,
    pub archiver_engine: QuantumArchiverEngine,
    pub input_engine: QuantumVirtualInputEngine,
    pub gop_engine: QuantumGopVgaEngine,
    pub proto_drivers: QuantumProtocolDrivers,
    pub fat32_engine: QuantumEfiFatEngine,
    pub telemetry_engine: QuantumTelemetryEngine,
    pub shell_engine: QuantumShellEngine,
    pub ipc_engine: QuantumIpcEngine,
    pub pkg_engine: QuantumPkgEngine,
    
    // Stanja prozora (da li su otvoreni ili zatvoreni na 'X')
    is_task_manager_open: bool,
    is_register_open: bool,
    is_vfs_open: bool,
    is_audio_open: bool,
    is_vm_open: bool,
    is_net_open: bool,
    is_db_open: bool,
    is_terminal_open: bool,
    is_container_open: bool,
    is_hypervisor_open: bool,
    pub is_video_open: bool,
    pub is_img_open: bool,
    pub is_compiler_open: bool,
    pub is_search_open: bool,
    pub is_crypto_open: bool,
    pub is_nvram_open: bool,
    pub is_boot_open: bool,
    pub is_stream_open: bool,
    pub is_hft_open: bool,
    pub is_gis_open: bool,
    pub is_tsdb_open: bool,
    pub is_linter_open: bool,
    pub is_pathfinder_open: bool,
    pub is_archiver_open: bool,
    pub is_input_open: bool,
    pub is_gop_open: bool,
    pub is_proto_drivers_open: bool,
    pub is_fat32_open: bool,
    pub is_telemetry_open: bool,
    pub is_shell_open: bool,
    pub is_ipc_open: bool,
    pub is_pkg_open: bool,
    new_item_name: String,
    db_key_input: String,
    new_cnt_name: String,
    selected_image: String,
    pub search_query_input: String,
    pub new_doc_title_input: String,
    pub new_doc_content_input: String,
    pub crypto_input_text: String,
    pub crypto_key_input: String,
    pub crypto_sign_text: String,
    pub crypto_signature: String,
    pub nvram_key_input: String,
    pub nvram_val_input: String,
    pub stream_pub_topic: String,
    pub stream_pub_key: String,
    pub stream_pub_val: String,
    pub selected_metric: String,
    pub arc_file_name_input: String,
    pub arc_file_content_input: String,
    pub arc_test_text_input: String,
    pub vinput_key_text: String,
    pub nvme_lba_input: String,
    pub nvme_data_input: String,
    pub selected_efi_file: String,
    pub klog_subsystem_input: String,
    pub klog_msg_input: String,
    pub usb_port_input: u8,
    pub usb_data_input: String,
    pub hft_trade_price: f64,
    pub hft_trade_qty: u32,
    pub gis_search_lat: f64,
    pub gis_search_lon: f64,
    pub gis_search_radius: f64,
    pub new_metric_val: f64,
    pub shell_input_pipeline: String,
    pub shell_new_env_key: String,
    pub shell_new_env_val: String,
    pub ipc_sender_pid: u32,
    pub ipc_target_pid: u32,
    pub ipc_msg_text: String,
    pub ipc_pipe_name: String,
    pub ipc_pipe_data: String, 
    pub pkg_search_input: String,
    new_cnt_ram: usize,
    selected_inode: Option<usize>,
    syscall_logs: Vec<String>,
    video_texture: Option<egui::TextureHandle>,
}

// Pomoćni extension trait za lep prikaz metrika
trait MetricUi {
    fn metric(&mut self, label: &str, value: String);
}

impl MetricUi for egui::Ui {
    fn metric(&mut self, label: &str, value: String) {
        self.vertical(|ui| {
            ui.small(label);
            ui.label(egui::RichText::new(value).strong().size(16.0));
        });
    }
}

impl QuantumOSWinApp {
    pub fn new() -> Self {
        let driver = WinQuantumDriver::new();
        let scheduler = VirtualScheduler::new(&driver);
        let vfs = QuquatVFS::new();
        let audio = QuantumAudioEngine::new();
        let vm = QuantumVM::new();
        let net = NetworkEngine::new();
        let db = QuquatDB::new();
        let terminal = TerminalEngine::new();
        let container_engine = HypervisorEngine::new();
        let video_engine = VideoEngine::new();
        let img_engine = QuantumImageEngine::new();
        let compiler = QuantumScriptCompiler::new();
        let search_engine = QuantumSearchEngine::new();
        let crypto_engine = QuantumCryptoEngine::new();
        let nvram_engine = QuantumNvramEngine::new();
        let boot_engine = QuantumBootEngine::new();
        let stream_engine = QuantumEventStreamEngine::new();
        let hft_engine = QuantumHftEngine::new();
        let gis_engine = QuantumSpatialEngine::new();
        let linter_engine = QuantumCodeLinterEngine::new();
        let pathfinder_engine = QuantumPathfinderEngine::new();
        let archiver_engine = QuantumArchiverEngine::new();
        let input_engine = QuantumVirtualInputEngine::new();
        let gop_engine = QuantumGopVgaEngine::new();
        let proto_drivers = QuantumProtocolDrivers::new();
        let fat32_engine = QuantumEfiFatEngine::new();
        let tsdb_engine = QuantumTimeSeriesEngine::new();
        let telemetry_engine = QuantumTelemetryEngine::new();
        let shell_engine = QuantumShellEngine::new();
        let pkg_engine = QuantumPkgEngine::new();
        let ipc_engine = QuantumIpcEngine::new();
        Self {
            driver,
            scheduler,
            vfs,
            audio,
            vm,
            net,
            db,
            terminal,
            container_engine,
            video_engine,
            img_engine,
            compiler,
            search_engine,
            crypto_engine,
            nvram_engine,
            boot_engine,
            stream_engine,
            hft_engine,
            gis_engine,
            linter_engine,
            pathfinder_engine,
            archiver_engine,
            input_engine,
            gop_engine,
            proto_drivers,
            fat32_engine,
            tsdb_engine,
            telemetry_engine,
            shell_engine,
            ipc_engine,
            pkg_engine,
            is_task_manager_open: false, // Inicijalno otvoren
            is_register_open: false,     // Inicijalno otvoren
            is_vfs_open: false,
            is_audio_open: false,
            is_vm_open: false,
            is_net_open: false,
            is_db_open: false,
            is_terminal_open: false,
            is_container_open: false,
            is_hypervisor_open: false,
            is_video_open: false,
            is_img_open: false,
            is_compiler_open: false,
            is_search_open: false,
            is_crypto_open: false,
            is_nvram_open: false,
            is_boot_open: false,
            is_stream_open: false,
            is_hft_open: false,
            is_gis_open: false,
            is_tsdb_open: false,
            is_linter_open: false,
            is_pathfinder_open: false,
            is_archiver_open: false,
            is_input_open: false,
            is_gop_open: false,
            is_proto_drivers_open: false,
            is_fat32_open: false,
            is_telemetry_open: false,
            is_shell_open: false,
            is_pkg_open: false,
            is_ipc_open: false,
            search_query_input: String::new(),
            new_doc_title_input: String::new(),
            new_doc_content_input: String::new(),
            crypto_signature: String::new(),
            nvram_key_input: String::new(),
            nvram_val_input: String::new(),
            selected_efi_file: String::new(),
            pkg_search_input: String::new(),
            new_cnt_name: "my_service".to_string(),
            selected_image: "ququat/ubuntu-mini:latest".to_string(),
            new_cnt_ram: 128,
            hft_trade_price: 101.0,
            hft_trade_qty: 10, 
            gis_search_lat: 44.7866, // Beograd koordinate
            gis_search_lon: 20.4489,
            gis_search_radius: 100.0,
            new_metric_val: 50.0,
            new_item_name: "test_program.qbin".to_string(),
            db_key_input: "user_session".to_string(),
            crypto_input_text: "Tajna Poruka".to_string(),
            crypto_key_input: "moj_tajni_kljuc".to_string(),
            crypto_sign_text: "Dokument za potpisivanje".to_string(),
            stream_pub_topic: "system.events".to_string(),
            stream_pub_key: "INFO".to_string(),
            stream_pub_val: "Test event payload".to_string(),
            selected_metric: "system.cpu.usage".to_string(),
            arc_file_name_input: "data_dump.bin".to_string(),
            vinput_key_text: "A".to_string(),
            klog_subsystem_input: "CUSTOM_DRV".to_string(),
            klog_msg_input: "Ručno poslat log u Kernel Ring Buffer".to_string(),
            arc_file_content_input: "000000001111111122222222AAAAAA".to_string(),
            arc_test_text_input: "TEST_ENTROPY_QUANTUM_OS_DATA_STREAM".to_string(),
            nvme_lba_input: "0".to_string(),
            nvme_data_input: "QuantumOS_Kernel_Payload".to_string(),
            usb_port_input: 1,
            usb_data_input: "Ping_Device".to_string(),
            shell_input_pipeline: "echo Pozdrav iz $USER | tr_upper > /output.txt".to_string(),
            shell_new_env_key: "APP_ENV".to_string(),
            shell_new_env_val: "PROD".to_string(),
            selected_inode: None,
            video_texture: None,
            syscall_logs: vec!["[KERNEL] VFS & Disk Block System Inicijalizovan.".to_string()],
            ipc_sender_pid: 1001,
            ipc_target_pid: 1002,
            ipc_msg_text: "Zdravo iz IPC modula!".to_string(),
            ipc_pipe_name: "sys_pipe".to_string(),
            ipc_pipe_data: "Strim podaci".to_string(),
        }
    }
}

impl eframe::App for QuantumOSWinApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Glavni impuls virtuelnog OS-a
        self.scheduler.tick(&self.driver);
        self.audio.sync_with_hardware(&self.driver);
        self.vm.tick(&self.driver);
        self.net.poll(&self.driver);
        ctx.request_repaint();

        // =======================================================
        // 1. TASKBAR NA DNU (Multi-tasking traka sa ikonicama)
        // =======================================================
        egui::TopBottomPanel::bottom("os_taskbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🪟 QuantumOS").strong());
                ui.separator();

                // Ikonica za Task Manager
                let tm_btn_text = if self.is_task_manager_open {
                    "📊 Task Manager (Aktivan)"
                } else {
                    "📊 Task Manager"
                };
                if ui.selectable_label(self.is_task_manager_open, tm_btn_text).clicked() {
                    self.is_task_manager_open = !self.is_task_manager_open;
                }

                // Ikonica za Kvantni Registar
                let reg_btn_text = if self.is_register_open {
                    "🎛 Kvantni Registar (Aktivan)"
                } else {
                    "🎛 Kvantni Registar"
                };
                if ui.selectable_label(self.is_register_open, reg_btn_text).clicked() {
                    self.is_register_open = !self.is_register_open;
                }
                if ui.selectable_label(self.is_vfs_open, "📁 VFS Inode & Disk Explorer").clicked() {
                    self.is_vfs_open = !self.is_vfs_open;
                }
                if ui.selectable_label(self.is_audio_open, "🔊 Kvantni Synth").clicked() {
                    self.is_audio_open = !self.is_audio_open;
                }
                if ui.selectable_label(self.is_vm_open, "⚡ Custom VM & JIT").clicked() {
                    self.is_vm_open = !self.is_vm_open;
                }
                if ui.selectable_label(self.is_net_open, "🌐 QNet Mreža").clicked() {
                    self.is_net_open = !self.is_net_open;
                }
                if ui.selectable_label(self.is_db_open, "🗄 QStore Baza").clicked() {
                    self.is_db_open = !self.is_db_open;
                }
                if ui.selectable_label(self.is_terminal_open, "💻 Terminal (QShell)").clicked() {
                    self.is_terminal_open = !self.is_terminal_open;
                }
                if ui.selectable_label(self.is_hypervisor_open, "🐳 Hypervisor / QDocker").clicked() {
                   self.is_hypervisor_open = !self.is_hypervisor_open;
                }
                if ui.selectable_label(self.is_video_open, "🎬 Video Player").clicked() {
                   self.is_video_open = !self.is_video_open;
                }
                if ui.selectable_label(self.is_img_open, "🖼 Image Studio").clicked() {
                   self.is_img_open = !self.is_img_open;
                }
                if ui.selectable_label(self.is_compiler_open, "⚙ QScript IDE").clicked() {
                   self.is_compiler_open = !self.is_compiler_open;
                }
                if ui.selectable_label(self.is_search_open, "🔍 QuantumSearch").clicked() {
                   self.is_search_open = !self.is_search_open;
                }
                if ui.selectable_label(self.is_crypto_open, "🔐 Crypto & TLS Studio").clicked() {
                    self.is_crypto_open = !self.is_crypto_open;
                }               
            });
        });

        // =======================================================
        // 2. POZADINA DESKTOP-A
        // =======================================================
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🖥 Virtuelna Radna Površina");
            ui.label("Klikni na ikonicu u donjoj traci da otvoriš ili zatvoriš aplikacije.");
            
            // Ikonice na radnoj površini
            ui.add_space(20.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button(egui::RichText::new("📊\nTask Manager").size(16.0)).clicked() {
                    self.is_task_manager_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🎛\nRegistar").size(16.0)).clicked() {
                    self.is_register_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("📁\nVFS & Disk Explorer").size(16.0)).clicked() {
                    self.is_vfs_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🎶\nAudio Register").size(16.0)).clicked() {
                    self.is_audio_open = true;
                }  
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("⚡\nVM").size(16.0)).clicked() {
                    self.is_vm_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🌐\nNETWORK").size(16.0)).clicked() {
                    self.is_net_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("💾\nDB").size(16.0)).clicked() {
                    self.is_db_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("💻\nTerminal Shell").size(16.0)).clicked() {
                    self.is_terminal_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🐳\nHypervisor Container").size(16.0)).clicked() {
                    self.is_hypervisor_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🎥\nVideo").size(16.0)).clicked() {
                    self.is_video_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🖼\nIMAGE").size(16.0)).clicked() {
                    self.is_img_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("⚙\nCompiler").size(16.0)).clicked() {
                    self.is_compiler_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🔍\n QuantumSearch").size(16.0)).clicked() {
                    self.is_search_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🔐\n Crypto & TLS Studio").size(16.0)).clicked() {
                    self.is_crypto_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("⚙\nUEFI NVRAM").size(16.0)).clicked() {
                    self.is_nvram_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🚀\nBoot Manager").size(16.0)).clicked() {
                    self.is_boot_open = true;
                }
            
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("📡\nEvent Stream (Kafka)").size(16.0)).clicked() {
                    self.is_stream_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("⚡\nHFT Trading Engine").size(16.0)).clicked() {
                    self.is_hft_open = true;
                }  
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🗺️\nGIS Spatial Engine").size(16.0)).clicked() {
                    self.is_gis_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("📈\nTime-Series DB").size(16.0)).clicked() {
                    self.is_tsdb_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🔍\nCode Linter & Formatter").size(16.0)).clicked() {
                    self.is_linter_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🗺\nPathfinding (A*)").size(16.0)).clicked() {
                    self.is_pathfinder_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("📦\nArchiver & Entropy").size(16.0)).clicked() {
                    self.is_archiver_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🎮\nVirtual Input").size(16.0)).clicked() {
                    self.is_input_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🎨\nGOP / VGA Display").size(16.0)).clicked() {
                    self.is_gop_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🔌\nUSB & NVMe Drivers").size(16.0)).clicked() {
                    self.is_proto_drivers_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("💾\nFAT32 / ESP").size(16.0)).clicked() {
                    self.is_fat32_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("📊\nTelemetry & dmesg").size(16.0)).clicked() {
                    self.is_telemetry_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🐚\nShell Pipelines & Scripting").size(16.0)).clicked() {
                    self.is_shell_open = true;
                }
                ui.add_space(15.0);           
                if ui.button(egui::RichText::new("📦\nPkg Manager & Loader").size(16.0)).clicked() {
                    self.is_pkg_open = true;
                }
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("🔄\nIPC & Signals").size(16.0)).clicked() {
                    self.is_ipc_open = true;
                }
            });
        });

// =======================================================
// 3. MULTITASKING PROZOR: TASK MANAGER (sa 'X' dugmetom)
// =======================================================
if self.is_task_manager_open {
    let mut is_open = self.is_task_manager_open;
    
    egui::Window::new("📊 Virtuelni Task Manager")
        .open(&mut is_open) // Sada pozajmljuje lokalnu promenljivu!
        .default_size([650.0, 400.0])
        .show(ctx, |ui| {
            self.scheduler.render_ui(ui, &self.driver);
        });

    self.is_task_manager_open = is_open; // Sinhronizujemo nazad sa self
}

// =======================================================
// 4. MULTITASKING PROZOR: KVANTNI REGISTAR (sa 'X' dugmetom)
// =======================================================
if self.is_register_open {
    let mut is_open = self.is_register_open;

    egui::Window::new("🎛 Atomski Kvantni Registar (32 Kvata)")
        .open(&mut is_open) // Sada pozajmljuje lokalnu promenljivu!
        .default_size([500.0, 350.0])
        .show(ctx, |ui| {
            self.render_register_grid(ui);
        });

    self.is_register_open = is_open; // Sinhronizujemo nazad sa self
}

      // 5. PROZOR: PRAVI VFS & DISK EXPLORER
        if self.is_vfs_open {
            let mut is_open = self.is_vfs_open;
            egui::Window::new("📁 Kernel VFS (Inode Arena & Sector Disk)")
                .open(&mut is_open)
                .default_size([720.0, 520.0])
                .show(ctx, |ui| {
                    self.render_vfs_panel(ui);
                });
            self.is_vfs_open = is_open;
        }

        if self.is_audio_open {
    let mut is_open = self.is_audio_open;
    egui::Window::new("🔊 Kvantni DSP Audio Sintisajzer")
        .open(&mut is_open)
        .default_size([450.0, 300.0])
        .show(ctx, |ui| {
            self.render_audio_panel(ui);
        });
    self.is_audio_open = is_open;
}

if self.is_vm_open {
    let mut is_open = self.is_vm_open;
    egui::Window::new("⚡ Custom Bytecode VM & JIT Compiler")
        .open(&mut is_open)
        .default_size([500.0, 380.0])
        .show(ctx, |ui| {
            self.render_vm_panel(ui);
        });
    self.is_vm_open = is_open;
}

if self.is_net_open {
    let mut is_open = self.is_net_open;
    egui::Window::new("🌐 QNet Protocol Analyzer & Socket Monitor")
        .open(&mut is_open)
        .default_size([580.0, 420.0])
        .show(ctx, |ui| {
            self.render_net_panel(ui);
        });
    self.is_net_open = is_open;
}

if self.is_db_open {
    let mut is_open = self.is_db_open;
    egui::Window::new("🗄 QStore In-Memory Engine & Transactions")
        .open(&mut is_open)
        .default_size([600.0, 440.0])
        .show(ctx, |ui| {
            self.render_db_panel(ui);
        });
    self.is_db_open = is_open;
}

if self.is_terminal_open {
    let mut is_open = self.is_terminal_open;
    egui::Window::new("💻 QShell Terminal Emulator")
        .open(&mut is_open)
        .default_size([650.0, 400.0])
        .show(ctx, |ui| {
            self.render_terminal_panel(ui);
        });
    self.is_terminal_open = is_open;
}

if self.is_hypervisor_open {
    let mut is_open = self.is_hypervisor_open;
    egui::Window::new("🐳 QDocker Hypervisor Engine")
        .open(&mut is_open)
        .default_size([700.0, 450.0])
        .show(ctx, |ui| {
            self.render_hypervisor_panel(ui);
        });
    self.is_hypervisor_open = is_open;
}

if self.is_video_open {
    let mut is_open = self.is_video_open;
    egui::Window::new("🎬 QVideo Codec Player")
        .open(&mut is_open)
        .default_size([400.0, 320.0])
        .show(ctx, |ui| {
            self.render_video_panel(ctx, ui);
        });
    self.is_video_open = is_open;
}

if self.is_img_open {
    let mut is_open = self.is_img_open;
    egui::Window::new("🖼 Quantum Image Processing Studio")
        .open(&mut is_open)
        .default_size([650.0, 480.0])
        .show(ctx, |ui| {
            self.render_image_panel(ctx, ui);
        });
    self.is_img_open = is_open;
}

if self.is_compiler_open {
    let mut is_open = self.is_compiler_open;
    egui::Window::new("⚙ QuantumScript Compiler & AST Studio")
        .open(&mut is_open)
        .default_size([700.0, 500.0])
        .show(ctx, |ui| {
            self.render_compiler_panel(ui);
        });
    self.is_compiler_open = is_open;
}

if self.is_search_open {
    let mut is_open = self.is_search_open;
    egui::Window::new("🔍 QuantumSearch Engine (Elasticsearch style)")
        .open(&mut is_open)
        .default_size([650.0, 450.0])
        .show(ctx, |ui| {
            self.render_search_panel(ui);
        });
    self.is_search_open = is_open;
}

if self.is_crypto_open {
    let mut is_open = self.is_crypto_open;
    egui::Window::new("🔐 Quantum Crypto & TLS 1.3 Engine")
        .open(&mut is_open)
        .default_size([700.0, 480.0])
        .show(ctx, |ui| {
            self.render_crypto_panel(ui);
        });
    self.is_crypto_open = is_open;
}

if self.is_nvram_open {
    let mut is_open = self.is_nvram_open;
    egui::Window::new("⚙ UEFI NVRAM Settings Manager")
        .open(&mut is_open)
        .default_size([650.0, 420.0])
        .show(ctx, |ui| {
            self.render_nvram_panel(ui);
        });
    self.is_nvram_open = is_open;
}

if self.is_boot_open {
    let mut is_open = self.is_boot_open;
    egui::Window::new("🚀 Quantum OS Boot Order Manager")
        .open(&mut is_open)
        .default_size([650.0, 440.0])
        .show(ctx, |ui| {
            self.render_boot_panel(ui);
        });
    self.is_boot_open = is_open;
}

if self.is_stream_open {
    let mut is_open = self.is_stream_open;
    egui::Window::new("📡 Quantum Event Streaming Engine (Distributed Log)")
        .open(&mut is_open)
        .default_size([840.0, 520.0])
        .show(ctx, |ui| {
            self.render_stream_panel(ui);
        });
    self.is_stream_open = is_open;
}

if self.is_hft_open {
    let mut is_open = self.is_hft_open;
    egui::Window::new("⚡ Quantum HFT Limit Order Book & Matching Engine")
        .open(&mut is_open)
        .default_size([820.0, 520.0])
        .show(ctx, |ui| {
            self.render_hft_panel(ui);
        });
    self.is_hft_open = is_open;
}

if self.is_gis_open {
    let mut is_open = self.is_gis_open;
    egui::Window::new("🗺️ Quantum GIS & Spatial Indexer")
        .open(&mut is_open)
        .default_size([780.0, 500.0])
        .show(ctx, |ui| {
            self.render_gis_panel(ui);
        });
    self.is_gis_open = is_open;
}

if self.is_tsdb_open {
    let mut is_open = self.is_tsdb_open;
    egui::Window::new("📈 Quantum Time-Series Telemetry Engine")
        .open(&mut is_open)
        .default_size([750.0, 480.0])
        .show(ctx, |ui| {
            self.render_tsdb_panel(ui);
        });
    self.is_tsdb_open = is_open;
}

if self.is_linter_open {
    let mut is_open = self.is_linter_open;
    egui::Window::new("🔍 Quantum Code Linter & Static Analyzer")
        .open(&mut is_open)
        .default_size([760.0, 500.0])
        .show(ctx, |ui| {
            self.render_linter_panel(ui);
        });
    self.is_linter_open = is_open;
}

if self.is_pathfinder_open {
    let mut is_open = self.is_pathfinder_open;
    egui::Window::new("🗺 Graph & Pathfinding Solver (A* / Dijkstra)")
        .open(&mut is_open)
        .default_size([720.0, 480.0])
        .show(ctx, |ui| {
            self.render_pathfinder_panel(ui);
        });
    self.is_pathfinder_open = is_open;
}

if self.is_archiver_open {
    let mut is_open = self.is_archiver_open;
    egui::Window::new("📦 Quantum Archiver & Entropy Engine")
        .open(&mut is_open)
        .default_size([720.0, 460.0])
        .show(ctx, |ui| {
            self.render_archiver_panel(ui);
        });
    self.is_archiver_open = is_open;
}

if self.is_input_open {
    let mut is_open = self.is_input_open;
    egui::Window::new("🎮 Virtual Input Controller & HID Driver")
        .open(&mut is_open)
        .default_size([720.0, 480.0])
        .show(ctx, |ui| {
            self.render_input_panel(ui);
        });
    self.is_input_open = is_open;
}

if self.is_gop_open {
    let mut is_open = self.is_gop_open;
    egui::Window::new("🎨 GOP / VGA Graphics Display Driver")
        .open(&mut is_open)
        .default_size([680.0, 460.0])
        .show(ctx, |ui| {
            self.render_gop_panel(ui);
        });
    self.is_gop_open = is_open;
    }

if self.is_proto_drivers_open {
    let mut is_open = self.is_proto_drivers_open;
    egui::Window::new("🔌 USB & NVMe Hardware Protocol Drivers")
        .open(&mut is_open)
        .default_size([700.0, 460.0])
        .show(ctx, |ui| {
            self.render_proto_drivers_panel(ui);
        });
    self.is_proto_drivers_open = is_open;
}

if self.is_fat32_open {
    let mut is_open = self.is_fat32_open;
    egui::Window::new("💾 FAT32 / VFS Parser (EFI System Partition)")
        .open(&mut is_open)
        .default_size([680.0, 440.0])
        .show(ctx, |ui| {
            self.render_fat32_panel(ui);
        });
    self.is_fat32_open = is_open;
}

if self.is_telemetry_open {
    let mut is_open = self.is_telemetry_open;
    egui::Window::new("📊 Telemetry, Profiler & Kernel Ring Buffer (dmesg)")
        .open(&mut is_open)
        .default_size([900.0, 580.0])
        .show(ctx, |ui| {
            self.render_telemetry_panel(ui);
        });
    self.is_telemetry_open = is_open;
}

if self.is_shell_open {
    let mut is_open = self.is_shell_open;
    egui::Window::new("🐚 Advanced Shell Pipeline & Automation Studio")
        .open(&mut is_open)
        .default_size([880.0, 560.0])
        .show(ctx, |ui| {
            self.render_shell_panel(ui);
        });
    self.is_shell_open = is_open;
}

if self.is_pkg_open {
    let mut is_open = self.is_pkg_open;
    egui::Window::new("📦 Quantum Package Manager & Dynamic Loader (.qmod / ELF)")
        .open(&mut is_open)
        .default_size([860.0, 540.0])
        .show(ctx, |ui| {
            self.render_pkg_panel(ui);
        });
    self.is_pkg_open = is_open;
}

if self.is_ipc_open {
    let mut is_open = self.is_ipc_open;
    egui::Window::new("🔄 Quantum Inter-Process Communication & Signals")
        .open(&mut is_open)
        .default_size([820.0, 520.0])
        .show(ctx, |ui| {
            self.render_ipc_panel(ui);
        });
    self.is_ipc_open = is_open;
}

    }

}

impl QuantumOSWinApp {
    fn render_register_grid(&mut self, ui: &mut egui::Ui) {
        let current_state = self.driver.read_register();

        ui.group(|ui| {
            ui.label("Atomski Registar (u64):");
            ui.monospace(format!("HEX: 0x{:016X}", current_state));
            ui.monospace(format!("BIN: {:064b}", current_state));
        });

        ui.add_space(10.0);
        ui.label("Matrica 32 Kvata (Pulsiranje u realnom vremenu):");

        egui::Grid::new("ququat_grid_win").spacing([8.0, 8.0]).show(ui, |ui| {
            for i in 0..32 {
                let val = self.driver.get_ququat(i);
                let color = match val {
                    QuquatVal::Q00 => egui::Color32::GRAY,
                    QuquatVal::Q01 => egui::Color32::LIGHT_BLUE,
                    QuquatVal::Q10 => egui::Color32::LIGHT_GREEN,
                    QuquatVal::Q11 => egui::Color32::GOLD,
                };

                ui.vertical(|ui| {
                    ui.label(format!("Q{:02}", i));
                    let btn = egui::Button::new(
                        egui::RichText::new(val.label()).color(color).strong()
                    ).min_size(egui::vec2(45.0, 32.0));

                    if ui.add(btn).clicked() {
                        self.driver.set_ququat(i, val.next());
                    }
                });

                if (i + 1) % 8 == 0 {
                    ui.end_row();
                }
            }
        });
    }
   
     fn render_vfs_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("💾 Virtuelni Kernel Fajl Sistem & Sektori Diska");
        ui.separator();

        // Putanja u Inode stablu
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Putanja:").strong());
            let mut curr = Some(self.vfs.current_dir);
            let mut path_parts = Vec::new();
            while let Some(id) = curr {
                path_parts.push((id, self.vfs.nodes[id].name.clone()));
                curr = self.vfs.nodes[id].parent;
            }
            path_parts.reverse();

            for (id, name) in path_parts {
                if ui.button(&name).clicked() {
                    self.vfs.current_dir = id;
                }
                ui.label("/");
            }
        });

        ui.add_space(5.0);

        // Kreiranje novih Inodova i Pisanje na Disk
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Ime unosa:");
                ui.text_edit_singleline(&mut self.new_item_name);

                if ui.button("📄 Napravi Fajl").clicked() && !self.new_item_name.is_empty() {
                    match self.vfs.create_file(self.vfs.current_dir, &self.new_item_name.clone()) {
                        Ok(id) => {
                            // Odmah upišemo testne kvate u fajl preko VFS blokova!
                            let dummy = vec![QuquatVal::Q01, QuquatVal::Q10, QuquatVal::Q11, QuquatVal::Q01];
                            let _ = self.vfs.write_bytes_to_inode(id, 0, &dummy);
                            self.syscall_logs.push(format!("[VFS] Kreiran Inode #{} ({})", id, self.new_item_name));
                        }
                        Err(e) => self.syscall_logs.push(format!("[ERROR] {}", e)),
                    }
                }

                if ui.button("📁 Napravi Dir").clicked() && !self.new_item_name.is_empty() {
                    if let Ok(id) = self.vfs.mkdir(self.vfs.current_dir, &self.new_item_name.clone()) {
                        self.syscall_logs.push(format!("[VFS] Kreiran Direktorijum Inode #{}", id));
                    }
                }
            });
        });

        ui.add_space(8.0);

        // STATISTIKA DISKA (Bitmap)
        let used_blocks = self.vfs.disk.free_bitmap.iter().filter(|&&is_free| !is_free).count();
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("DISK STATS:").strong());
            ui.label(format!("Zauzeto Sektora/Blokova: {} / {}", used_blocks, TOTAL_DISK_BLOCKS));
            let pct = used_blocks as f32 / TOTAL_DISK_BLOCKS as f32;
            ui.add(egui::ProgressBar::new(pct).text(format!("{:.1}%", pct * 100.0)));
        });

        ui.add_space(8.0);

        // TABELA INODOVA U TRENUTNOM DIREKTORIJUMU
        let children = self.vfs.nodes[self.vfs.current_dir].children.clone();

        egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
            egui::Grid::new("vfs_inode_grid")
                .striped(true)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Inode").strong());
                    ui.label(egui::RichText::new("Ime").strong());
                    ui.label(egui::RichText::new("Tip").strong());
                    ui.label(egui::RichText::new("Veličina").strong());
                    ui.label(egui::RichText::new("Disk Pokazivači (Blokovi)").strong());
                    ui.end_row();

                    for child_id in children {
                        let node = &self.vfs.nodes[child_id];
                        ui.label(format!("#{:02}", node.id));

                        match node.node_type {
                            NodeType::Directory => {
                                if ui.button(format!("📁 {}", node.name)).clicked() {
                                    self.vfs.current_dir = node.id;
                                }
                                ui.label("Dir");
                                ui.label("-");
                                ui.label("-");
                            }
                            NodeType::File => {
                                let is_selected = self.selected_inode == Some(node.id);
                                if ui.selectable_label(is_selected, format!("📄 {}", node.name)).clicked() {
                                    self.selected_inode = Some(node.id);
                                }
                                ui.label("File");
                                ui.label(format!("{} Q", node.size_in_ququats));
                                ui.label(format!("{:?}", node.block_pointers));
                            }
                        }
                        ui.end_row();
                    }
                });
        });

        ui.add_space(10.0);

        // PREGLED IZABRANOG INODE-A & SIROVI SEKTORI DISKA
        if let Some(inode_id) = self.selected_inode {
            if let Some(node) = self.vfs.nodes.get(inode_id) {
                ui.group(|ui| {
                    ui.label(egui::RichText::new(format!("🔎 Inode Inspection #{}: {}", node.id, node.name)).strong());
                    ui.separator();
                    
                    if let Ok(data) = self.vfs.read_bytes_from_inode(node.id, 0, node.size_in_ququats) {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Sadržaj na Disku: ");
                            for (idx, q) in data.iter().enumerate() {
                                ui.label(format!("[{}: {}]", idx, q.label()));
                            }
                        });
                    }
                });
            }
        }

        ui.add_space(5.0);

        // KERNEL SYSCALL LOGOVI NA DNU
        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 Kernel Syscall Logs").strong());
            egui::ScrollArea::vertical().max_height(80.0).show(ui, |ui| {
                for log in self.syscall_logs.iter().rev() {
                    ui.monospace(log);
                }
            });
        });
    }

    fn render_audio_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎛 Kvantni Zvučni Sintisajzer (4^32 Hz)");
        ui.separator();

        // UKLJUČIVANJE / ISKLJUČIVANJE AUDIJA
        ui.horizontal(|ui| {
            let btn_text = if self.audio.is_enabled { "🔊 Audio UKLJUČEN" } else { "🔇 Audio ISKLJUČEN" };
            if ui.button(btn_text).clicked() {
                self.audio.is_enabled = !self.audio.is_enabled;
            }
            ui.label(format!("Oblik talasa: {:?}", self.audio.current_params.waveform));
        });

        ui.add_space(8.0);

        // OČITAVANJA DSP PARAMETARA IZ REGISTRA
        ui.group(|ui| {
            ui.label(egui::RichText::new("Telemetrija u realnom vremenu:").strong());
            ui.label(format!("Osnovna Frekvencija: {:.2} Hz", self.audio.current_params.frequency));
            ui.label(format!("Glasnoća (Volume): {:.0}%", self.audio.current_params.volume * 100.0));
            ui.label(format!("Filter Cutoff: {:.2} Hz", self.audio.current_params.cutoff_freq));
        });

        ui.add_space(10.0);

        // OSCOSLOSKOP (VIZUELIZACIJA ZVUČNOG TALASA)
        ui.label(egui::RichText::new("📊 Osciloskop Zvučnog Talasa:").strong());
        
        let samples = self.audio.render_audio_frame(&self.driver, 128);
        let points: Vec<egui::Pos2> = samples
            .iter()
            .enumerate()
            .map(|(i, &sample)| {
                let x = i as f32 * 3.0;
                let y = 50.0 - (sample * 40.0); // Skaliranje na visinu
                egui::pos2(x, y)
            })
            .collect();

        let (response, painter) = ui.allocate_painter(egui::vec2(380.0, 100.0), egui::Sense::hover());
        painter.rect_filled(response.rect, 4.0, egui::Color32::BLACK);

        if points.len() > 1 {
            let offset = response.rect.min;
            for i in 0..points.len() - 1 {
                let p1 = egui::pos2(points[i].x + offset.x, points[i].y + offset.y);
                let p2 = egui::pos2(points[i + 1].x + offset.x, points[i + 1].y + offset.y);
                painter.line_segment([p1, p2], egui::Stroke::new(2.0, egui::Color32::GREEN));
            }
        }
    }
    fn render_vm_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚡ CPU Registri & JIT Optimizator");
        ui.separator();

        // TELEMETRIJA VM CPU-a
        ui.group(|ui| {
            ui.label(egui::RichText::new("CPU Status:").strong());
            ui.horizontal(|ui| {
                ui.label(format!("Program Counter (PC): {}", self.vm.cpu.pc));
                ui.separator();
                ui.label(format!("Ukupno CPU Taktova: {}", self.vm.cpu.total_cycles));
                ui.separator();
                let status = if self.vm.cpu.is_halted { "🛑 Halted" } else { "▶️ Running" };
                ui.label(format!("Status: {}", status));
            });
        });

        ui.add_space(8.0);

        // KONTROLE ZA UČITAVANJE PROGRAMA I TESTIRANJE JIT-a
        ui.horizontal(|ui| {
            if ui.button("🚀 Učitaj Test Program (Spora petlja)").clicked() {
                // Generišemo niz od 5 SET instrukcija da potpalimo JIT!
                let program = vec![
                    crate::domain::QuquatVal::Q01, crate::domain::QuquatVal::Q01, // SET Q0 = Q01
                    crate::domain::QuquatVal::Q01, crate::domain::QuquatVal::Q01, // SET Q1 = Q01
                    crate::domain::QuquatVal::Q01, crate::domain::QuquatVal::Q01, // SET Q2 = Q01
                    crate::domain::QuquatVal::Q01, crate::domain::QuquatVal::Q01, // SET Q3 = Q01
                ];
                self.vm.load_program(program);
            }

            if ui.button("🔥 Pokreni JIT Kompajler").clicked() {
                let insts = vec![
                    Instruction { opcode: Opcode::SetQuquat, arg1: 0, arg2: 1 },
                    Instruction { opcode: Opcode::SetQuquat, arg1: 1, arg2: 2 },
                    Instruction { opcode: Opcode::SetQuquat, arg1: 2, arg2: 3 },
                    Instruction { opcode: Opcode::SetQuquat, arg1: 3, arg2: 1 },
                ];
                self.vm.cpu.jit_compile(&insts);
            }

            if ui.button("🔄 Reset CPU").clicked() {
                self.vm.cpu.reset();
            }
        });

        ui.add_space(10.0);

        // PRIKAZ JIT KEŠA (Sastavljene bitmaske)
        ui.group(|ui| {
            ui.label(egui::RichText::new("🔥 JIT Compiled Blocks Cache").strong());
            if self.vm.cpu.jit_cache.is_empty() {
                ui.label("JIT keš je prazan. Klikni na 'Pokreni JIT Kompajler' da spojiš instrukcije u 1 u64 takt.");
            } else {
                for (idx, block) in self.vm.cpu.jit_cache.iter().enumerate() {
                    ui.monospace(format!(
                        "Block #{}: Start PC={}, Len={}, AND=0x{:016X}, OR=0x{:016X}",
                        idx, block.start_pc, block.length, block.and_mask, block.or_mask
                    ));
                }
            }
        });
    }

    fn render_net_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🌐 Virtuelni Mrežni Stack (QNet Protocol)");
        ui.separator();

        // STATISTIKA
        ui.horizontal(|ui| {
            ui.label(format!("Poslato Paketa: {}", self.net.packets_sent));
            ui.separator();
            ui.label(format!("Primljeno Paketa: {}", self.net.packets_received));
            ui.separator();
            ui.label(format!("Aktivnih Soketa: {}", self.net.sockets.len()));
        });

        ui.add_space(8.0);

        // KONTROLE ZA TESTIRANJE MREŽE
        ui.group(|ui| {
            ui.label(egui::RichText::new("Mrežne Akcije:").strong());
            ui.horizontal(|ui| {
                if ui.button("🔌 Otvori Soket 8080 (PID 1)").clicked() {
                    self.net.open_socket(8080, 1);
                }

                if ui.button("📡 Pošalji QuantumSync Paket").clicked() {
                    let pkt = QuquatPacket::new(
                        1, 2, 8080,
                        PacketType::QuantumSync,
                        vec![crate::domain::QuquatVal::Q11, crate::domain::QuquatVal::Q10]
                    );
                    self.net.transmit(pkt);
                }

                if ui.button("⚠️ Pošalji Pokvaren Paket").clicked() {
                    let mut pkt = QuquatPacket::new(
                        1, 2, 8080,
                        PacketType::Data,
                        vec![crate::domain::QuquatVal::Q01]
                    );
                    pkt.checksum = 0xFFFF; // Namerno lažiramo checksum!
                    self.net.transmit(pkt);
                }
            });
        });

        ui.add_space(10.0);

        // PRIKAZ SOKETA
        ui.group(|ui| {
            ui.label(egui::RichText::new("📋 Tabela Otvorenih Soketa").strong());
            if self.net.sockets.is_empty() {
                ui.label("Nema otvorenih soketa. Klikni 'Otvori Soket 8080' gore.");
            } else {
                for sock in &self.net.sockets {
                    ui.label(format!(
                        "Port: {} | Vlasnik PID: {} | Status: {:?} | RX Bafer: {} paketa",
                        sock.port, sock.owner_pid, sock.state, sock.rx_buffer.len()
                    ));
                }
            }
        });

        ui.add_space(8.0);

        // LOGOVI SAOBRAĆAJA
        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 Network Packet Logs").strong());
            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                for log in self.net.network_log.iter().rev() {
                    ui.monospace(log);
                }
            });
        });
    }

    fn render_db_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗄 QStore In-Memory Database (ACID & VFS Persisted)");
        ui.separator();

        // STATISTIKA BAZE
        ui.horizontal(|ui| {
            ui.label(format!("Ukupno Ključeva: {}", self.db.storage.total_keys));
            ui.separator();
            let tx_status = if self.db.active_tx.is_some() { "🟡 Transakcija u toku" } else { "🟢 Idle" };
            ui.label(format!("Status: {}", tx_status));
        });

        ui.add_space(8.0);

        // UNOS KLJUČEVA I TRANSAKCIJE
        ui.group(|ui| {
            ui.label(egui::RichText::new("Baza i Transakcione Operacije:").strong());
            ui.horizontal(|ui| {
                ui.label("Ključ:");
                ui.text_edit_singleline(&mut self.db_key_input);

                if ui.button("➕ Upisi Ključ").clicked() && !self.db_key_input.is_empty() {
                    let payload = vec![crate::domain::QuquatVal::Q01, crate::domain::QuquatVal::Q11];
                    self.db.set(&self.db_key_input.clone(), payload, self.scheduler.total_ticks);
                }
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button("▶️ Begin Tx").clicked() {
                    self.db.begin_transaction();
                }
                if ui.button("✅ Commit Tx").clicked() {
                    self.db.commit_transaction(self.scheduler.total_ticks);
                }
                if ui.button("❌ Rollback Tx").clicked() {
                    self.db.rollback_transaction();
                }
                ui.separator();
                if ui.button("💾 Snapshot u VFS (/sys/qstore.db)").clicked() {
                    self.db.snapshot(&mut self.vfs);
                }
            });
        });

        ui.add_space(10.0);

        // TABELA KLJUČEVA
        ui.group(|ui| {
            ui.label(egui::RichText::new("📋 In-Memory Indeks Baze").strong());
            egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                egui::Grid::new("db_keys_grid").striped(true).spacing([12.0, 6.0]).show(ui, |ui| {
                    ui.label(egui::RichText::new("Ključ").strong());
                    ui.label(egui::RichText::new("Verzija").strong());
                    ui.label(egui::RichText::new("Kreirano (Tick)").strong());
                    ui.label(egui::RichText::new("Vrednost (Kvati)").strong());
                    ui.end_row();

                    for (key, rec) in &self.db.storage.index {
                        ui.label(key);
                        ui.label(format!("v{}", rec.version));
                        ui.label(format!("#{}", rec.created_at_tick));
                        ui.label(format!("{:?}", rec.data));
                        ui.end_row();
                    }
                });
            });
        });

        ui.add_space(8.0);

        // DB LOGOVI
        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 Database Audit Logs").strong());
            egui::ScrollArea::vertical().max_height(90.0).show(ui, |ui| {
                for log in self.db.logs.iter().rev() {
                    ui.monospace(log);
                }
            });
        });
    }

     fn render_terminal_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("💻 Ququat Kernel Shell Console");
        ui.separator();

        // EKRA SKROLOVANJA TERMINALA
        let mut execute_requested = false;

        egui::Frame::canvas(ui.style())
            .fill(egui::Color32::from_rgb(15, 15, 20))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(280.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        for line in &self.terminal.output_lines {
                            let color = match line.line_type {
                                LineType::Prompt => egui::Color32::LIGHT_BLUE,
                                LineType::Info => egui::Color32::LIGHT_GRAY,
                                LineType::Success => egui::Color32::GREEN,
                                LineType::Error => egui::Color32::LIGHT_RED,
                                LineType::Warning => egui::Color32::GOLD,
                                LineType::Header => egui::Color32::YELLOW,
                            };
                            ui.monospace(egui::RichText::new(&line.text).color(color));
                        }
                    });
            });

        ui.add_space(8.0);

        // POLJE ZA UNOS KOMANDI (ENTER POTVRĐUJE)
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("ququat@kernel:~$").strong().color(egui::Color32::GREEN));

            let response = ui.add(
                egui::TextEdit::singleline(&mut self.terminal.input_buffer)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace)
            );

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                execute_requested = true;
            }

            if ui.button("Pošalji").clicked() {
                execute_requested = true;
            }
        });

        // IZVRŠAVANJE UNESENE KOMANDE
        if execute_requested {
            let input = self.terminal.input_buffer.clone();
            self.terminal.input_buffer.clear();

            CommandDispatcher::dispatch(
                &input,
                &mut self.terminal,
                &self.driver,
                &mut self.vfs,
                &mut self.scheduler,
                &mut self.db,
                &mut self.net,
                &mut self.container_engine,
                &mut self.video_engine,
                &mut self.img_engine,
                &mut self.compiler,
                &mut self.search_engine,
                &mut self.crypto_engine,
                &mut self.nvram_engine,
                &mut self.boot_engine,
                &mut self.stream_engine,
                &mut self.hft_engine,
                &mut self.gis_engine,
                &mut self.tsdb_engine,
                &mut self.linter_engine,
                &mut self.pathfinder_engine,
                &mut self.archiver_engine,
                &mut self.input_engine,
                &mut self.gop_engine,
                &mut self.proto_drivers,
                &mut self.fat32_engine,
                &mut self.ipc_engine,
                &mut self.pkg_engine,
                &mut self.shell_engine,
                &mut self.telemetry_engine,
            );
            
        }
    }
    fn render_hypervisor_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🐳 QDocker Container & VM Hypervisor");
        ui.separator();

        // FORMA ZA KREIRANJE NOVOG KONTEJNERA
        ui.group(|ui| {
            ui.label(egui::RichText::new("Novi Kontejner").strong());
            ui.horizontal(|ui| {
                ui.label("Naziv:");
                ui.text_edit_singleline(&mut self.new_cnt_name);

                ui.label("Imidž:");
                egui::ComboBox::from_id_source("img_combo")
                    .selected_text(&self.selected_image)
                    .show_ui(ui, |ui| {
                        for img in &self.container_engine.available_images.clone() {
                            ui.selectable_value(&mut self.selected_image, img.clone(), img);
                        }
                    });

                ui.label("RAM (MB):");
                ui.add(egui::DragValue::new(&mut self.new_cnt_ram).clamp_range(16..=1024));

                if ui.button("➕ Kreiraj").clicked() {
                    self.container_engine.create_container(
                        &self.new_cnt_name,
                        &self.selected_image,
                        self.new_cnt_ram,
                        2,
                    );
                }
            });
        });

        ui.add_space(10.0);

        // TABELA AKTIVNIH KONTEJNERA
        ui.label(egui::RichText::new("Aktivne Izolovane Instatnce").strong());
        egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
            egui::Grid::new("containers_grid").striped(true).min_col_width(80.0).show(ui, |ui| {
                ui.heading("ID");
                ui.heading("Naziv");
                ui.heading("Imidž");
                ui.heading("IP Adresa");
                ui.heading("RAM");
                ui.heading("Status");
                ui.heading("Akcije");
                ui.end_row();

                let mut action_start = None;
                let mut action_stop = None;
                let mut action_pause = None;
                let mut action_remove = None;

                for (id, c) in &self.container_engine.containers {
                    ui.label(id);
                    ui.label(&c.name);
                    ui.label(&c.image);
                    ui.label(&c.ip_address);
                    ui.label(format!("{} MB", c.allocated_memory_mb));

                    let status_color = match c.status {
                        ContainerStatus::Running => egui::Color32::GREEN,
                        ContainerStatus::Paused => egui::Color32::GOLD,
                        ContainerStatus::Stopped => egui::Color32::RED,
                        ContainerStatus::Created => egui::Color32::LIGHT_BLUE,
                    };
                    ui.colored_label(status_color, format!("{:?}", c.status));

                    ui.horizontal(|ui| {
                        if c.status != ContainerStatus::Running && ui.button("▶").clicked() {
                            action_start = Some(id.clone());
                        }
                        if c.status == ContainerStatus::Running && ui.button("⏸").clicked() {
                            action_pause = Some(id.clone());
                        }
                        if c.status != ContainerStatus::Stopped && ui.button("⏹").clicked() {
                            action_stop = Some(id.clone());
                        }
                        if ui.button("🗑").clicked() {
                            action_remove = Some(id.clone());
                        }
                    });
                    ui.end_row();
                }

                if let Some(id) = action_start { self.container_engine.start_container(&id); }
                if let Some(id) = action_pause { self.container_engine.pause_container(&id); }
                if let Some(id) = action_stop { self.container_engine.stop_container(&id); }
                if let Some(id) = action_remove { self.container_engine.remove_container(&id); }
            });
        });
    }
    fn render_video_panel(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🎬 Quantum Video Decoder & Player");
        ui.separator();

        // Izbor videa
        ui.horizontal(|ui| {
            ui.label("Izvori strima:");
            let current = self.video_engine.current_video.clone();
            egui::ComboBox::from_id_source("video_select")
                .selected_text(&current)
                .show_ui(ui, |ui| {
                    for vid in self.video_engine.available_videos.clone() {
                        if ui.selectable_label(vid == current, &vid).clicked() {
                            self.video_engine.load_video(&vid);
                        }
                    }
                });
        });

        ui.add_space(5.0);

        // Dekodiranje frejma i osvežavanje teksture
        let frame = self.video_engine.decode_next_frame();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [frame.width, frame.height],
            &frame.rgba,
        );

        let texture = ctx.load_texture("video_stream_frame", color_image, egui::TextureOptions::LINEAR);
        ui.image(&texture);

        // Kontrole reprodukcije
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            if ui.button("▶ Play").clicked() {
                self.video_engine.play();
            }
            if ui.button("⏸ Pause").clicked() {
                self.video_engine.pause();
            }
            if ui.button("⏹ Stop").clicked() {
                self.video_engine.stop();
            }

            ui.separator();
            ui.label(format!("Frejm: {} / {}", self.video_engine.current_frame, self.video_engine.total_frames));
        });

        // Ako se video reprodukuje, tražimo osvežavanje GUI-ja
        if self.video_engine.state == PlaybackState::Playing {
            ctx.request_repaint();
        }
    }
    fn render_image_panel(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🖼 Quantum Studio za Obradu Slika");
        ui.separator();

        ui.horizontal(|ui| {
            // UNDO / REDO / GENERISANJE
            if ui.button("↶ Undo").clicked() { self.img_engine.undo(); }
            if ui.button("↷ Redo").clicked() { self.img_engine.redo(); }
            ui.separator();
            if ui.button("🌀 Mandelbrot").clicked() { self.img_engine.load_mandelbrot(); }
            if ui.button("🏁 Šahovnica").clicked() { self.img_engine.load_checkerboard(); }
            if ui.button("🌈 Gradijent").clicked() { self.img_engine.load_gradient(); }
        });

        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Kontrole i Filteri
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Filteri & Efekti").strong());
                ui.horizontal(|ui| {
                    if ui.button("Grayscale").clicked() { self.img_engine.apply_grayscale(); }
                    if ui.button("Invert").clicked() { self.img_engine.apply_invert(); }
                    if ui.button("Sepia").clicked() { self.img_engine.apply_sepia(); }
                });

                ui.horizontal(|ui| {
                    if ui.button("Box Blur").clicked() { self.img_engine.apply_blur(); }
                    if ui.button("Sobel Ivice").clicked() { self.img_engine.apply_sobel(); }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Transformacije").strong());
                ui.horizontal(|ui| {
                    if ui.button("🔄 Rotiraj 90°").clicked() { self.img_engine.rotate_90(); }
                    if ui.button("↔ Flip H").clicked() { self.img_engine.flip_h(); }
                    if ui.button("↕ Flip V").clicked() { self.img_engine.flip_v(); }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Korekcija Boja").strong());
                ui.add(egui::Slider::new(&mut self.img_engine.brightness, -100..=100).text("Osvetljenje"));
                ui.add(egui::Slider::new(&mut self.img_engine.contrast, 0.2..=3.0).text("Kontrast"));

                if ui.button("Premeni Osvetljenje/Kontrast").clicked() {
                    self.img_engine.apply_color_adjustments();
                }
            });

            // DESNA KOLONA: Pregled Slike (Canvas)
            cols[1].vertical(|ui| {
                let img_data = &self.img_engine.current_image;
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [img_data.width, img_data.height],
                    &img_data.pixels,
                );
                let texture = ctx.load_texture("img_studio_canvas", color_image, egui::TextureOptions::NEAREST);

                ui.label(format!("Dimenzije: {} x {} px", img_data.width, img_data.height));
                ui.image(&texture);
            });
        });
    }
    fn render_compiler_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙ QuantumScript Lexer / Parser / AST Compiler");
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("▶ Kompajliraj & Pokreni").clicked() {
                let _ = self.compiler.compile_and_run();
            }
        });

        ui.add_space(5.0);

        ui.columns(2, |cols| {
            // LEVA KOLONA: Editor Koda
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Izvorni Kôd (QScript)").strong());
                ui.add(
                    egui::TextEdit::multiline(&mut self.compiler.source_code)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(14)
                        .desired_width(f32::INFINITY),
                );

                ui.add_space(5.0);
                ui.label(egui::RichText::new("Konzolni Izlaz (Evaluator Output)").strong());
                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    for line in &self.compiler.evaluator.output {
                        ui.colored_label(egui::Color32::GREEN, line);
                    }
                });
            });

            // DESNA KOLONA: AST Stablo i Tokeni
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("Generisano AST Stablo (Parser)").strong());
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    if let Some(ast) = &self.compiler.last_ast {
                        ui.monospace(format!("{:#?}", ast));
                    } else {
                        ui.label("Kliknite na 'Kompajliraj' za generisanje AST-a.");
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Leksički Tokeni (Lexer)").strong());
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    ui.monospace(format!("{:?}", self.compiler.last_tokens));
                });
            });
        });
    }
    fn render_search_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔍 In-Memory Elasticsearch Pretraživač");
        ui.separator();

        // Polje za pretragu
        ui.horizontal(|ui| {
            ui.label("Upit:");
            ui.text_edit_singleline(&mut self.search_query_input);
        });

        ui.add_space(5.0);

        let results = self.search_engine.search(&self.search_query_input);

        ui.columns(2, |cols| {
            // LEVA KOLONA: Rezultati pretrage
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new(format!("Rezultati Pretrage ({})", results.len())).strong());
                ui.separator();

                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    if results.is_empty() {
                        ui.label("Unesite upit za pretragu ili indeksirajte dokumente.");
                    } else {
                        for res in results {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(&res.title).strong());
                                    ui.colored_label(egui::Color32::YELLOW, format!("BM25: {:.2}", res.score));
                                });
                                ui.label(&res.content_snippet);
                                ui.colored_label(
                                    egui::Color32::LIGHT_BLUE,
                                    format!("Match: {:?}", res.matched_terms),
                                );
                            });
                            ui.add_space(3.0);
                        }
                    }
                });
            });

            // DESNA KOLONA: Dodavanje novog dokumenta
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("Indeksiranje Novog Dokumenta").strong());
                ui.separator();

                ui.label("Naslov:");
                ui.text_edit_singleline(&mut self.new_doc_title_input);

                ui.label("Sadržaj:");
                ui.add(
                    egui::TextEdit::multiline(&mut self.new_doc_content_input)
                        .desired_rows(4)
                        .desired_width(f32::INFINITY),
                );

                ui.add_space(5.0);
                if ui.button("➕ Indeksiraj Dokument").clicked() {
                    if !self.new_doc_title_input.is_empty() && !self.new_doc_content_input.is_empty() {
                        self.search_engine.index_doc(
                            &self.new_doc_title_input,
                            &self.new_doc_content_input,
                            vec!["custom".into()],
                        );
                        self.new_doc_title_input.clear();
                        self.new_doc_content_input.clear();
                    }
                }

                ui.add_space(10.0);
                ui.label(format!("Ukupno indeksirano dokumenata: {}", self.search_engine.documents.len()));
            });
        });
    }
    fn render_crypto_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔐 Crypto Studio & TLS 1.3 Protocol Simulator");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Simetrična & Asimetrična Enkripcija
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Simetrična Enkripcija").strong());
                ui.horizontal(|ui| {
                    ui.label("Tekst:");
                    ui.text_edit_singleline(&mut self.crypto_input_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Ključ:");
                    ui.text_edit_singleline(&mut self.crypto_key_input);
                });

                ui.horizontal(|ui| {
                    if ui.button("🔒 Enkriptuj").clicked() {
                        self.crypto_engine.encrypt_text(&self.crypto_input_text, &self.crypto_key_input);
                    }
                    if ui.button("🔓 Dešifruj").clicked() {
                        let bytes = self.crypto_engine.last_encrypted_bytes.clone();
                        self.crypto_engine.decrypt_bytes(&bytes, &self.crypto_key_input);
                    }
                });

                ui.monospace(format!("Šifrovani bajtovi: {:?}", self.crypto_engine.last_encrypted_bytes));
                ui.monospace(format!("Dešifrovani tekst: {}", self.crypto_engine.last_decrypted_text));

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Digitalni Potpis (RSA)").strong());
                ui.text_edit_singleline(&mut self.crypto_sign_text);

                ui.horizontal(|ui| {
                    if ui.button("✍️ Potpiši").clicked() {
                        self.crypto_signature = crate::crypto::asymmetric::sign_message(
                            &self.crypto_sign_text,
                            &self.crypto_engine.rsa_keys.private_key,
                        );
                    }
                });
                ui.monospace(format!("Potpis: {}", self.crypto_signature));

                if ui.button("✔️ Verifikuj Potpis").clicked() {
                    let valid = crate::crypto::asymmetric::verify_signature(
                        &self.crypto_sign_text,
                        &self.crypto_signature,
                        &self.crypto_engine.rsa_keys.public_key,
                    );
                    if valid {
                        self.crypto_engine.last_hash = "POTPIS JE VAŽEĆI!".to_string();
                    } else {
                        self.crypto_engine.last_hash = "NEVAŽEĆI POTPIS!".to_string();
                    }
                }
            });

            // DESNA KOLONA: TLS 1.3 Simulacija
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("TLS 1.3 Handshake Simulator").strong());
                ui.separator();

                if ui.button("⚡ Pokreni TLS 1.3 Handshake").clicked() {
                    self.crypto_engine.tls_session.execute_handshake();
                }

                ui.add_space(5.0);
                ui.label(format!("Status: {:?}", self.crypto_engine.tls_session.state));

                egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                    for log in &self.crypto_engine.tls_session.handshake_logs {
                        ui.colored_label(egui::Color32::GREEN, log);
                    }
                });
            });
        });
    }
    fn render_nvram_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙ UEFI / BIOS Non-Volatile RAM (NVRAM)");
        ui.separator();

        // Status iskorišćenosti memorije
        let used = self.nvram_engine.storage.current_used_bytes;
        let max = self.nvram_engine.storage.max_capacity_bytes;
        ui.add(egui::ProgressBar::new(used as f32 / max as f32).text(format!("Iskorišćeno: {} / {} Bajtova", used, max)));

        ui.add_space(5.0);

        ui.columns(2, |cols| {
            // LEVA KOLONA: Pregled varijabli
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Iskoristive NVRAM Promenljive").strong());
                ui.separator();

                egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                    for (name, var) in &self.nvram_engine.storage.memory {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(name).strong());
                                if var.is_read_only() {
                                    ui.colored_label(egui::Color32::LIGHT_RED, "[Read-Only]");
                                } else {
                                    ui.colored_label(egui::Color32::GREEN, "[Read-Write]");
                                }
                            });
                            ui.monospace(format!("Vrednost: {}", var.get_string_val()));
                        });
                        ui.add_space(3.0);
                    }
                });
            });

            // DESNA KOLONA: Upis i upravljanje
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("Izmena NVRAM Podešavanja").strong());
                ui.separator();

                ui.label("Naziv varijable:");
                ui.text_edit_singleline(&mut self.nvram_key_input);

                ui.label("Vrednost:");
                ui.text_edit_singleline(&mut self.nvram_val_input);

                ui.add_space(5.0);
                if ui.button("💾 Sačuvaj u NVRAM").clicked() {
                    if !self.nvram_key_input.is_empty() {
                        let _ = self.nvram_engine.set_var(&self.nvram_key_input, &self.nvram_val_input);
                        self.nvram_key_input.clear();
                        self.nvram_val_input.clear();
                    }
                }

                ui.add_space(15.0);
                if ui.button("🔄 Fabrički Reset NVRAM-a").clicked() {
                    self.nvram_engine.factory_reset();
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Sistemski Logovi").strong());
                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    for log in &self.nvram_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_boot_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🚀 Boot Sequence & Device Priority Manager");
        ui.separator();

        // Opcije UEFI / Fast Boot
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.boot_engine.uefi_mode, "UEFI Režim (Omogućeno)");
            ui.add_space(20.0);
            ui.checkbox(&mut self.boot_engine.fast_boot, "Fast Boot");
        });

        ui.add_space(10.0);

        ui.columns(2, |cols| {
            // LEVA KOLONA: Redosled uređaja i kontrolna dugmad
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Prioritet Uređaja za Podizanje").strong());
                ui.separator();

                let mut to_move_up = None;
                let mut to_move_down = None;

                egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                    for (idx, dev) in self.boot_engine.devices.iter_mut().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{}.", idx + 1)).strong());
                                ui.checkbox(&mut dev.enabled, "");
                                ui.label(egui::RichText::new(&dev.name).strong());
                            });

                            ui.horizontal(|ui| {
                                ui.small(format!("Tip: {:?}", dev.dev_type));
                                ui.add_space(10.0);
                                if ui.small_button("⬆ Gore").clicked() {
                                    to_move_up = Some(idx);
                                }
                                if ui.small_button("⬇ Dole").clicked() {
                                    to_move_down = Some(idx);
                                }
                            });
                        });
                        ui.add_space(2.0);
                    }
                });

                if let Some(idx) = to_move_up {
                    self.boot_engine.move_up(idx);
                }
                if let Some(idx) = to_move_down {
                    self.boot_engine.move_down(idx);
                }

                ui.add_space(10.0);
                if ui.button("⚡ Testiraj Podizanje Sistema").clicked() {
                    let _ = self.boot_engine.simulate_boot_sequence();
                }
            });

            // DESNA KOLONA: Status i Logovi Podizanja
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("Status Boot Sekvence").strong());
                ui.separator();

                ui.colored_label(
                    if self.boot_engine.last_boot_status.contains("GREŠKA") {
                        egui::Color32::LIGHT_RED
                    } else {
                        egui::Color32::GREEN
                    },
                    &self.boot_engine.last_boot_status,
                );

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Boot Dnevnik (Logs)").strong());
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for log in &self.boot_engine.boot_logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_stream_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("📡 Distributed Commit Log & Event Streaming Engine");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Producer Kontrola i Teme
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("⚡ Producer: Objavljivanje Događaja").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Topic:");
                        ui.text_edit_singleline(&mut self.stream_pub_topic);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Key:");
                        ui.text_edit_singleline(&mut self.stream_pub_key);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Value:");
                        ui.text_edit_singleline(&mut self.stream_pub_val);
                    });

                    if ui.button("🚀 Objavi Događaj (Publish)").clicked() {
                        let topic = self.stream_pub_topic.clone();
                        let key = self.stream_pub_key.clone();
                        let val = self.stream_pub_val.clone();
                        self.stream_engine.publish_event(&topic, &key, &val);
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("📂 Pregled Particija i Logova").strong());
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for (name, topic) in &self.stream_engine.topics {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(format!("Topic: {}", name)).strong());
                            for partition in &topic.partitions {
                                ui.horizontal(|ui| {
                                    ui.label(format!(" Particija #{}:", partition.id));
                                    ui.colored_label(egui::Color32::LIGHT_BLUE, format!("{} poruka", partition.log.len()));
                                    ui.small(format!("(Offset: {})", partition.next_offset));
                                });
                            }
                        });
                    }
                });
            });

            // DESNA KOLONA: Consumer Groups i Log Događaja
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("📥 Consumer Group: Čitanje Strimova").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("▶ Čitaj sa 'system.events' (Particija 0)").clicked() {
                            self.stream_engine.consume_events("gui_group", "system.events", 0, 5);
                        }
                    });
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("📜 Dnevnik Striming Modula (Live Stream)").strong());
                egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
                    for log in self.stream_engine.logs.iter().rev() {
                        ui.group(|ui| {
                            ui.small(log);
                        });
                    }
                });
            });
        });
    }
    fn render_hft_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading(format!("⚡ HFT Exchange Engine [{}]", self.hft_engine.book.symbol));
        ui.separator();

        // Taktuj bota pri svakom osvežavanju za akciju u realnom vremenu
        self.hft_engine.tick_bot();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Order Book (Kupovina vs Prodaja)
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("📖 Limit Order Book (Depth)").strong());

                ui.group(|ui| {
                    ui.label(egui::RichText::new("🔴 ASKS (Prodajni Nalozi)").color(egui::Color32::LIGHT_RED).strong());
                    egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                        for ask in self.hft_engine.book.asks.iter().rev() {
                            ui.horizontal(|ui| {
                                ui.colored_label(egui::Color32::LIGHT_RED, format!("{:.2} USD", ask.price));
                                ui.label(format!("x {}", ask.quantity));
                            });
                        }
                    });

                    let spread = self.hft_engine.book.spread().unwrap_or(0.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("SPREAD:").strong());
                        ui.colored_label(egui::Color32::GOLD, format!("{:.2} USD", spread));
                    });
                    ui.separator();

                    ui.label(egui::RichText::new("🟢 BIDS (Kupovni Nalozi)").color(egui::Color32::LIGHT_GREEN).strong());
                    egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                        for bid in &self.hft_engine.book.bids {
                            ui.horizontal(|ui| {
                                ui.colored_label(egui::Color32::LIGHT_GREEN, format!("{:.2} USD", bid.price));
                                ui.label(format!("x {}", bid.quantity));
                            });
                        }
                    });
                });
            });

            // DESNA KOLONA: Ručno trgovanje, Latencija i Nedavne Transakcije
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("⚡ Brzo Plasiranje Naloga").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Cena:");
                        ui.add(egui::DragValue::new(&mut self.hft_trade_price).speed(0.1));
                        ui.label("Količina:");
                        ui.add(egui::DragValue::new(&mut self.hft_trade_qty));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("🟢 Kupi (BUY)").clicked() {
                            let p = self.hft_trade_price;
                            let q = self.hft_trade_qty;
                            self.hft_engine.submit_order(OrderSide::Buy, p, q);
                        }
                        if ui.button("🔴 Prodaj (SELL)").clicked() {
                            let p = self.hft_trade_price;
                            let q = self.hft_trade_qty;
                            self.hft_engine.submit_order(OrderSide::Sell, p, q);
                        }
                        ui.checkbox(&mut self.hft_engine.bot_enabled, "Bot Liquidity");
                    });
                });

                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("Brzina obrade:");
                    ui.colored_label(egui::Color32::LIGHT_BLUE, format!("{} ns", self.hft_engine.execution_latency_ns));
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("📜 Poslednje Trgovine (Matched Trades)").strong());
                egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                    for trade in self.hft_engine.book.trades.iter().rev().take(10) {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("Trade #{}", trade.trade_id));
                                ui.colored_label(egui::Color32::YELLOW, format!("{:.2} USD", trade.price));
                                ui.small(format!("x{}", trade.quantity));
                            });
                        });
                    }
                });
            });
        });
    }
    fn render_gis_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗺️ Quantum Spatial / GIS Indexing Engine");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Prostorni Upiti i Kontrole
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("📍 Radijus Pretraga (Radius Search)").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Lat:");
                        ui.add(egui::DragValue::new(&mut self.gis_search_lat).speed(0.01));
                        ui.label("Lon:");
                        ui.add(egui::DragValue::new(&mut self.gis_search_lon).speed(0.01));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Radijus (km):");
                        ui.add(egui::DragValue::new(&mut self.gis_search_radius).speed(5.0));
                    });
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Aktivne GIS Lokacije u Bazi:").strong());
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for feature in &self.gis_engine.features {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&feature.name).strong());
                                ui.small(format!("[{}]", feature.category));
                            });
                            ui.small(format!("Lat: {:.4}, Lon: {:.4}", feature.location.lat, feature.location.lon));
                        });
                    }
                });
            });

            // DESNA KOLONA: Rezultati i GIS Dnevnik
            cols[1].vertical(|ui| {
                let center = GeoPoint::new(self.gis_search_lat, self.gis_search_lon);
                let results = self.gis_engine.search_in_radius(&center, self.gis_search_radius);

                ui.label(egui::RichText::new(format!("Pronađeno u krugu od {:.0} km: {}", self.gis_search_radius, results.len())).strong());
                ui.separator();

                egui::ScrollArea::vertical().max_height(240.0).show(ui, |ui| {
                    if results.is_empty() {
                        ui.colored_label(egui::Color32::LIGHT_RED, "Nema objekata u zadatome radijusu.");
                    } else {
                        for (feat, dist) in results {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(&feat.name).strong());
                                    ui.colored_label(egui::Color32::GREEN, format!("{:.2} km", dist));
                                });
                                ui.small(format!("Kategorija: {} | ID: #{}", feat.category, feat.id));
                            });
                        }
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("GIS Logovi Engine-a").strong());
                egui::ScrollArea::vertical().max_height(80.0).show(ui, |ui| {
                    for log in &self.gis_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_tsdb_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("📈 Time-Series Metrics & Telemetry Engine");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Lista Metrika i Unos Novih Tačaka
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Registrovane Vremenske Serije:").strong());

                let keys: Vec<String> = self.tsdb_engine.series_map.keys().cloned().collect();
                for key in keys {
                    let is_selected = self.selected_metric == key;
                    if ui.selectable_label(is_selected, format!("📊 {}", key)).clicked() {
                        self.selected_metric = key;
                    }
                }

                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("⚡ Ubaci novu tačku:").strong());
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut self.new_metric_val).speed(1.0));
                        if ui.button("Upisi").clicked() {
                            let metric = self.selected_metric.clone();
                            let val = self.new_metric_val;
                            self.tsdb_engine.record_metric(&metric, val);
                        }
                    });
                });

                ui.add_space(5.0);
                if ui.button("🧹 Prune (Očisti sve)").clicked() {
                    self.tsdb_engine.prune_all();
                }
            });

            // DESNA KOLONA: Visual Plot i Agregatne Statistike
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new(format!("Aktivna Serija: {}", self.selected_metric)).strong());

                if let Some(series) = self.tsdb_engine.series_map.get(&self.selected_metric) {
                    ui.horizontal(|ui| {
                        let avg = series.aggregate(AggregationFunc::Average).unwrap_or(0.0);
                        let min = series.aggregate(AggregationFunc::Min).unwrap_or(0.0);
                        let max = series.aggregate(AggregationFunc::Max).unwrap_or(0.0);

                        ui.label(format!("Avg: {:.1}", avg));
                        ui.label(format!("Min: {:.1}", min));
                        ui.label(format!("Max: {:.1}", max));
                    });

                    ui.separator();
                    ui.label(egui::RichText::new("Vizuelni prikaz tačaka (Grafik):").strong());

                    // Vizuelni prikaz tačaka u vidu stubića (Bar chart)
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for pt in &series.points {
                                let height = (pt.value.abs() as f32).clamp(5.0, 120.0);
                                ui.vertical(|ui| {
                                    ui.add_space(120.0 - height);
                                    ui.colored_label(
                                        egui::Color32::LIGHT_BLUE,
                                        "|"
                                    );
                                    ui.add(egui::ProgressBar::new((pt.value / 100.0) as f32).show_percentage());
                                    ui.small(format!("{:.0}", pt.value));
                                });
                            }
                        });
                    });
                } else {
                    ui.label("Izaberite metriku sa leve strane.");
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("TSDB Logovi").strong());
                egui::ScrollArea::vertical().max_height(80.0).show(ui, |ui| {
                    for log in &self.tsdb_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_linter_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔍 Code Linter & Formatter Engine");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Code Editor i Kontrole
            cols[0].vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Izvorni Kod za Analizu:").strong());
                    if ui.button("⚡ Analiziraj").clicked() {
                        self.linter_engine.analyze();
                    }
                    if ui.button("✨ Auto-Formatiraj").clicked() {
                        self.linter_engine.format_code();
                    }
                });

                let mut code_text = self.linter_engine.source_code.clone();
                if ui.add(
                    egui::TextEdit::multiline(&mut code_text)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(18)
                        .desired_width(f32::INFINITY)
                ).changed() {
                    self.linter_engine.set_code(&code_text);
                }
            });

            // DESNA KOLONA: Diagnostics, Health Score i Sugestije
            cols[1].vertical(|ui| {
                // Health Score Indicator
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Code Health Score:").strong());
                        let score = self.linter_engine.health_score;
                        let color = if score > 80 {
                            egui::Color32::GREEN
                        } else if score > 50 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::RED
                        };
                        ui.colored_label(color, egui::RichText::new(format!("{}%", score)).strong().size(18.0));
                    });
                    ui.add(egui::ProgressBar::new(self.linter_engine.health_score as f32 / 100.0));
                });

                ui.add_space(5.0);
                ui.label(egui::RichText::new(format!("Pronađena Upozorenja ({})", self.linter_engine.issues.len())).strong());
                ui.separator();

                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    if self.linter_engine.issues.is_empty() {
                        ui.colored_label(egui::Color32::GREEN, "✔ Kod je čist! Nisu pronađene greške.");
                    } else {
                        for issue in &self.linter_engine.issues {
                            let (icon_color, tag) = match issue.level {
                                LintLevel::Error => (egui::Color32::RED, "[ERROR]"),
                                LintLevel::Warning => (egui::Color32::GOLD, "[WARN]"),
                                LintLevel::Info => (egui::Color32::LIGHT_BLUE, "[INFO]"),
                            };

                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(icon_color, tag);
                                    ui.label(egui::RichText::new(format!("Red {}", issue.line)).strong());
                                    ui.small(&issue.rule_id);
                                });
                                ui.label(&issue.message);
                                ui.small(egui::RichText::new(format!("💡 Savet: {}", issue.suggestion)).italics());
                            });
                            ui.add_space(2.0);
                        }
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Dnevnik Linter Modula").strong());
                egui::ScrollArea::vertical().max_height(80.0).show(ui, |ui| {
                    for log in &self.linter_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
     fn render_pathfinder_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗺 Graph & Pathfinding Solver (A* Algorithm)");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Interaktivna Mreža (Grid)
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Interaktivna Mreža 10x10 (Klikni za Zid)").strong());
                ui.separator();

                egui::Grid::new("pathfinding_grid").spacing([3.0, 3.0]).show(ui, |ui| {
                    for y in 0..self.pathfinder_engine.graph.height {
                        for x in 0..self.pathfinder_engine.graph.width {
                            let cell = self.pathfinder_engine.graph.grid[y][x];
                            let (bg, label) = match cell {
                                CellType::Start => (egui::Color32::GREEN, "S"),
                                CellType::Target => (egui::Color32::RED, "T"),
                                CellType::Wall => (egui::Color32::DARK_GRAY, "#"),
                                CellType::Path => (egui::Color32::YELLOW, "*"),
                                CellType::Visited => (egui::Color32::from_rgb(50, 70, 100), "·"),
                                CellType::Empty => (egui::Color32::from_rgb(30, 35, 45), " "),
                            };

                            let btn = egui::Button::new(
                                egui::RichText::new(label).strong().color(egui::Color32::BLACK)
                            ).fill(bg).min_size(egui::vec2(24.0, 24.0));

                            if ui.add(btn).clicked() {
                                self.pathfinder_engine.toggle_cell(x, y);
                            }
                        }
                        ui.end_row();
                    }
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("⚡ Izračunaj Putanju (A*)").clicked() {
                        self.pathfinder_engine.solve();
                    }
                    if ui.button("🗑 Očisti Zidove").clicked() {
                        self.pathfinder_engine.clear_walls();
                    }
                });
            });

            // DESNA KOLONA: Statistika i Logovi
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("📊 Rezultati Pretrage").strong());
                ui.separator();

                if let Some(res) = &self.pathfinder_engine.last_result {
                    ui.group(|ui| {
                        ui.label(format!("Status: {}", if res.success { "Putanja pronađena!" } else { "Blokirano!" }));
                        ui.label(format!("Dužina putanje: {} koraka", res.path.len()));
                        ui.label(format!("Istraženo čvorova: {}", res.visited_count));
                        ui.label(format!("Ukupna cena (g-cost): {:.1}", res.total_cost));
                    });
                } else {
                    ui.label("Pritisnite 'Izračunaj Putanju' za pokretanje A* algoritma.");
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Dnevnik Algoritma").strong());
                egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                    for log in &self.pathfinder_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_archiver_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("📦 Entropy & Data Compression Engine (.qarc)");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Arhiva i dodavanje fajlova
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new(format!("Aktivna Arhiva: {}", self.archiver_engine.active_archive.archive_name)).strong());
                ui.separator();

                egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                    for file in &self.archiver_engine.active_archive.files {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&file.name).strong());
                                ui.small(format!("{} B -> {} B", file.uncompressed_size, file.compressed_size));
                            });
                        });
                        ui.add_space(2.0);
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Dodaj Novi Fajl u Arhivu").strong());
                ui.horizontal(|ui| {
                    ui.label("Naziv:");
                    ui.text_edit_singleline(&mut self.arc_file_name_input);
                });
                ui.horizontal(|ui| {
                    ui.label("Sadržaj:");
                    ui.text_edit_singleline(&mut self.arc_file_content_input);
                });

                if ui.button("⚡ Kompresuj & Arhiviraj").clicked() {
                    self.archiver_engine.add_file_to_archive(
                        &self.arc_file_name_input.clone(),
                        &self.arc_file_content_input.clone(),
                    );
                }
            });

            // DESNA KOLONA: Analiza Shannon-ove Entropije i Logovi
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("📊 Shannon Entropy & RLE Analizator").strong());
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Test tekst:");
                    ui.text_edit_singleline(&mut self.arc_test_text_input);
                });

                if ui.button("🔍 Izračunaj Entropiju & RLE").clicked() {
                    self.archiver_engine.test_raw_compression(&self.arc_test_text_input.clone());
                }

                ui.add_space(5.0);
                if let Some(stats) = &self.archiver_engine.last_stats {
                    ui.group(|ui| {
                        ui.label(format!("Shannon Entropija: {:.3} bita/bajtu", stats.shannon_entropy));
                        ui.label(format!("Stepen Kompresije: {:.1}%", stats.ratio_percentage));
                        ui.add(egui::ProgressBar::new((stats.ratio_percentage / 100.0).max(0.0)).text(format!("{:.1}% Uštede", stats.ratio_percentage)));
                    });
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Dnevnik Archiver Modula").strong());
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    for log in &self.archiver_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_input_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎮 Virtual HID Controller & Input Event Emulator");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Kontrole za Emulaciju (Miš, Tastatura, Gamepad)
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("⌨ Tastatura & Miš Pad").strong());
                ui.separator();

                // Tastatura komanda
                ui.horizontal(|ui| {
                    ui.label("Taster za slanje:");
                    ui.text_edit_singleline(&mut self.vinput_key_text);
                    if ui.button("⚡ Pošalji KeyPress").clicked() {
                        self.input_engine.inject_key(&self.vinput_key_text.clone(), true);
                    }
                });

                ui.add_space(5.0);
                ui.label(egui::RichText::new("🖱 Apsolutna Podloga za Miša (Klikni / Prevuci)").strong());

                // Interaktivna površina za pomeranje virtuelnog miša
                let (response, painter) = ui.allocate_painter(egui::vec2(280.0, 140.0), egui::Sense::drag());
                let rect = response.rect;

                painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(30, 35, 45));
                painter.rect_stroke(rect, 4.0, egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE));

                if response.dragged() || response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let rel_x = (pos.x - rect.min.x) * (800.0 / rect.width());
                        let rel_y = (pos.y - rect.min.y) * (600.0 / rect.height());
                        self.input_engine.move_mouse(rel_x, rel_y);
                    }
                }

                // Crtanje indikatora kursor miša
                let norm_x = rect.min.x + (self.input_engine.mouse_x / 800.0) * rect.width();
                let norm_y = rect.min.y + (self.input_engine.mouse_y / 600.0) * rect.height();
                painter.circle_filled(egui::pos2(norm_x, norm_y), 5.0, egui::Color32::RED);

                ui.horizontal(|ui| {
                    if ui.button("L-Klik").clicked() {
                        self.input_engine.click_mouse_button("left", true);
                    }
                    if ui.button("R-Klik").clicked() {
                        self.input_engine.click_mouse_button("right", true);
                    }
                    ui.label(format!("Pozicija: ({:.0}, {:.0})", self.input_engine.mouse_x, self.input_engine.mouse_y));
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("🕹 Gamepad / Analogni Joystick").strong());
                ui.separator();

                let mut gx = self.input_engine.gamepad_axis_x;
                let mut gy = self.input_engine.gamepad_axis_y;
                let mut btn_a = self.input_engine.gamepad_btn_a;

                ui.horizontal(|ui| {
                    ui.label("Osa X:");
                    ui.add(egui::Slider::new(&mut gx, -1.0..=1.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Osa Y:");
                    ui.add(egui::Slider::new(&mut gy, -1.0..=1.0));
                });
                ui.checkbox(&mut btn_a, "Dugme A (Pritisnuto)");

                self.input_engine.update_gamepad(gx, gy, btn_a, false);
            });

            // DESNA KOLONA: Dnevnik događaja i IRQ Red
            cols[1].vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📋 Red Događaja (Input Event Queue)").strong());
                    if ui.button("🗑 Očisti").clicked() {
                        self.input_engine.clear_queue();
                    }
                });
                ui.separator();

                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for evt in self.input_engine.events_queue.iter().rev() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("#{}", evt.id)).strong());
                                ui.small(format!("[{:?}]", evt.device_type));
                            });
                            ui.small(format!("Data: {:?}", evt.event_data));
                        });
                        ui.add_space(2.0);
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Dnevnik IRQ Prekida").strong());
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    for log in &self.input_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_gop_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎨 GOP / VGA Hardware Display Driver");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Prikaz modova i kontrola
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Učitani Grafički Režimi (GOP Modes)").strong());
                ui.separator();

                for mode in self.gop_engine.available_modes.clone() {
                    let is_current = self.gop_engine.active_mode.id == mode.id;
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&mode.name).strong());
                            if is_current {
                                ui.colored_label(egui::Color32::GREEN, "[Aktivan]");
                            }
                        });
                        ui.small(format!("Rezolucija: {}x{} | Scanline: {}", mode.width, mode.height, mode.pixels_per_scan_line));
                        ui.small(format!("Format Piksela: {:?}", mode.pixel_format));

                        if !is_current && ui.button("⚡ Postavi Mod").clicked() {
                            let _ = self.gop_engine.set_mode(mode.id);
                        }
                    });
                    ui.add_space(3.0);
                }
            });

            // DESNA KOLONA: Framebuffer Preview i Hardverski Metapodaci
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("Linear Framebuffer Preview").strong());
                ui.separator();

                ui.group(|ui| {
                    ui.monospace(format!("BAR0 Adresa: 0x{:X}", self.gop_engine.framebuffer_base_address));
                    ui.monospace(format!("VGA Legacy Status: {}", if self.gop_engine.is_vga_legacy_fallback { "Aktiviran (320x200)" } else { "Onemogućen (UEFI GOP)" }));
                });

                ui.add_space(5.0);
                ui.label("Simulirani bafer okvira (Color-Bar Test Pattern):");

                // Prikaz minijaturne šeme ekrana
                let (response, painter) = ui.allocate_painter(egui::vec2(240.0, 135.0), egui::Sense::hover());
                let rect = response.rect;

                painter.rect_filled(rect, 0.0, egui::Color32::BLACK);

                let fb_w = self.gop_engine.framebuffer.width as f32;
                let fb_h = self.gop_engine.framebuffer.height as f32;
                let scale_x = rect.width() / fb_w;
                let scale_y = rect.height() / fb_h;

                // Crtanje traka iz Framebuffer-a
                for y in (0..self.gop_engine.framebuffer.height).step_by(5) {
                    for x in (0..self.gop_engine.framebuffer.width).step_by(5) {
                        let offset = ((y * self.gop_engine.framebuffer.width + x) * 4) as usize;
                        if offset + 3 < self.gop_engine.framebuffer.buffer.len() {
                            let r = self.gop_engine.framebuffer.buffer[offset];
                            let g = self.gop_engine.framebuffer.buffer[offset + 1];
                            let b = self.gop_engine.framebuffer.buffer[offset + 2];
                            let color = egui::Color32::from_rgb(r, g, b);

                            let min_p = rect.min + egui::vec2(x as f32 * scale_x, y as f32 * scale_y);
                            let max_p = min_p + egui::vec2(5.0 * scale_x, 5.0 * scale_y);
                            painter.rect_filled(egui::Rect::from_min_max(min_p, max_p), 0.0, color);
                        }
                    }
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Dnevnik Grafičkog Drajvera").strong());
                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    for log in &self.gop_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_proto_drivers_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔌 Hardware Protocol Drivers (USB xHCI & NVMe PCIe)");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: NVMe Controller & Queues
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("⚡ NVMe Controller (SQ / CQ Queues)").strong());
                ui.separator();

                ui.small(format!("Model: {}", self.proto_drivers.nvme.model));
                ui.small(format!("Firmware: {}", self.proto_drivers.nvme.firmware));
                ui.monospace(format!("SQ Tail: {} | CQ Head: {}", self.proto_drivers.nvme.sq_tail, self.proto_drivers.nvme.cq_head));

                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("LBA:");
                    ui.text_edit_singleline(&mut self.nvme_lba_input);
                });
                ui.horizontal(|ui| {
                    ui.label("Podatak:");
                    ui.text_edit_singleline(&mut self.nvme_data_input);
                });

                ui.horizontal(|ui| {
                    if ui.button("📖 Read Sector").clicked() {
                        if let Ok(lba) = self.nvme_lba_input.parse::<u64>() {
                            self.proto_drivers.nvme_read_sector(lba);
                        }
                    }
                    if ui.button("✍ Write Sector").clicked() {
                        if let Ok(lba) = self.nvme_lba_input.parse::<u64>() {
                            self.proto_drivers.nvme_write_sector(lba, &self.nvme_data_input);
                        }
                    }
                });

                ui.add_space(5.0);
                ui.label(egui::RichText::new("Poslednje Submission Queue (SQ) Komande").strong());
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    for cmd in self.proto_drivers.nvme.submission_queue.iter().rev() {
                        ui.small(format!("[SQ #{}] Opcode: {:?} | LBA: {} | Data: {}", cmd.command_id, cmd.opcode, cmd.lba, cmd.payload));
                    }
                });
            });

            // DESNA KOLONA: USB xHCI Controller
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("🔱 xHCI USB Host Controller").strong());
                ui.separator();

                for dev in &self.proto_drivers.xhci.ports {
                    ui.group(|ui| {
                        ui.label(egui::RichText::new(format!("Port {}: {}", dev.port, dev.name)).strong());
                        ui.small(format!("Speed: {:?} | VID: {:04X} PID: {:04X}", dev.speed, dev.vendor_id, dev.product_id));
                    });
                }

                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("Slanje na Port:");
                    ui.add(egui::DragValue::new(&mut self.usb_port_input).clamp_range(1..=4));
                });
                ui.text_edit_singleline(&mut self.usb_data_input);

                if ui.button("🚀 Pošalji USB Bulk Transfer").clicked() {
                    self.proto_drivers.usb_transfer(self.usb_port_input, &self.usb_data_input);
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Dnevnik Drajvera Magistrale").strong());
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    for log in &self.proto_drivers.driver_logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_fat32_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("💾 FAT32 / ESP (EFI System Partition) Parser");
        ui.separator();

        // Zaglavlje BPB Metapodataka
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Volume:").strong());
                ui.label(&self.fat32_engine.bpb.volume_label);
                ui.add_space(15.0);
                ui.label(egui::RichText::new("Klaster Size:").strong());
                ui.label(format!("{} B", self.fat32_engine.get_cluster_size_bytes()));
                ui.add_space(15.0);
                ui.label(egui::RichText::new("OEM ID:").strong());
                ui.label(&self.fat32_engine.bpb.oem_name);
            });
        });

        ui.add_space(5.0);

        ui.columns(2, |cols| {
            // LEVA KOLONA: Prikaz fajl strukture (Directory Entries)
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("EFI Stablo Fajlova").strong());
                ui.separator();

                egui::ScrollArea::vertical().max_height(240.0).show(ui, |ui| {
                    for entry in &self.fat32_engine.directory_entries {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                let icon = if entry.is_dir { "📁" } else { "📄" };
                                ui.label(format!("{} {}", icon, entry.path));
                            });
                            ui.horizontal(|ui| {
                                ui.small(format!("Start Cluster: {}", entry.start_cluster));
                                ui.small(format!("{} KB", entry.file_size_bytes / 1024));
                                if !entry.is_dir && ui.small_button("🔍 Pročitaj").clicked() {
                                    self.selected_efi_file = entry.path.clone();
                                }
                            });
                        });
                        ui.add_space(2.0);
                    }
                });
            });

            // DESNA KOLONA: Heksadecadalni / Tekstualni Pregled Sadržaja
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("Inspekcija Klastera i Sadržaja").strong());
                ui.separator();

                if self.selected_efi_file.is_empty() {
                    ui.label("Izaberite EFI fajl sa leve strane za pregled.");
                } else {
                    ui.label(egui::RichText::new(format!("Otvoren: {}", self.selected_efi_file)).strong());
                    ui.add_space(5.0);

                    let content = self.fat32_engine.read_mock_file_content(&self.selected_efi_file);
                    ui.group(|ui| {
                        ui.monospace(&content);
                    });
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Dnevnik Parser-a").strong());
                egui::ScrollArea::vertical().max_height(80.0).show(ui, |ui| {
                    for log in &self.fat32_engine.logs {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_telemetry_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("📊 System Telemetry, Profiler & Kernel Ring Buffer");
        ui.separator();

        // 1. SISTEMSKE METRIKE (SNAPSHOT)
        ui.group(|ui| {
            ui.label(egui::RichText::new("⚡ Real-Time System Metrics").strong());
            let snap = &self.telemetry_engine.snapshot;
            ui.columns(4, |cols| {
                cols[0].metric("CPU Usage", format!("{:.1}%", snap.cpu_usage_pct));
                cols[1].metric("RAM Used", format!("{} MB", snap.ram_used_mb));
                cols[2].metric("Disk IOPS", format!("{}", snap.iops));
                cols[3].metric("Context Switches", format!("{}", snap.context_switches));
            });
        });

        ui.add_space(10.0);

        ui.columns(2, |cols| {
            // LEVA KOLONA: Kernel Ring Buffer (dmesg)
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("📜 Kernel Ring Buffer (dmesg)").strong());

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.klog_subsystem_input);
                        ui.text_edit_singleline(&mut self.klog_msg_input);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("➕ Upisi INFO Log").clicked() {
                            let sub = self.klog_subsystem_input.clone();
                            let msg = self.klog_msg_input.clone();
                            self.telemetry_engine.log_kmessage(LogLevel::Info, &sub, &msg);
                        }
                        if ui.button("⚠️ Upisi ERR Log").clicked() {
                            let sub = self.klog_subsystem_input.clone();
                            let msg = self.klog_msg_input.clone();
                            self.telemetry_engine.log_kmessage(LogLevel::Error, &sub, &msg);
                        }
                    });
                });

                egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                    for entry in self.telemetry_engine.ring_buffer.read_all().iter().rev() {
                        let color = match entry.level {
                            LogLevel::Error | LogLevel::Emergency => egui::Color32::RED,
                            LogLevel::Warning => egui::Color32::YELLOW,
                            _ => egui::Color32::LIGHT_BLUE,
                        };
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("[{}]", entry.level.as_str())).color(color));
                            ui.label(format!("{}: {}", entry.subsystem, entry.message));
                        });
                    }
                });
            });

            // DESNA KOLONA: Function Profiler
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("⏱ Function Latency Profiler").strong());
                let profiler_data = self.telemetry_engine.profiler.metrics.clone();

                egui::ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
                    for (name, prof) in profiler_data {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(format!("ƒ {}", name)).strong());
                            ui.small(format!("Poziva: {} | Prosečno: {} us", prof.call_count, self.telemetry_engine.profiler.get_avg_time_us(&name)));
                            ui.small(format!("Min: {} us | Max: {} us", prof.min_time_us, prof.max_time_us));
                        });
                    }
                });
            });
        });
    }

    fn render_shell_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🐚 Quantum Shell: Pipelines, Redirections & Scripting");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Pipeline Runner & Env Variables
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("🔗 Pokreni Pipeline Komandu (| > >> <)").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.shell_input_pipeline);
                        if ui.button("⚡ Pokreni").clicked() {
                            let p = self.shell_input_pipeline.clone();
                            self.shell_engine.run_pipeline(&p);
                        }
                    });
                    ui.small("Primer: echo $USER | uppercase >> /var/log.txt");
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("💲 Promenljive Okruženja (Environment)").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.shell_new_env_key);
                        ui.label("=");
                        ui.text_edit_singleline(&mut self.shell_new_env_val);
                        if ui.button("➕ Postavi").clicked() {
                            let k = self.shell_new_env_key.clone();
                            let v = self.shell_new_env_val.clone();
                            self.shell_engine.set_env(&k, &v);
                        }
                    });

                    egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                        for (k, v) in &self.shell_engine.env_vars {
                            ui.label(format!("${} = {}", k, v));
                        }
                    });
                });
            });

            // DESNA KOLONA: Automated Scripts (.qsh) & Logs
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("📜 Automatizovane .qsh Skripte").strong());
                ui.group(|ui| {
                    let scripts = self.shell_engine.scripts.clone();
                    for (idx, scr) in scripts.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("• {}", scr.name));
                            if ui.button("▶ Pokreni Skriptu").clicked() {
                                self.shell_engine.execute_script(idx);
                            }
                        });
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("📊 Dnevnik Izvršavanja Shell-a").strong());
                egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                    for log in self.shell_engine.logs.iter().rev() {
                        ui.group(|ui| {
                            ui.small(log);
                        });
                    }
                });
            });
        });
    }
    fn render_pkg_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("📦 QPM Package Manager & Dynamic ELF Loader");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Instalacija i Repozitorijum
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("🌐 Dostupni Paketi (Repo Catalog)").strong());
                ui.group(|ui| {
                    let repo_items: Vec<_> = self.pkg_engine.repository_catalog.clone();
                    for p in repo_items {
                        ui.horizontal(|ui| {
                            ui.label(format!("• {} (v{})", p.name, p.version));
                            let is_installed = self.pkg_engine.installed_packages.contains_key(&p.name);
                            if is_installed {
                                ui.label(egui::RichText::new("[Instaliran]").color(egui::Color32::GREEN));
                            } else if ui.button("📥 Instaliraj").clicked() {
                                self.pkg_engine.install_package(&p.name);
                            }
                        });
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("⚡ Učitavanje Binarnog Modula (.qmod / ELF)").strong());
                ui.group(|ui| {
                    if ui.button("⚙ Učitaj Novi ELF Modul u Memoriju").clicked() {
                        let demo_elf = vec![0x7F, b'E', b'L', b'F', 2, 1, 1, 0];
                        let _ = self.pkg_engine.load_qmod_bytes(&demo_elf);
                    }
                });
            });

            // DESNA KOLONA: Živi Učitani Moduli u Memoriji
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("🧠 Učitani Dinamički Moduli u Memoriji").strong());
                let loaded: Vec<_> = self.pkg_engine.loaded_modules.clone().into_iter().collect();

                egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                    for (name, m) in loaded {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(format!("Modul: {}", name)).strong());
                            ui.small(format!("Adresa: 0x{:X}", m.base_address));
                            ui.small(format!("Izvezeni Simboli: {:?}", m.exported_symbols));
                            if let Some(ref elf) = m.elf_details {
                                ui.small(format!("ELF Klasa: {:?}, Sekcija: {}", elf.class, elf.sections.len()));
                            }
                            if ui.button("🔌 Izbaci iz Memorije").clicked() {
                                self.pkg_engine.unload_module(&name);
                            }
                        });
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("📜 Dnevnik Upravljača Paketima").strong());
                egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                    for log in self.pkg_engine.logs.iter().rev() {
                        ui.small(log);
                    }
                });
            });
        });
    }
    fn render_ipc_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔄 IPC, POSIX Signals & Named Pipes Engine");
        ui.separator();

        ui.columns(2, |cols| {
            // LEVA KOLONA: Message Queues & Signals
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("✉ Slanje IPC Poruka i Signala").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Od PID:");
                        ui.add(egui::DragValue::new(&mut self.ipc_sender_pid));
                        ui.label("Za PID:");
                        ui.add(egui::DragValue::new(&mut self.ipc_target_pid));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Poruka:");
                        ui.text_edit_singleline(&mut self.ipc_msg_text);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("✉ Pošalji Poruku").clicked() {
                            let from = self.ipc_sender_pid;
                            let to = self.ipc_target_pid;
                            let txt = self.ipc_msg_text.clone();
                            self.ipc_engine.send_message(from, to, &txt);
                        }
                        if ui.button("💥 SIGKILL").clicked() {
                            let from = self.ipc_sender_pid;
                            let to = self.ipc_target_pid;
                            self.ipc_engine.send_signal(from, to, Signal::SIGKILL);
                        }
                        if ui.button("⚠️ SIGUSR1").clicked() {
                            let from = self.ipc_sender_pid;
                            let to = self.ipc_target_pid;
                            self.ipc_engine.send_signal(from, to, Signal::SIGUSR1);
                        }
                    });
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("🚰 Named Pipes (Cevovodi)").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Pipe Ime:");
                        ui.text_edit_singleline(&mut self.ipc_pipe_name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Sadržaj:");
                        ui.text_edit_singleline(&mut self.ipc_pipe_data);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("⬆ Upiši u Pipe").clicked() {
                            let p = self.ipc_pipe_name.clone();
                            let d = self.ipc_pipe_data.clone();
                            self.ipc_engine.write_pipe(&p, &d);
                        }
                        if ui.button("⬇ Pročitaj iz Pipe-a").clicked() {
                            let p = self.ipc_pipe_name.clone();
                            self.ipc_engine.read_pipe(&p);
                        }
                    });
                });
            });

            // DESNA KOLONA: Dnevnik Događaja
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("📜 Dnevnik IPC Događaja i Aktivnosti").strong());
                egui::ScrollArea::vertical().max_height(350.0).show(ui, |ui| {
                    for log in self.ipc_engine.logs.iter().rev() {
                        ui.group(|ui| {
                            ui.small(log);
                        });
                    }
                });
            });
        });
    }
}

//Vise te je uspavao tockic na misu nego enrikeove balade (Enrike iglesias)
//PRIZNAJ!!!