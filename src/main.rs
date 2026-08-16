mod domain;
mod driver;
mod gui;
mod task_manager;
mod vfs;
mod audio;
mod vm;
mod network;
mod db;
mod terminal;
mod container;
mod video;
mod image_engine;
mod compiler;
mod search_engine;
mod crypto;
mod nvram;
mod boot_manager;
mod virtual_input;
mod tsdb;
mod streaming;
mod protocol_drivers;
mod pathfinding;
mod linter;
mod hft;
mod gop_vga;
mod gis;
mod efi_fat32;
mod compression;
mod telemetry;
mod ipc;
mod pkg_loader;
mod shell_automation;
mod allocator;
mod bio_comp;
mod branch_predictor;
mod cache_numa;
mod cache;
mod compute_raymarch;
mod dod;
mod drivers;
mod ebpf;
mod ecs;
mod formal_proofs;
mod hazard_ebr;
mod intrinsics;
mod jit_engine;
mod lockfree;
mod lockfree_aba;
mod mechanical_sympathy;
mod memory;
mod memory_fences;
mod mesh_geometry;
mod pcie_dma;
mod photonic_asm;
mod pipeline;
mod pipeline_sim;
mod quantum_asm;
mod scheduler;
mod sdf_gi;
mod simd;
mod spectre_meltdown;
mod swar_vector;
pub mod tlb_paging;
pub mod upscaler;
pub mod zero_div;
pub mod cache_coherence;
pub mod cache_oblivious;
pub mod hardware;
pub mod raytracing;
pub mod security;
pub mod consensus;
pub mod cpu;
pub mod interrupts;
pub mod storage;
pub mod verification;

use gui::QuantumOSWinApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Windows Quantum Simulator OS")
            .with_inner_size([780.0, 560.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Quantum OS Windows",
        options,
        Box::new(|_cc| Box::new(QuantumOSWinApp::new())),
    )
}

//SOFIA VERGARA TE AMO!!!❤️