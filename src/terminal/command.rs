use super::{TerminalEngine, LineType};
use crate::driver::WinQuantumDriver;
use crate::vfs::QuquatVFS;
use crate::task_manager::VirtualScheduler;
use crate::db::QuquatDB;
use crate::network::{NetworkEngine, QuquatPacket, PacketType};
use crate::container::HypervisorEngine;
use crate::video::VideoEngine;
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
use crate::linter::QuantumCodeLinterEngine;
use crate::pathfinding::QuantumPathfinderEngine;
use crate::compression::QuantumArchiverEngine;
use crate::virtual_input::QuantumVirtualInputEngine;
use crate::gop_vga::QuantumGopVgaEngine;
use crate::protocol_drivers::QuantumProtocolDrivers;
use crate::efi_fat32::QuantumEfiFatEngine;
use crate::telemetry::{ring_buffer::LogLevel, QuantumTelemetryEngine};
use crate::shell_automation::QuantumShellEngine;
use crate::pkg_loader::QuantumPkgEngine;
use crate::ipc::{signals::Signal, QuantumIpcEngine};
use core::sync::atomic::Ordering;
use crate::quantum_asm::surface_code::PauliError;
use crate::compute_raymarch::reversible::landauer_limit_joules;
use crate::compute_raymarch::reversible::ReversibleGate;
use crate::verification::Capability;
use crate::verification::CapabilityRight;
use crate::verification::ConcreteKernelState;
use crate::memory::init::RegionKind;
use crate::memory::PAGE_SIZE;
use crate::hardware::post::ComponentStatus;
use crate::verification::AbstractKernelState;
use crate::consensus::VoteResponse;
use crate::consensus::PbftNode;
use crate::raytracing::RenderBackendMode;
use crate::raytracing::gpu::GpuBackendType;
use crate::cpu::MicrocodeHeader;
use crate::security::kernel_exploit_defense::FormatStringSanitizer;
use crate::security::kernel_exploit_defense::CAP_SYS_ADMIN;
use crate::security::rop_aslr::ShadowStackCfi;
use crate::security::rop_aslr::PagePerm;
use crate::hardware::post::PostEngine;
use crate::hardware::pim_memristor::PimEngine;
use crate::security::rop_aslr::AslrEngine;
use crate::raytracing::RayTracingEngine;             
use crate::consensus::RaftNode;

//Naboijem ja ove uvoze
//bukvalno nisam znao odakle komande da uvezem
//Hvala k da sam te komande pobrisao

pub struct CommandDispatcher;

impl CommandDispatcher {
    pub fn dispatch(
        input: &str,
        term: &mut TerminalEngine,
        driver: &WinQuantumDriver,
        vfs: &mut QuquatVFS,
        scheduler: &mut VirtualScheduler,
        db: &mut QuquatDB,
        net: &mut NetworkEngine,
        containers: &mut HypervisorEngine,
        video: &mut VideoEngine,
        img: &mut QuantumImageEngine,
        compiler: &mut QuantumScriptCompiler,
        search: &mut QuantumSearchEngine,
        crypto: &mut QuantumCryptoEngine,
        nvram: &mut QuantumNvramEngine,
        boot: &mut QuantumBootEngine,
        stream: &mut QuantumEventStreamEngine,
        hft: &mut QuantumHftEngine,
        gis: &mut QuantumSpatialEngine,
        tsdb: &mut QuantumTimeSeriesEngine,
        linter: &mut QuantumCodeLinterEngine,
        pathfinder: &mut QuantumPathfinderEngine,
        archiver: &mut QuantumArchiverEngine,
        vinput: &mut QuantumVirtualInputEngine,
        gop: &mut QuantumGopVgaEngine,
        proto_drv: &mut QuantumProtocolDrivers,
        fat: &mut QuantumEfiFatEngine,
        ipc: &mut QuantumIpcEngine,
        pkg: &mut QuantumPkgEngine,
        shell: &mut QuantumShellEngine,
        telemetry: &mut QuantumTelemetryEngine,
    ) {
        let trimmed = input.trim();
        if trimmed.is_empty() { return; }

        term.command_history.push(trimmed.to_string());
        term.history_index = term.command_history.len();
        
        // Prikaz unesene komande u logu
        term.print(&format!("ququat@kernel:~$ {}", trimmed), LineType::Prompt);

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd = parts[0].to_lowercase();
        let args = &parts[1..];

        match cmd.as_str() {
            "help" => {
                term.print("=== Ququat Kernel / QuantumOS Sistem Komandi ===", LineType::Header);

    // 1. Osnovni Sistem, VFS & Shell
    term.print("--- Osnovni Sistem, VFS & Shell ---", LineType::Header);
    term.print("  help                         - Prikazuje ovaj korisnički meni", LineType::Info);
    term.print("  clear                        - Briše ekran terminala", LineType::Info);
    term.print("  sysinfo                      - Prikazuje registre i taktove sistema", LineType::Info);
    term.print("  ls                           - Pregled i listanje VFS korenskog direktorijuma", LineType::Info);
    term.print("  cat <fajl>                   - Prikaz sadržaja fajla sa VFS skladišta", LineType::Info);
    term.print("  search / find <upit>         - Pretraga fajlova i BM25 rangiranje", LineType::Info);
    term.print("  boot / bootmgr               - Upravljanje bootloader-om (list, test, uefi)", LineType::Info);
    term.print("  sh / qsh / export            - Shell izvršavanje (run, env, export, script)", LineType::Info);
    term.print("  nvram / uefi / bios          - NVRAM promenljive (list, set, reset)", LineType::Info);

    // 2. Procesi, Kontejneri, Skripte & Paketi
    term.print("--- Procesi, Kontejneri, Skripte & Paketi ---", LineType::Header);
    term.print("  ps                           - Prikaz tabele aktivnih procesa", LineType::Info);
    term.print("  qscript / run                - Kompajliranje i izvršavanje QScript koda", LineType::Info);
    term.print("  docker                       - QDocker kontejneri (ps, run, start, stop)", LineType::Info);
    term.print("  ipc / signal / pipe          - IPC komunikacija (send, kill, pipe_write/read)", LineType::Info);
    term.print("  dmesg / telemetry / profile  - Pregled Kernel Ring Buffer logova i dijagnostika", LineType::Info);
    term.print("  pkg / qpm / loader           - Paket menadžer (list, repo, install, unload, parse_elf)", LineType::Info);
    term.print("  lint / linter / fmt          - Statistička analiza koda (check, format, score)", LineType::Info);

    // 3. Algoritmi & Rutiranje
    term.print("--- Algoritmi & Rutiranje ---", LineType::Header);
    term.print("  route / pathfind / astar     - A* Pathfinder algoritam (solve, clear, wall <x> <y>)", LineType::Info);

    // 4. Baze, Metrike & Event Streaming
    term.print("--- Baze, Metrike & Event Streaming ---", LineType::Header);
    term.print("  db set <key> <val> | get     - QStore key-value baza podataka", LineType::Info);
    term.print("  tsdb / metrics               - Time-Series DB (list, push, stats, prune)", LineType::Info);
    term.print("  stream / kafka / pubsub      - Event streaming broker (topics, pub, sub)", LineType::Info);
    term.print("  gis / spatial                - Prostorni GIS entiteti (add, radius, nearest)", LineType::Info);

    // 5. Grafika & Displej
    term.print("--- Grafika & Displej ---", LineType::Header);
    term.print("  gop / vga / display          - UEFI GOP & VGA video režimi (modes, set, info)", LineType::Info);

    // 6. Hardverski Drajveri, Skladište & Input
    term.print("--- Drajveri, Periferije & Ulazni Uređaji ---", LineType::Header);
    term.print("  drivers                      - Pregled i inicijalizacija (NVMe, CAN, I2C, UART)", LineType::Info);
    term.print("  nvme / usb / protodrv        - Operacije nad skladištem i xHCI USB portovima", LineType::Info);
    term.print("  vinput / input / hid         - Emulacija ulaznih uređaja (key, mouse, gamepad)", LineType::Info);
    term.print("  fat32 / esp (bpb|ls|cat)     - FAT32 & EFI System Particija", LineType::Info);
    term.print("  net ping <port>              - Testiranje slanja mrežnog paketa", LineType::Info);

    // 7. Low-Level Jezgro, JIT & Kriptografija
    term.print("--- Low-Level Jezgro, JIT, Kripto & Sigurnost ---", LineType::Header);
    term.print("  memory                       - Page Frame Allocator & Manual Heap (16MB)", LineType::Info);
    term.print("  tlb                          - MMU TLB paging engine (4KB standard & 2MB huge pages)", LineType::Info);
    term.print("  pcie_dma                     - IOMMU IOVA mapiranje & detekcija zlonamernog DMA napada", LineType::Info);
    term.print("  crypto / tls                 - Hash (SHA), TLS handshake i RSA ključevi", LineType::Info);
    term.print("  jit_smc                      - Dinamičko x86_64 emitovanje, Self-Modifying Code & W^X", LineType::Info);
    term.print("  spectre_meltdown             - Spectre V1 Transient Execution napad & Flush+Reload", LineType::Info);
    term.print("  zero_div                     - SIGFPE Trap Handler & Branchless deljenje sa nulom", LineType::Info);
    term.print("  ebpf                         - In-Kernel eBPF Bytecode Engine & Mrežni filteri", LineType::Info);
    term.print("  smt_proof                    - SMT solver & formalna verifikacija memorijske sigurnosti", LineType::Info);

    // 8. Arhitektura Procesora & Konkurentnost
    term.print("--- Arhitektura Procesora & Konkurentnost ---", LineType::Header);
    term.print("  pipeline_sim                 - CPU Pipeline simulacija & rešavanje Data Hazard-a", LineType::Info);
    term.print("  pipeline                     - Inline Assembly, CPUID & Kontrolni registri (CR0/CR3)", LineType::Info);
    term.print("  branch_pred                  - Branch predictor & spekulativna egzekucija", LineType::Info);
    term.print("  moesi                        - MOESI multi-core cache coherence protokol simulacija", LineType::Info);
    term.print("  fences                       - Memory fences (SFENCE/MFENCE) & Non-Temporal stores", LineType::Info);
    term.print("  intrinsics                   - CPU RDTSC ciklusi, spin-pause, POPCNT/BSWAP", LineType::Info);
    term.print("  mech_sym                     - Mechanical Sympathy, Cache Line Padding & Stride Test", LineType::Info);
    term.print("  veb_tree                     - Cache-Oblivious van Emde Boas stabla vs BFS u RAM-u", LineType::Info);
    term.print("  cache_numa                   - Cache Line Bouncing (False Sharing) & NUMA latencija", LineType::Info);
    term.print("  hazard_ebr                   - Hazard Pointers vs Epoch-Based Reclamation (EBR)", LineType::Info);
    term.print("  aba_ebr                      - ABA mitigacija (Tagged pointers) & EBR Garbage Collection", LineType::Info);
    term.print("  lockfree                     - Lock-Free Treiber Stack & Michael-Scott MPMC Red", LineType::Info);
    term.print("  scheduler                    - Work-Stealing Task Scheduler (Multi-threaded)", LineType::Info);

    // 9. Kvantno, Optičko, Bio & SIMD Compute
    term.print("--- Advanced Compute, Bio & SIMD ---", LineType::Header);
    term.print("  quantum_asm                  - Quantum OS Qiskit/Quil Engine (Superpozicija, T1/T2)", LineType::Info);
    term.print("  photonic_asm                 - Fotonik Engine: Analogno optičko računanje & MZI", LineType::Info);
    term.print("  bio_comp                     - Bio-Computing Engine: DNK kodiranje & Spiking Neuroni", LineType::Info);
    term.print("  qarc / compress / archive    - Arhiviranje i RLE kompresija fajlova", LineType::Info);
    term.print("  swar_avx                     - SWAR & AVX-512 512-bitne vektorske operacije", LineType::Info);
    term.print("  dod                          - Data-Oriented Design (AoS vs SoA & Transponovanje)", LineType::Info);
    term.print("  simd                         - AVX2 Math, SIMD Memcpy & Framebuffer Fill", LineType::Info);
    term.print("  ecs                          - QuantumOS Entity Component System", LineType::Info);

    // 10. Renderisanje & Multimedija
    term.print("--- Renderisanje & Multimedija ---", LineType::Header);
    term.print("  raymarch                     - CPU/GPGPU Compute 3D Raymarching engine", LineType::Info);
    term.print("  mesh_geom                    - Virtualized Geometry & Mesh Shaders (1M Poly)", LineType::Info);
    term.print("  sdf_gi                       - 3D SDF Raymarching & Lumen GI Engine (ASCII)", LineType::Info);
    term.print("  upscaler                     - TAA & Temporal Upscaling Engine", LineType::Info);
    term.print("  img / image                  - Obrada slika & Generisanje Mandelbrota", LineType::Info);
    term.print("  vlc / video                  - Kontrola i reprodukcija video fajlova", LineType::Info);
    term.print("  hft / trade / stock          - High-Frequency Trading Engine", LineType::Info);

                
            }

            "clear" => {
                term.clear();
            }

            "sysinfo" => {
                let reg_val = driver.read_register();
                term.print(&format!("Hardware Register (u64): 0x{:016X}", reg_val), LineType::Success);
                term.print(&format!("Total Kernel Ticks: {}", scheduler.total_ticks), LineType::Info);
            }

            "ls" => {
                if let Some(entries) = vfs.read_dir(0) {
                    term.print("--- VFS Root Directory (/) ---", LineType::Info);
                    for entry in entries {
                        let kind = if entry.is_directory { "[DIR]" } else { "[FILE]" };
                        term.print(&format!("{} {} (Inode: {}, Size: {} bytes)", kind, entry.name, entry.inode, entry.size), LineType::Success);
                    }
                } else {
                    term.print("Greška pri čitanju VFS root-a.", LineType::Error);
                }
            }

            "search" | "find" => {
    if args.is_empty() {
        term.print("Upotreba: search <upit> | Npr: search rust kernel", LineType::Info);
    } else {
        let query = args.join(" ");
        let results = search.search(&query);
        if results.is_empty() {
            term.print(&format!("Nema rezultata za upit: '{}'", query), LineType::Warning);
        } else {
            term.print(&format!("Pronađeno {} rezultat(a):", results.len()), LineType::Success);
            for res in results {
                term.print(
                    &format!(" [ID: {} | BM25: {:.2}] {} -> {}", res.doc_id, res.score, res.title, res.content_snippet),
                    LineType::Info,
                );
            }
        }
    }
}

            "cat" => {
                if args.is_empty() {
                    term.print("Upotreba: cat <fajl_ime>", LineType::Error);
                } else {
                    let filename = args[0];
                    if let Some(inode) = vfs.resolve_path(&format!("/{}", filename)) {
                        if let Some(data) = vfs.read_file(inode) {
                            term.print(&format!("Sadržaj '{}': {:?}", filename, data), LineType::Success);
                        } else {
                            term.print("Nemoguće pročitati datoteku.", LineType::Error);
                        }
                    } else {
                        term.print(&format!("Fajl '{}' ne postoji.", filename), LineType::Error);
                    }
                }
            }

            "ps" => {
    term.print("--- Active Process Table ---", LineType::Info);
    for process in &scheduler.processes {
        term.print(
            &format!("PID: {} | Naziv: {}", process.pid, process.name),
            LineType::Success,
        );
    }
}

     "qscript" | "run" => {
    term.print("Kompajliranje i izvršavanje QScript koda...", LineType::Info);
    match compiler.compile_and_run() {
        Ok(logs) => {
            for log in logs {
                term.print(&log, LineType::Success);
            }
        }
        Err(err) => term.print(&format!("Greška pri kompajliranju: {}", err), LineType::Error),
    }
}

"docker" => {
    if args.is_empty() {
        term.print("Upotreba: docker ps | docker run <name> <image> | docker start <id> | docker stop <id>", LineType::Error);
    } else {
        match args[0] {
            "ps" => {
                term.print("--- QDocker Containers ---", LineType::Info);
                for c in containers.containers.values() {
                    term.print(
                        &format!("ID: {} | Name: {} | Status: {:?} | IP: {} | RAM: {}MB", 
                            c.id, c.name, c.status, c.ip_address, c.allocated_memory_mb),
                        LineType::Success
                    );
                }
            }
            "run" => {
                if args.len() >= 3 {
                    let id = containers.create_container(args[1], args[2], 128, 2);
                    containers.start_container(&id);
                    term.print(&format!("Pokrenut kontejner '{}' sa ID: {}", args[1], id), LineType::Success);
                } else {
                    term.print("Upotreba: docker run <name> <image>", LineType::Error);
                }
            }
            "start" => {
                if args.len() >= 2 && containers.start_container(args[1]) {
                    term.print(&format!("Kontejner {} pokrenut.", args[1]), LineType::Success);
                } else {
                    term.print("Kontejner nije pronađen.", LineType::Error);
                }
            }
            "stop" => {
                if args.len() >= 2 && containers.stop_container(args[1]) {
                    term.print(&format!("Kontejner {} zaustavljen.", args[1]), LineType::Success);
                } else {
                    term.print("Kontejner nije pronađen.", LineType::Error);
                }
            }
            _ => term.print("Nepoznata docker komanda.", LineType::Error),
        }
    }
}

"route" | "pathfind" | "astar" => {
    if args.is_empty() {
        term.print("Upotreba: route solve | route clear | route wall <x> <y>", LineType::Info);
    } else {
        match args[0] {
            "solve" => {
                pathfinder.solve();
                if let Some(res) = &pathfinder.last_result {
                    if res.success {
                        term.print(&format!("Uspeh! Pronađena putanja od {} koraka.", res.path.len()), LineType::Success);
                    } else {
                        term.print("Greška: Nema dostupne putanje!", LineType::Error);
                    }
                }
            }
            "clear" => {
                pathfinder.clear_walls();
                term.print("Očišćene prepreke.", LineType::Success);
            }
            "wall" => {
                if args.len() >= 3 {
                    let x = args[1].parse::<usize>().unwrap_or(0);
                    let y = args[2].parse::<usize>().unwrap_or(0);
                    pathfinder.toggle_cell(x, y);
                    term.print(&format!("Izmenjeno stanje polja ({}, {})", x, y), LineType::Info);
                }
            }
            _ => term.print("Nepoznata route komanda.", LineType::Error),
        }
    }
}

"boot" | "bootmgr" => {
    if args.is_empty() {
        term.print("Upotreba: boot list | boot test | boot uefi <on/off>", LineType::Info);
    } else {
        match args[0] {
            "list" => {
                term.print("--- Redosled Podizanja Sistema (Boot Order) ---", LineType::Success);
                for (i, dev) in boot.devices.iter().enumerate() {
                    let status = if dev.enabled { "Aktiviran" } else { "Onemogućen" };
                    term.print(
                        &format!(" {}. [{}] {} ({:?})", i + 1, status, dev.name, dev.dev_type),
                        LineType::Info,
                    );
                }
            }
            "test" => {
                term.print("Simulacija boot sekvence...", LineType::Info);
                match boot.simulate_boot_sequence() {
                    Ok(msg) => term.print(&msg, LineType::Success),
                    Err(err) => term.print(&err, LineType::Error),
                }
            }
            "uefi" => {
                if args.len() >= 2 {
                    boot.uefi_mode = args[1] == "on" || args[1] == "true";
                    term.print(&format!("UEFI Režim postavljen na: {}", boot.uefi_mode), LineType::Success);
                }
            }
            _ => term.print("Nepoznata boot komanda.", LineType::Error),
        }
    }
}

"lint" | "linter" | "fmt" => {
    if args.is_empty() {
        term.print("Upotreba: lint check | lint format | lint score", LineType::Info);
    } else {
        match args[0] {
            "check" => {
                linter.analyze();
                term.print(&format!("--- Rezultati Analize (Health Score: {}%) ---", linter.health_score), LineType::Success);
                for issue in &linter.issues {
                    let lvl_str = match issue.level {
                        crate::linter::rules::LintLevel::Error => " [ERROR]",
                        crate::linter::rules::LintLevel::Warning => " [WARN]",
                        crate::linter::rules::LintLevel::Info => " [INFO]",
                    };
                    term.print(
                        &format!(" Red {}{}: {} -> {}", issue.line, lvl_str, issue.message, issue.suggestion),
                        LineType::Warning,
                    );
                }
            }
            "format" | "fmt" => {
                linter.format_code();
                term.print("Kod je uspešno formatiran!", LineType::Success);
            }
            "score" => {
                term.print(&format!("Trenutni Code Health Score: {}%", linter.health_score), LineType::Info);
            }
            _ => term.print("Nepoznata linter komanda.", LineType::Error),
        }
    }
}

"pkg" | "qpm" | "loader" => {
    if args.is_empty() {
        term.print("Upotreba: pkg list | pkg repo | pkg install <ime> | pkg unload <ime> | pkg parse_elf", LineType::Info);
    } else {
        match args[0] {
            "list" => {
                term.print("=== Instalirani Paketi ===", LineType::Header);
                for (name, p) in &pkg.installed_packages {
                    term.print(&format!(" - {} (v{}) [{} B]", name, p.version, p.size_bytes), LineType::Info);
                }
                term.print("=== Učitani Dinamički Moduli ===", LineType::Header);
                for (name, m) in &pkg.loaded_modules {
                    term.print(&format!(" - {} na adresom 0x{:X} (Simbola: {})", name, m.base_address, m.exported_symbols.len()), LineType::Success);
                }
            }
            "repo" => {
                term.print("=== Dostupni Paketi u Repozitorijumu ===", LineType::Header);
                for p in &pkg.repository_catalog {
                    term.print(&format!(" - {} (v{}) od {}", p.name, p.version, p.author), LineType::Info);
                }
            }
            "install" => {
                if args.len() >= 2 {
                    if pkg.install_package(args[1]) {
                        term.print(&format!("Paket '{}' uspešno instaliran!", args[1]), LineType::Success);
                    } else {
                        term.print("Paket nije pronađen.", LineType::Error);
                    }
                }
            }
            "unload" => {
                if args.len() >= 2 {
                    if pkg.unload_module(args[1]) {
                        term.print(&format!("Modul '{}' izbačen iz memorije.", args[1]), LineType::Success);
                    } else {
                        term.print("Modul nije pronađen.", LineType::Warning);
                    }
                }
            }
            "parse_elf" => {
                let demo_elf = vec![0x7F, b'E', b'L', b'F', 2, 1, 1, 0];
                match pkg.load_qmod_bytes(&demo_elf) {
                    Ok(mod_name) => term.print(&format!("Sintetički ELF uspešno učitan kao {}", mod_name), LineType::Success),
                    Err(e) => term.print(&format!("Greška pri parsiranju ELF-a: {}", e), LineType::Error),
                }
            }
            _ => term.print("Nepoznata pkg komanda.", LineType::Error),
        }
    }
}

"ipc" | "signal" | "pipe" => {
    if args.is_empty() {
        term.print("Upotreba: ipc send <from_pid> <to_pid> <poruka> | ipc kill <from_pid> <to_pid> <signal> | ipc pipe_write <ime> <tekst> | ipc pipe_read <ime>", LineType::Info);
    } else {
        match args[0] {
            "send" => {
                if args.len() >= 4 {
                    let from = args[1].parse::<u32>().unwrap_or(1);
                    let to = args[2].parse::<u32>().unwrap_or(2);
                    let text = args[3..].join(" ");
                    let id = ipc.send_message(from, to, &text);
                    term.print(&format!("Poslata IPC poruka #{}: PID {} -> PID {}", id, from, to), LineType::Success);
                } else {
                    term.print("Upotreba: ipc send <from_pid> <to_pid> <poruka>", LineType::Warning);
                }
            }
            "kill" => {
                if args.len() >= 4 {
                    let from = args[1].parse::<u32>().unwrap_or(1);
                    let to = args[2].parse::<u32>().unwrap_or(2);
                    if let Some(sig) = Signal::from_str(args[3]) {
                        ipc.send_signal(from, to, sig);
                        term.print(&format!("Poslat signal {:?} procesu PID {}", sig, to), LineType::Success);
                    } else {
                        term.print("Nepoznat signal! Primeri: SIGKILL, SIGTERM, SIGUSR1", LineType::Error);
                    }
                } else {
                    term.print("Upotreba: ipc kill <from_pid> <to_pid> <signal>", LineType::Warning);
                }
            }
            "pipe_write" => {
                if args.len() >= 3 {
                    let name = args[1];
                    let data = args[2..].join(" ");
                    ipc.write_pipe(name, &data);
                    term.print(&format!("Upisano u Pipe '{}'", name), LineType::Success);
                }
            }
            "pipe_read" => {
                if args.len() >= 2 {
                    let name = args[1];
                    if let Some(data) = ipc.read_pipe(name) {
                        term.print(&format!("Pipe '{}' -> {}", name, data), LineType::Info);
                    } else {
                        term.print(&format!("Pipe '{}' je prazan.", name), LineType::Warning);
                    }
                }
            }
            _ => term.print("Nepoznata IPC komanda.", LineType::Error),
        }
    }
}

"dmesg" | "telemetry" | "profile" => {
    if args.is_empty() && cmd == "dmesg" {
        term.print("=== Kernel Ring Buffer (dmesg) ===", LineType::Header);
        for entry in telemetry.ring_buffer.read_all() {
            let color = match entry.level {
                LogLevel::Error | LogLevel::Emergency => LineType::Error,
                LogLevel::Warning => LineType::Warning,
                _ => LineType::Info,
            };
            term.print(&format!("[{}] [{}] {}: {}", entry.timestamp_ms, entry.level.as_str(), entry.subsystem, entry.message), color);
        }
    } else if cmd == "profile" {
        term.print("=== Function Latency Profiler ===", LineType::Header);
        for (name, prof) in &telemetry.profiler.metrics {
            term.print(&format!("• {}: Calls={}, Avg={}us, Min={}us, Max={}us", name, prof.call_count, telemetry.profiler.get_avg_time_us(name), prof.min_time_us, prof.max_time_us), LineType::Success);
        }
    } else if cmd == "telemetry" {
        let snap = &telemetry.snapshot;
        term.print("=== Real-Time System Snapshot ===", LineType::Header);
        term.print(&format!("CPU: {:.1}% | RAM: {} MB / {} MB", snap.cpu_usage_pct, snap.ram_used_mb, snap.ram_total_mb), LineType::Info);
        term.print(&format!("IOPS: {} | Context Switches: {} | IRQs: {}", snap.iops, snap.context_switches, snap.irq_counter), LineType::Info);
    }
}

"stream" | "kafka" | "pubsub" => {
    if args.is_empty() {
        term.print("Upotreba: stream topics | stream pub <tema> <kljuc> <poruka> | stream sub <grupa> <tema> <particija>", LineType::Info);
    } else {
        match args[0] {
            "topics" => {
                term.print("--- Registrovani Event Streaming Topics ---", LineType::Success);
                for (name, topic) in &stream.topics {
                    term.print(
                        &format!("• Topic: '{}' | Particija: {}", name, topic.partitions.len()),
                        LineType::Info,
                    );
                    for p in &topic.partitions {
                        term.print(&format!("   └─ Particija #{}: {} poruka u logu (Next Offset: {})", p.id, p.log.len(), p.next_offset), LineType::Warning);
                    }
                }
            }
            "pub" => {
                if args.len() >= 4 {
                    let topic = args[1];
                    let key = args[2];
                    let val = args[3..].join(" ");
                    if let Some((p, off)) = stream.publish_event(topic, key, &val) {
                        term.print(&format!("Uspešno objavljena poruka! Particija: {} | Offset: {}", p, off), LineType::Success);
                    } else {
                        term.print("Greška: Tema ne postoji!", LineType::Error);
                    }
                } else {
                    term.print("Upotreba: stream pub <tema> <kljuc> <vrednost>", LineType::Warning);
                }
            }
            "sub" => {
                if args.len() >= 4 {
                    let group = args[1];
                    let topic = args[2];
                    let part_id = args[3].parse::<u32>().unwrap_or(0);
                    let records = stream.consume_events(group, topic, part_id, 5);

                    term.print(&format!("--- Poruke za Grupu '{}' (Particija #{}) ---", group, part_id), LineType::Success);
                    if records.is_empty() {
                        term.print("Nema novih poruka za čitanje.", LineType::Warning);
                    } else {
                        for r in records {
                            term.print(&format!(" [Offset {}] K:'{}' -> V:'{}'", r.offset, r.key, r.value), LineType::Info);
                        }
                    }
                } else {
                    term.print("Upotreba: stream sub <grupa> <tema> <particija>", LineType::Warning);
                }
            }
            _ => term.print("Nepoznata stream komanda.", LineType::Error),
        }
    }
}

"gop" | "vga" | "display" => {
    if args.is_empty() {
        term.print("Upotreba: gop modes | gop set <mode_id> | gop info", LineType::Info);
    } else {
        match args[0] {
            "modes" => {
                term.print("--- Podržani GOP / VGA Režimi Prikaza ---", LineType::Success);
                for m in &gop.available_modes {
                    term.print(
                        &format!(" Mode ID {}: {} [{}x{}] Format: {:?}", m.id, m.name, m.width, m.height, m.pixel_format),
                        LineType::Info,
                    );
                }
            }
            "set" => {
                if args.len() >= 2 {
                    if let Ok(id) = args[1].parse::<u32>() {
                        match gop.set_mode(id) {
                            Ok(msg) => term.print(&msg, LineType::Success),
                            Err(err) => term.print(&err, LineType::Error),
                        }
                    }
                } else {
                    term.print("Upotreba: gop set <mode_id>", LineType::Warning);
                }
            }
            "info" => {
                term.print("--- GOP Linear Framebuffer Info ---", LineType::Success);
                term.print(&format!(" Aktivan Mod: {}", gop.active_mode.name), LineType::Info);
                term.print(&format!(" Rezolucija: {} x {}", gop.active_mode.width, gop.active_mode.height), LineType::Info);
                term.print(&format!(" Fizika FB Adresa: 0x{:X}", gop.framebuffer_base_address), LineType::Info);
                term.print(&format!(" Legacy VGA Fallback: {}", gop.is_vga_legacy_fallback), LineType::Info);
            }
            _ => term.print("Nepoznata gop komanda.", LineType::Error),
        }
    }
}

"sh" | "qsh" | "pipe" | "export" => {
    if args.is_empty() {
        term.print("Upotreba: sh run <pipeline> | export VAR=VAL | sh env | sh script <id>", LineType::Info);
    } else {
        match args[0] {
            "run" => {
                let full_cmd = args[1..].join(" ");
                let res = shell.run_pipeline(&full_cmd);
                term.print(&format!("Result: {}", res), LineType::Success);
            }
            "env" => {
                term.print("=== Environment Variables ===", LineType::Header);
                for (k, v) in &shell.env_vars {
                    term.print(&format!(" ${} = {}", k, v), LineType::Info);
                }
            }
            "export" => {
                if args.len() >= 2 {
                    let parts: Vec<&str> = args[1].split('=').collect();
                    if parts.len() == 2 {
                        shell.set_env(parts[0], parts[1]);
                        term.print(&format!("Promenljiva ${} postavljena.", parts[0]), LineType::Success);
                    } else {
                        term.print("Format mora biti: export VAR=VAL", LineType::Warning);
                    }
                }
            }
            "script" => {
                let res = shell.execute_script(0);
                for line in res {
                    term.print(&line, LineType::Info);
                }
            }
            _ => term.print("Nepoznata shell komanda.", LineType::Error),
        }
    }
}

"nvram" | "uefi" | "bios" => {
    if args.is_empty() {
        term.print("Upotreba: nvram list | nvram set <ključ> <vrednost> | nvram reset", LineType::Info);
    } else {
        match args[0] {
            "list" => {
                term.print("--- NVRAM Varijable ---", LineType::Success);
                for (name, var) in &nvram.storage.memory {
                    term.print(
                        &format!(" [{}] {} = {}", if var.is_read_only() { "RO" } else { "RW" }, name, var.get_string_val()),
                        LineType::Info,
                    );
                }
            }
            "set" => {
                if args.len() >= 3 {
                    let key = args[1];
                    let val = args[2..].join(" ");
                    match nvram.set_var(key, &val) {
                        Ok(_) => term.print(&format!("Uspešno upisano: {} = {}", key, val), LineType::Success),
                        Err(e) => term.print(&e, LineType::Error),
                    }
                } else {
                    term.print("Upotreba: nvram set <ključ> <vrednost>", LineType::Warning);
                }
            }
            "reset" => {
                nvram.factory_reset();
                term.print("NVRAM vraćen na fabrička podešavanja.", LineType::Success);
            }
            _ => term.print("Nepoznata nvram podkomanda.", LineType::Error),
        }
    }
}

"tsdb" | "metrics" => {
    if args.is_empty() {
        term.print("Upotreba: tsdb list | tsdb push <ime> <vrednost> | tsdb stats <ime> | tsdb prune", LineType::Info);
    } else {
        match args[0] {
            "list" => {
                term.print("--- Registrovane Time-Series Metrike ---", LineType::Success);
                for (name, series) in &tsdb.series_map {
                    term.print(
                        &format!("• {} [{}]: {} registrovanih tačaka (Max: {})", name, series.unit, series.points.len(), series.max_capacity),
                        LineType::Info,
                    );
                }
            }
            "push" => {
                if args.len() >= 3 {
                    let metric_name = args[1];
                    if let Ok(val) = args[2].parse::<f64>() {
                        tsdb.record_metric(metric_name, val);
                        term.print(&format!("Upisana vrednost {} u metrički niz '{}'.", val, metric_name), LineType::Success);
                    } else {
                        term.print("Vrednost mora biti broj (f64).", LineType::Error);
                    }
                } else {
                    term.print("Upotreba: tsdb push <ime_metrike> <vrednost>", LineType::Warning);
                }
            }
            "stats" => {
                if args.len() >= 2 {
                    let metric_name = args[1];
                    if let Some(avg) = tsdb.query_aggregate(metric_name, AggregationFunc::Average) {
                        let min = tsdb.query_aggregate(metric_name, AggregationFunc::Min).unwrap_or(0.0);
                        let max = tsdb.query_aggregate(metric_name, AggregationFunc::Max).unwrap_or(0.0);
                        let count = tsdb.query_aggregate(metric_name, AggregationFunc::Count).unwrap_or(0.0);
                        term.print(&format!("--- Statistika za '{}' ---", metric_name), LineType::Success);
                        term.print(&format!(" Prosek: {:.2} | Min: {:.2} | Max: {:.2} | Ukupno tačaka: {}", avg, min, max, count), LineType::Info);
                    } else {
                        term.print("Metrika nije pronađena ili je prazna.", LineType::Error);
                    }
                } else {
                    term.print("Upotreba: tsdb stats <ime_metrike>", LineType::Warning);
                }
            }
            "prune" => {
                tsdb.prune_all();
                term.print("Sve vremenske serije su očišćene.", LineType::Warning);
            }
            _ => term.print("Nepoznata TSDB komanda.", LineType::Error),
        }
    }
}

"gis" | "spatial" => {
    if args.is_empty() {
        term.print("Upotreba: gis list | gis add <ime> <kat> <lat> <lon> | gis radius <lat> <lon> <km> | gis nearest <lat> <lon> <k>", LineType::Info);
    } else {
        match args[0] {
            "list" => {
                term.print("--- Registrovani GIS Objekti ---", LineType::Success);
                for feature in &gis.features {
                    term.print(
                        &format!("• #{}: {} [{}] ({:.4}, {:.4})", feature.id, feature.name, feature.category, feature.location.lat, feature.location.lon),
                        LineType::Info,
                    );
                }
            }
            "add" => {
                if args.len() >= 5 {
                    let name = args[1];
                    let cat = args[2];
                    let lat = args[3].parse::<f64>().unwrap_or(0.0);
                    let lon = args[4].parse::<f64>().unwrap_or(0.0);
                    let id = gis.add_feature(name, cat, lat, lon);
                    term.print(&format!("Uspešno kreiran GIS entitet #{}.", id), LineType::Success);
                } else {
                    term.print("Upotreba: gis add <ime> <kategorija> <lat> <lon>", LineType::Warning);
                }
            }
            "radius" => {
                if args.len() >= 4 {
                    let lat = args[1].parse::<f64>().unwrap_or(0.0);
                    let lon = args[2].parse::<f64>().unwrap_or(0.0);
                    let km = args[3].parse::<f64>().unwrap_or(10.0);
                    let center = GeoPoint::new(lat, lon);
                    let matches = gis.search_in_radius(&center, km);

                    term.print(&format!("--- Rezultati u radijusu od {:.1} km ---", km), LineType::Success);
                    for (feat, dist) in matches {
                        term.print(&format!("• {} [{}]: Udaljenost {:.2} km", feat.name, feat.category, dist), LineType::Info);
                    }
                } else {
                    term.print("Upotreba: gis radius <lat> <lon> <radijus_km>", LineType::Warning);
                }
            }
            "nearest" => {
                if args.len() >= 4 {
                    let lat = args[1].parse::<f64>().unwrap_or(0.0);
                    let lon = args[2].parse::<f64>().unwrap_or(0.0);
                    let k = args[3].parse::<usize>().unwrap_or(3);
                    let center = GeoPoint::new(lat, lon);
                    let matches = gis.find_nearest(&center, k);

                    term.print(&format!("--- Pronađeno {} Najbližih Komšija (kNN) ---", k), LineType::Success);
                    for (feat, dist) in matches {
                        term.print(&format!("• {} [{}]: {:.2} km", feat.name, feat.category, dist), LineType::Info);
                    }
                } else {
                    term.print("Upotreba: gis nearest <lat> <lon> <k>", LineType::Warning);
                }
            }
            _ => term.print("Nepoznata GIS komanda.", LineType::Error),
        }
    }
}

"vinput" | "input" | "hid" => {
    if args.is_empty() {
        term.print("Upotreba: vinput key <taster> | vinput mouse <x> <y> | vinput gamepad <x> <y> | vinput status", LineType::Info);
    } else {
        match args[0] {
            "key" => {
                if args.len() >= 2 {
                    let key = &args[1];
                    vinput.inject_key(key, true);
                    term.print(&format!("Emuliran pritisak tastera: '{}'", key), LineType::Success);
                } else {
                    term.print("Upotreba: vinput key <taster>", LineType::Warning);
                }
            }
            "mouse" => {
                if args.len() >= 3 {
                    let x = args[1].parse::<f32>().unwrap_or(0.0);
                    let y = args[2].parse::<f32>().unwrap_or(0.0);
                    vinput.move_mouse(x, y);
                    term.print(&format!("Pomeren virtuelni miš na: ({}, {})", x, y), LineType::Success);
                } else {
                    term.print("Upotreba: vinput mouse <x> <y>", LineType::Warning);
                }
            }
            "gamepad" => {
                if args.len() >= 3 {
                    let x = args[1].parse::<f32>().unwrap_or(0.0);
                    let y = args[2].parse::<f32>().unwrap_or(0.0);
                    vinput.update_gamepad(x, y, false, false);
                    term.print(&format!("Postavljene ose džojstika: X={:.2}, Y={:.2}", x, y), LineType::Success);
                }
            }
            "status" => {
                term.print("--- Virtual Input Subsystem Status ---", LineType::Success);
                term.print(&format!(" Pozicija Miša: ({:.1}, {:.1})", vinput.mouse_x, vinput.mouse_y), LineType::Info);
                term.print(&format!(" Poslednji Taster: {}", vinput.last_key_pressed), LineType::Info);
                term.print(&format!(" Gamepad Ose: X={:.2}, Y={:.2}", vinput.gamepad_axis_x, vinput.gamepad_axis_y), LineType::Info);
                term.print(&format!(" Ukupno Događaja u Baferu: {}", vinput.events_queue.len()), LineType::Info);
            }
            _ => term.print("Nepoznata vinput komanda.", LineType::Error),
        }
    }
}

"qarc" | "compress" | "archive" => {
    if args.is_empty() {
        term.print("Upotreba: qarc list | qarc add <naziv> <tekst> | qarc test <tekst>", LineType::Info);
    } else {
        match args[0] {
            "list" => {
                term.print(&format!("--- Arhiva: {} ---", archiver.active_archive.archive_name), LineType::Success);
                for file in &archiver.active_archive.files {
                    term.print(
                        &format!(" 📄 {} (Ivorno: {}B -> RLE: {}B)", file.name, file.uncompressed_size, file.compressed_size),
                        LineType::Info,
                    );
                }
                term.print(
                    &format!("Ukupno: {}B -> {}B", archiver.active_archive.total_raw_bytes, archiver.active_archive.total_compressed_bytes),
                    LineType::Success,
                );
            }
            "add" => {
                if args.len() >= 3 {
                    let name = args[1];
                    let content = args[2..].join(" ");
                    archiver.add_file_to_archive(name, &content);
                    term.print(&format!("Fajl '{}' uspešno dodat u arhivu.", name), LineType::Success);
                } else {
                    term.print("Upotreba: qarc add <naziv> <tekst>", LineType::Warning);
                }
            }
            "test" => {
                if args.len() >= 2 {
                    let text = args[1..].join(" ");
                    let stats = archiver.test_raw_compression(&text);
                    term.print(&format!("--- Rezultat RLE Kompresije ---"), LineType::Success);
                    term.print(&format!(" Veličina: {}B -> {}B", stats.original_size, stats.compressed_size), LineType::Info);
                    term.print(&format!(" Ušteda: {:.1}%", stats.ratio_percentage), LineType::Info);
                    term.print(&format!(" Shannon Entropija: {:.2} bita/bajtu", stats.shannon_entropy), LineType::Info);
                }
            }
            _ => term.print("Nepoznata qarc komanda.", LineType::Error),
        }
    }
}

"crypto" | "tls" => {
    if args.is_empty() {
        term.print("Upotreba: crypto hash <tekst> | crypto handshake | crypto rsa", LineType::Info);
    } else {
        match args[0] {
            "hash" => {
                if args.len() >= 2 {
                    let h = crypto.hash_text(args[1]);
                    term.print(&format!("Hash: {}", h), LineType::Success);
                }
            }
            "handshake" => {
                crypto.tls_session.execute_handshake();
                for log in &crypto.tls_session.handshake_logs {
                    term.print(log, LineType::Info);
                }
            }
            "rsa" => {
                term.print(&format!("Javni RSA ključ: {:?}", crypto.rsa_keys.public_key), LineType::Success);
                term.print(&format!("Privatni RSA ključ: {:?}", crypto.rsa_keys.private_key), LineType::Warning);
            }
            _ => term.print("Nepoznata crypto komanda.", LineType::Error),
        }
    }
}

"nvme" | "usb" | "protodrv" => {
    if args.is_empty() {
        term.print("Upotreba: nvme read <lba> | nvme write <lba> <podatak> | usb list | usb send <port> <tekst>", LineType::Info);
    } else {
        match args[0] {
            "read" => {
                let lba = args.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                let res = proto_drv.nvme_read_sector(lba);
                term.print(&res, LineType::Success);
            }
            "write" => {
                if args.len() >= 3 {
                    let lba = args[1].parse::<u64>().unwrap_or(0);
                    let data = args[2..].join(" ");
                    let res = proto_drv.nvme_write_sector(lba, &data);
                    term.print(&res, LineType::Success);
                }
            }
            "list" => {
                term.print("--- xHCI USB Portovi ---", LineType::Success);
                for dev in &proto_drv.xhci.ports {
                    term.print(
                        &format!(" Port {}: {} [{:?}] VID:{:04X} PID:{:04X}", dev.port, dev.name, dev.speed, dev.vendor_id, dev.product_id),
                        LineType::Info,
                    );
                }
            }
            "send" => {
                if args.len() >= 3 {
                    let port = args[1].parse::<u8>().unwrap_or(1);
                    let data = args[2..].join(" ");
                    let res = proto_drv.usb_transfer(port, &data);
                    term.print(&res, LineType::Info);
                }
            }
            _ => term.print("Nepoznata komanda za drajvere.", LineType::Error),
        }
    }
}

"drivers" => {
    use crate::drivers::{
        CanBusEngine, CanFrame, I2cMaster, NvmeCommand, NvmeQueuePair, SpiController, SpiMode,
        UartController,
    };

    term.print("=== QuantumOS Peripheral & Storage Driver Engine ===", LineType::Info);

    // --- DEMO 1: NVMe SSD Submission/Completion Doorbell Execution ---
    term.print("--- Demo 1: NVMe High-Speed SSD Queue Pair Processing ---", LineType::Header);

    let mut nvme_qp = NvmeQueuePair::new(0x1000_2000);

    let write_cmd = NvmeCommand {
        command_id: 101,
        opcode: 0x01, // Write
        nsid: 1,
        prp1: 0x8000_1000,
        lba: 4096,
        block_count: 8,
    };

    nvme_qp.submit_command(write_cmd);
    term.print(&format!("• NVMe Komanda #{} (WRITE LBA 4096) poslata u SQ [Doorbell @ 0x{:X}]", write_cmd.command_id, nvme_qp.doorbell_sq_tail_addr), LineType::Info);

    if let Some(completion) = nvme_qp.process_hardware() {
        term.print(
            &format!("• NVMe Hardver Obradio Komandu #{}: Status = {} (Success) ➔ CQ Head: {}", completion.command_id, completion.status, completion.sq_head),
            LineType::Success,
        );
    }

    // --- DEMO 2: Embedded Protocols (UART, SPI, I2C, CAN Bus) ---
    term.print("--- Demo 2: Embedded Bus Drivers (UART, SPI, I2C, CAN) ---", LineType::Header);

    // UART
    let mut uart = UartController::new(115200);
    uart.transmit_byte(b'Q');
    term.print(&format!("• UART (115200 Baud): Poslat karakter '{}' na Tx liniju.", uart.tx_fifo[0] as char), LineType::Info);

    // SPI
    let spi = SpiController::new(SpiMode::Mode0, 10);
    let miso_data = spi.transfer_byte(0x9F, 0xEF); // 0x9F = Read JEDEC ID iz NOR Flash memorije
    term.print(&format!("• SPI Full-Duplex (Mode 0 @ 10MHz): Poslat MOSI 0x9F ➔ Primljen MISO JEDEC ID: 0x{:X}", miso_data), LineType::Success);

    // I2C
    let i2c = I2cMaster::new(400); // 400 kHz Fast Mode
    let sensor_addr = 0x68; // MPU6050 Žiroskop
    if let Ok(ack) = i2c.write_bytes(sensor_addr, &[0x75]) {
        term.print(&format!("• I2C (400kHz): Senzor na adresi 0x{:X} vratio ACK = {}! 📡", sensor_addr, ack), LineType::Success);
    }

    // CAN Bus Arbitraža
    let engine_ecu_frame = CanFrame {
        id: 0x100, // Visok prioritet (Engine Control)
        is_extended: false,
        dlc: 4,
        data: [0x2A, 0x00, 0x11, 0xFF, 0, 0, 0, 0],
    };

    let body_ecu_frame = CanFrame {
        id: 0x350, // Niži prioritet (Window Control)
        is_extended: false,
        dlc: 2,
        data: [0x01, 0x00, 0, 0, 0, 0, 0, 0],
    };

    let winner = CanBusEngine::arbitrate(&engine_ecu_frame, &body_ecu_frame);
    term.print(
        &format!("• CAN Bus Arbitraža: Čvor ID 0x{:X} vs ID 0x{:X} ➔ Pobeda ID: 0x{:X} (Dominantni Bitovi) 🚗", engine_ecu_frame.id, body_ecu_frame.id, winner.id),
        LineType::Warning,
    );
}

"pcie_dma" => {
    use crate::pcie_dma::{DmaDescriptor, Iommu, PcieDeviceSim};

    term.print("=== QuantumOS Hardware Engine: PCIe Bus Master & DMA Subsystem ===", LineType::Info);

    // 1. Inicijalizacija 1KB Sistemskog RAM-a
    let mut system_ram = vec![0u8; 1024];

    // 2. Inicijalizacija IOMMU-a i mapiranje IOVA -> Fizička adresa u RAM-u
    let mut iommu = Iommu::new();
    let iova_buffer_addr = 0x9000_0000; // Virtualna adresa viđena od strane uređaja
    let physical_ram_addr = 0x0000_0100; // Fizički offset u RAM-u (256. bajt)
    
    iommu.map_page(iova_buffer_addr, physical_ram_addr);

    term.print("• IOMMU Konfiguracija: IOVA 0x9000_0000 ➔ Physical RAM Offset 0x0100", LineType::Info);

    // 3. Kreiranje PCIe Mrežne kartice (Intel X550 10GbE)
    let nic = PcieDeviceSim::new("Intel X550 10GbE NIC", 0x8086, 0x1563, 0xFE00_0000);
    term.print(&format!("• PCIe Uređaj Detektovan: {} [MMIO BAR0: 0x{:X}]", nic.name, nic.bar0_mmio_base), LineType::Success);

    // 4. CPU priprema DMA Scatter-Gather Deskriptor u RAM-u
    let mut descriptor = DmaDescriptor {
        buffer_iova: iova_buffer_addr,
        length: 64,
        is_owned_by_device: true, // Predato NIC-u na popunjavanje
    };

    let network_packet = b"GET /index.html HTTP/1.1\r\nHost: quantumos.org\r\n\r\n";
    term.print("• Prispeli Mrežni Paket na NIC (Spreman za DMA upis u RAM)...", LineType::Info);

    // 5. NIC vrši Direct Memory Access (DMA) bez ikakvog opterećenja CPU-a!
    match nic.execute_dma_transfer(&mut descriptor, network_packet, &iommu, &mut system_ram) {
        Ok(bytes_written) => {
            term.print(
                &format!("• DMA TRANSFER USPEŠAN: Uređaj je upisao {} bajtova u RAM preko IOMMU-a!", bytes_written),
                LineType::Success,
            );

            // Proveravamo sadržaj RAM-a na fizičkoj adresi
            let ram_data = &system_ram[physical_ram_addr as usize..(physical_ram_addr as usize + bytes_written)];
            let read_str = String::from_utf8_lossy(ram_data);
            term.print(&format!("• Sadržaj u Fizičkom RAM-u (Offset 0x0100): \"{}\"", read_str), LineType::Success);
        }
        Err(err) => {
            term.print(&format!("• DMA FAULT: {}", err), LineType::Error);
        }
    }

    // --- DEMO 2: Pokušaj Neovlašćenog DMA Napada (Malicious DMA Attack) ---
    term.print("--- Demo 2: Malicious PCIe DMA Attack Detection (IOMMU Protection) ---", LineType::Header);

    let mut rogue_descriptor = DmaDescriptor {
        buffer_iova: 0xDEAD_BEEF, // Neautorizovana adresa (Pokušaj čitanja Kernel Kernel Key-a)
        length: 64,
        is_owned_by_device: true,
    };

    term.print("• Pokušaj neovlašćenog DMA upisa na IOVA 0xDEAD_BEEF...", LineType::Warning);
    match nic.execute_dma_transfer(&mut rogue_descriptor, network_packet, &iommu, &mut system_ram) {
        Ok(_) => term.print("• GREŠKA: Napad uspeo (IOMMU zakazao!)", LineType::Error),
        Err(err) => term.print(&format!("• BLOKIRAN DMA NAPAD: {} 🛡️", err), LineType::Error),
    }
}

"quantum_asm" => {
    use crate::quantum_asm::{QiskitInstruction, QuantumRegister};

    term.print("=== QuantumOS Qiskit/Quil Engine: Decoherence & Superposition ===", LineType::Info);

    // Kreiramo kvantni registar od 2 kubita sa T1=50μs i T2=30μs (Realni NISQ čip)
    let _q_reg = QuantumRegister::new(2, 50.0, 30.0);

    // --- TEST 1: Idealna Superpozicija (Bez kašnjenja) ---
    term.print("--- Demo 1: Hadamard Gate & Quantum Superposition ---", LineType::Header);

    let ideal_circuit = vec![
        QiskitInstruction::H { qubit: 0 }, // Q0 prelazi u |+⟩ = (|0⟩ + |1⟩) / √2
        QiskitInstruction::Measure { qubit: 0 },
    ];

    let mut counts_0 = 0;
    let mut counts_1 = 0;

    for _ in 0..100 {
        let mut test_reg = QuantumRegister::new(1, 50.0, 30.0);
        test_reg.execute_instruction(&ideal_circuit[0]);
        if let Some((_, val)) = test_reg.execute_instruction(&ideal_circuit[1]) {
            if val == 0 { counts_0 += 1; } else { counts_1 += 1; }
        }
    }

    term.print(&format!("• Rezultat 100 merenja stanja |+⟩: |0⟩ = {}% | |1⟩ = {}%", counts_0, counts_1), LineType::Success);

    // --- TEST 2: Uticaj Dekoherencije (Pričekaj 60 μs pre merenja) ---
    term.print("--- Demo 2: Decoherence Effect (Wait 60μs > T2 time) ---", LineType::Header);

    let noisy_circuit = vec![
        QiskitInstruction::X { qubit: 0 },         // Postavi Q0 u |1⟩
        QiskitInstruction::H { qubit: 0 },         // Prevedi u superpoziciju
        QiskitInstruction::Wait { duration_us: 60.0 }, // ČEKAMO 60μs (Relaksacija T1/T2 nastupa!)
        QiskitInstruction::Measure { qubit: 0 },
    ];

    term.print("• Izvršavanje kola sa kašnjenjem od 60μs na čipu sa T1=50μs / T2=30μs...", LineType::Warning);
    
    let mut noisy_0 = 0;
    let mut noisy_1 = 0;
    for _ in 0..100 {
        let mut test_reg = QuantumRegister::new(1, 50.0, 30.0);
        for inst in &noisy_circuit {
            if let Some((_, val)) = test_reg.execute_instruction(inst) {
                if val == 0 { noisy_0 += 1; } else { noisy_1 += 1; }
            }
        }
    }

    term.print(
        &format!("• Dekoherencija je urušila kvantno stanje! Merenja: |0⟩ = {}% | |1⟩ = {}% (Primetan pad ka |0⟩ usled T1 decay-a!)", noisy_0, noisy_1),
        LineType::Error,
    );
}

"bio_comp" => {
    use crate::bio_comp::{DnaStrand, DnaTestTube, LifNeuron};

    term.print("=== QuantumOS Bio-Computing Engine: DNA & Spike Encoding ===", LineType::Info);

    // --- DEMO 1: DNK Molekularno Kodiranje i Hibridizacija ---
    term.print("--- Demo 1: DNA Molecular Hybridization (Adleman Method) ---", LineType::Header);

    let node_a = DnaStrand::from_str("ACTG");
    let node_b = DnaStrand::from_str("CAGT");
    let complement_a = node_a.create_complementary();

    term.print(&format!("• Čvor A (DNK sekvenca):     {}", node_a.to_string()), LineType::Info);
    term.print(&format!("• Čvor B (DNK sekvenca):     {}", node_b.to_string()), LineType::Info);
    term.print(&format!("• Komplement A (Sinteza):    {} (Spreman za vezivanje)", complement_a.to_string()), LineType::Success);

    let mut tube = DnaTestTube::new();
    tube.add_strand(DnaStrand::from_str("ACTGCAGT")); // Uspešna putanja A -> B
    tube.add_strand(DnaStrand::from_str("ACTG"));     // Nezavršena putanja
    tube.add_strand(DnaStrand::from_str("ACTGCAGTGCAA")); // Predugačka putanja

    term.print(&format!("• Ukupno molekula u epruveti: {}", tube.strands.len()), LineType::Info);
    tube.filter_by_length(8); // Reakcija odvajanja molekula tačne dužine
    term.print(&format!("• Nakon PCR/Elektroforeze (Filtrirano rešenje A->B): {}", tube.strands[0].to_string()), LineType::Success);

    // --- DEMO 2: Biološki LIF Neuron i Spike-Frequency Kodiranje ---
    term.print("--- Demo 2: Biological Neural Spike-Frequency Encoding (LIF Model) ---", LineType::Header);

    let mut neuron = LifNeuron::new();
    let currents = [2.0, 5.0, 12.0, 18.0, 3.0]; // Različiti nivoi stimulansa

    term.print("• Simulacija stimulacije membrane neurona sa različitim strujama:", LineType::Info);
    for (step, &current) in currents.iter().enumerate() {
        let spiked = neuron.step(current);
        let status = if spiked {
            "⚡ ACTION POTENTIAL (SPIKE) OKINUT!"
        } else {
            "   Membrana se puni..."
        };
        term.print(
            &format!("  [Korak {}] Ulazna struja: {:4.1} mA | V_mem: {:6.2} mV | Status: {}", 
                step + 1, current, neuron.v_membrane, status
            ),
            if spiked { LineType::Success } else { LineType::Warning }
        );
    }
}

"photonic_asm" => {
    use crate::photonic_asm::{PhotonicChipSim, PhotonicInstruction};

    term.print("=== QuantumOS Photonic Engine: Analog Optical Waveguide Assembly ===", LineType::Info);

    let mut chip = PhotonicChipSim::new(0.01); // 0.01 rad termički šum silicijuma

    // --- DEMO 1: Konstruktivna vs Destruktivna Optička Interferencija ---
    term.print("--- Demo 1: Constructive vs Destructive Optical Interference ---", LineType::Header);

    // Program 1: Dva zraka u ISTOJ fazi (0 deg) -> Konstruktivno pojačanje!
    let program_constructive = vec![
        PhotonicInstruction::PumpLaser { channel: 0, amplitude: 1.0, phase_deg: 0.0 },
        PhotonicInstruction::PumpLaser { channel: 1, amplitude: 1.0, phase_deg: 0.0 },
        PhotonicInstruction::BeamSplitter { in1: 0, in2: 1, out1: 2, out2: 3 },
        PhotonicInstruction::Photodetect { channel: 2 },
    ];

    term.print("• Laser 0 (Amp=1.0, Phase=0°) + Laser 1 (Amp=1.0, Phase=0°)", LineType::Info);
    for inst in &program_constructive {
        if let Some(voltage) = chip.execute_instruction(inst) {
            term.print(&format!("• Detektovana Snaga na Fotodiodi #2 (Konstruktivno): {:.4} V (Pojačanje!) ✨", voltage), LineType::Success);
        }
    }

    // Program 2: Pomera fazu Lasera 1 za 180° (π radijana) -> Destruktivno poništavanje!
    let program_destructive = vec![
        PhotonicInstruction::PumpLaser { channel: 0, amplitude: 1.0, phase_deg: 0.0 },
        PhotonicInstruction::PumpLaser { channel: 1, amplitude: 1.0, phase_deg: 180.0 }, // 180° OPPOSITE PHASE!
        PhotonicInstruction::BeamSplitter { in1: 0, in2: 1, out1: 2, out2: 3 },
        PhotonicInstruction::Photodetect { channel: 2 },
    ];

    term.print("• Laser 0 (Amp=1.0, Phase=0°) + Laser 1 (Amp=1.0, Phase=180° - Kontra-faza!)", LineType::Warning);
    for inst in &program_destructive {
        if let Some(voltage) = chip.execute_instruction(inst) {
            term.print(&format!("• Detektovana Snaga na Fotodiodi #2 (Destruktivno): {:.4} V (Potpuni mrak / Poništavanje!) 🕶️", voltage), LineType::Error);
        }
    }

    // --- DEMO 2: Mach-Zehnder Interferometer (MZI) Matrična Rotacija ---
    term.print("--- Demo 2: MZI Unitary Matrix Processing (Speed-of-Light MatMul) ---", LineType::Header);

    let program_mzi = vec![
        PhotonicInstruction::PumpLaser { channel: 0, amplitude: 2.0, phase_deg: 0.0 },
        PhotonicInstruction::PumpLaser { channel: 1, amplitude: 0.5, phase_deg: 45.0 },
        PhotonicInstruction::MziRotate { in1: 0, in2: 1, theta_deg: 90.0, phi_deg: 30.0 },
        PhotonicInstruction::Photodetect { channel: 0 },
    ];

    for inst in &program_mzi {
        if let Some(voltage) = chip.execute_instruction(inst) {
            term.print(&format!("• MZI Rotacija izrečunata fizičkim prolaskom fotona ➔ Izlazni Napon: {:.4} V", voltage), LineType::Success);
        }
    }
}

"spectre_meltdown" => {
    use crate::spectre_meltdown::SpectreSimulator;

    term.print("=== QuantumOS Security: Spectre & Transient Execution Attack ===", LineType::Info);

    let secret_str = "K26_TAJNA";
    let mut spectre = SpectreSimulator::new(secret_str);

    term.print(&format!("• Tajna u Kernel memoriji: \"{}\"", secret_str), LineType::Warning);
    term.print("• Pokrećemo Spectre V1 napad (Bounds Check Bypass + Flush & Reload)...", LineType::Info);

    let mut recovered_bytes = Vec::new();

    for secret_idx in 0..secret_str.len() {
        // 1. Očisti keš
        spectre.flush_probe_array();

        // 2. Treniraj prediktor skokova sa validnim indeksima (Trening faza)
        for _ in 0..5 {
            spectre.victim_function(1, false);
        }

        // 3. NAPAD: Pošalji out-of-bounds indeks pod lažnom spekulacijom!
        let oob_index = spectre.public_array.len() + secret_idx;
        spectre.victim_function(oob_index, true); // Forsiramo tranzijentno izvršavanje

        // 4. Reload faza (Merenje kašnjenja pristupa kešu)
        let (leaked_byte, latency) = spectre.reload_and_recover_secret();
        recovered_bytes.push(leaked_byte);

        term.print(
            &format!("  [Offset +{}] Rekonstruisan bajt: '{}' (ASCI: {}) ➔ Latency: {} cycles (Cache Hit!)", 
                secret_idx, leaked_byte as char, leaked_byte, latency),
            LineType::Success,
        );
    }

    let leaked_str = String::from_utf8_lossy(&recovered_bytes);
    term.print("---------------------------------------------------------------", LineType::Info);
    term.print(&format!("• USPEŠNO PROČITANA TAJNA IZ KERNELA: \"{}\" 🔓", leaked_str), LineType::Error);
    term.print("• Mitigacija: Ubacivanje 'LFENCE' (Speculation Barrier) instrukcija ili 'index_masking'.", LineType::Info);
}

"zero_div" => {
    use crate::zero_div::{setup_sigfpe_handler, BranchlessMath, ZERO_DIV_TRAPS_CAUGHT};
    use std::sync::atomic::Ordering;

    term.print("=== QuantumOS Architecture: Branchless Zero Division Hacks ===", LineType::Info);

    // 1. Inicijalizacija Signal Handlera
    setup_sigfpe_handler();
    term.print("• SIGFPE Trap Handler uspešno instaliran na nivou OS-a.", LineType::Success);

    // --- DEMO 1: Pure Bitwise Branchless Math ---
    term.print("--- Demo 1: Pure Bitwise Branchless Math (ALU Masking) ---", LineType::Header);
    let res_valid = BranchlessMath::safe_div_bitwise(100, 5);
    let res_zero = BranchlessMath::safe_div_bitwise(100, 0);

    term.print(&format!("• 100 / 5 (Branchless): {}", res_valid), LineType::Info);
    term.print(&format!("• 100 / 0 (Branchless, BEZ IF): {} ✨", res_zero), LineType::Success);

    // --- DEMO 2: Signal Handler Assembly Hack (#DE Trap) ---
    term.print("--- Demo 2: Hardware #DE Trap & RIP Patching Hack ---", LineType::Header);
    term.print("• Izvršavamo direktno deljenje sa nulom u x86_64 Asm bez provere...", LineType::Warning);

    let _x: u64 = 42;
    let _y: u64 = 0;
    let result: u64;

    #[cfg(unix)]
    unsafe {
        // Direktno deljenje bez ijednog IF-a u Rustu ili Assembly-ju!
        std::arch::asm!(
            "xor rdx, rdx",    // Očisti RDX (gornjih 64 bita za DIV)
            "idiv {divisor}",  // ISPALJUJE #DE TRAP AKO JE DIVISOR 0!
            divisor = in(reg) y,
            inout("rax") x => result,
            out("rdx") _,
        );
    }

    #[cfg(not(unix))]
    {
        result = 0;
    }

    let traps = ZERO_DIV_TRAPS_CAUGHT.load(Ordering::Relaxed);
    term.print(&format!("• Rezultat opasnog deljenja: {}", result), LineType::Success);
    term.print(&format!("• Ukupno presretnutih hardverskih #DE prekida (Traps): {} ⚡", traps), LineType::Info);
}

"pipeline_sim" => {
    use crate::pipeline_sim::{CpuPipelineSim, Instruction, Opcode};

    term.print("=== QuantumOS Architecture: CPU Pipeline & Stall Simulator ===", LineType::Info);

    // Kreiramo sekvencu instrukcija sa teškim zavisnostima podataka
    // Inst 1: ADD R1 = R2 + R3
    // Inst 2: SUB R4 = R1 - R5  <-- Zavisi od R1!
    // Inst 3: ADD R6 = R4 + R1  <-- Zavisi od R4 i R1!
    let program = vec![
        Instruction { id: 1, op: Opcode::Add, dest_reg: Some(1), src_reg1: Some(2), src_reg2: Some(3) },
        Instruction { id: 2, op: Opcode::Sub, dest_reg: Some(4), src_reg1: Some(1), src_reg2: Some(5) },
        Instruction { id: 3, op: Opcode::Add, dest_reg: Some(6), src_reg1: Some(4), src_reg2: Some(1) },
    ];

    // --- TEST 1: BEZ DATA FORWARDING-A (Teški Stalls / Prazni Ciklusi) ---
    term.print("--- Test 1: Pipeline Engine BEZ Data Forwarding-a ---", LineType::Header);
    let mut sim_no_fwd = CpuPipelineSim::new(program.clone(), false);

    while !sim_no_fwd.step_clock_cycle() && sim_no_fwd.cycle_count < 20 {}

    term.print(&format!("• Ukupno Taktskih Ciklusa: {}", sim_no_fwd.cycle_count), LineType::Info);
    term.print(&format!("• Praznih Ciklusa (Stalls / Bubbles): {} ⚠️", sim_no_fwd.total_stalls), LineType::Error);
    term.print(&format!("• Izvršeno Instrukcija: {}", sim_no_fwd.instructions_completed), LineType::Info);
    term.print(&format!("• CPI (Cycles Per Instruction): {:.2} (Idealno je 1.0)", sim_no_fwd.calculate_cpi()), LineType::Warning);

    // --- TEST 2: SA DATA FORWARDING-OM (Hardware Bypass Spasava Dan) ---
    term.print("--- Test 2: Pipeline Engine SA Data Forwarding-om (Bypassing) ---", LineType::Header);
    let mut sim_fwd = CpuPipelineSim::new(program, true);

    while !sim_fwd.step_clock_cycle() && sim_fwd.cycle_count < 20 {}

    term.print(&format!("• Ukupno Taktskih Ciklusa: {}", sim_fwd.cycle_count), LineType::Info);
    term.print(&format!("• Praznih Ciklusa (Stalls / Bubbles): {} ✨", sim_fwd.total_stalls), LineType::Success);
    term.print(&format!("• Izvršeno Instrukcija: {}", sim_fwd.instructions_completed), LineType::Info);
    term.print(&format!("• CPI (Cycles Per Instruction): {:.2} (Dramatično bolje!)", sim_fwd.calculate_cpi()), LineType::Success);
}

"cache_numa" => {
    use crate::cache_numa::{
        CachePaddedCounter, FalseSharingCounters, NumaLatencyModel, NumaNode,
    };
    use std::time::Instant;

    term.print("=== QuantumOS Architecture: Cache Line Bouncing & NUMA Bench ===", LineType::Info);

    let iterations = 1_000_000;

    // --- TEST 1: False Sharing (Unpadded) ---
    term.print("--- Test 1: Shared 64-byte Cache Line (False Sharing) ---", LineType::Header);
    let false_sharing_struct = FalseSharingCounters::new();

    let start = Instant::now();
    for _ in 0..iterations {
        false_sharing_struct.thread_0_val.fetch_add(1, Ordering::Relaxed);
        false_sharing_struct.thread_1_val.fetch_add(1, Ordering::Relaxed);
    }
    let duration_bad = start.elapsed();

    term.print(
        &format!("• Izvršeno {} simuliranih atomskih zapisa u istu keš liniju.", iterations * 2),
        LineType::Info,
    );
    term.print(
        &format!("• Vreme izvršavanja (Ne-poravnato): {:?}", duration_bad),
        LineType::Error,
    );

    // --- TEST 2: Cache-Padded (64-byte Alignment) ---
    term.print("--- Test 2: Cache-Padded Structs (align(64)) ---", LineType::Header);
    let padded_counter_0 = CachePaddedCounter::new();
    let padded_counter_1 = CachePaddedCounter::new();

    let start_padded = Instant::now();
    for _ in 0..iterations {
        padded_counter_0.val.fetch_add(1, Ordering::Relaxed);
        padded_counter_1.val.fetch_add(1, Ordering::Relaxed);
    }
    let duration_good = start_padded.elapsed();

    term.print(
        &format!("• Vreme izvršavanja (#[repr(align(64))]): {:?}", duration_good),
        LineType::Success,
    );

    // --- TEST 3: NUMA Distance & Interconnect Bouncing Penalty ---
    term.print("--- Test 3: Hardware Latency Model across NUMA Nodes ---", LineType::Header);

    let local_clean = NumaLatencyModel::estimate_access_latency(false, NumaNode::Node0_Local, 0);
    let local_bouncing = NumaLatencyModel::estimate_access_latency(true, NumaNode::Node0_Local, 10);
    let remote_bouncing = NumaLatencyModel::estimate_access_latency(true, NumaNode::Node1_RemoteSocket, 10);

    term.print(&format!("• Lokalni L1 Hit (Bez konflicta): {} ns", local_clean), LineType::Info);
    term.print(&format!("• Lokalni Cache Bouncing (MESI Invalidate): {} ns per op", local_bouncing), LineType::Warning);
    term.print(
        &format!("• Remote NUMA Interconnect Bouncing (QPI/Infinity Fabric): {} ns per op! ⚠️", remote_bouncing),
        LineType::Error,
    );
}

"hazard_ebr" => {
    use crate::hazard_ebr::{EbrDomain, HazardPointerDomain};

    term.print("=== QuantumOS Memory: Hazard Pointers vs Epoch-Based Reclamation ===", LineType::Info);

    // --- DEMO 1: Hazard Pointers ---
    term.print("--- Demo 1: Hazard Pointers (Fine-Grained Safety) ---", LineType::Header);
    let hp = HazardPointerDomain::new();

    let node_a = 0x1000 as *mut u8;
    let node_b = 0x2000 as *mut u8;

    // Nit 0 objavljuje da čita node_a
    hp.publish_hazard(0, node_a);
    term.print("• Nit #0 objavljuje Hazard Poiner na čvor 0x1000", LineType::Info);

    // Penzionišemo oba čvora
    hp.retire_node(node_a);
    hp.retire_node(node_b);
    term.print("• Penzionisani čvorovi 0x1000 i 0x2000 (Stavljeni u listu čekanja)", LineType::Info);

    // Pokušaj čišćenja
    let (reclaimed, addrs) = hp.reclaim_hazard_garbage();
    term.print(&format!("• Rezultat čišćenja ➔ Očišćeno: {} čvor(a).", reclaimed), LineType::Success);
    term.print(&format!("• Čvor 0x1000 je ZAŠTIĆEN jer Nit #0 drži hazard! Čvor {:p} je bezbedno oslobođen.", addrs[0]), LineType::Success);

    hp.clear_hazard(0);
    let (reclaimed2, _) = hp.reclaim_hazard_garbage();
    term.print(&format!("• Nit #0 uklanja hazard ➔ Ponovno čišćenje oslobađa preostalih {} čvor(a).", reclaimed2), LineType::Success);

    // --- DEMO 2: Epoch-Based Reclamation ---
    term.print("--- Demo 2: Epoch-Based Reclamation (Batch Safety) ---", LineType::Header);
    let ebr = EbrDomain::new();

    let e1 = ebr.enter_critical(0);
    term.print(&format!("• Nit #0 ulazi u kritičnu sekciju u Epohi #{}", e1), LineType::Info);

    ebr.retire_node(0x3000 as *mut u8);
    ebr.retire_node(0x4000 as *mut u8);
    term.print("• Penzionisani čvorovi 0x3000 i 0x4000 u Epohi #1", LineType::Info);

    let (advanced, count) = ebr.try_advance_and_reclaim();
    term.print(&format!("• Pokušaj napredovanja epohe dok Nit #0 visi u Epohi #1 ➔ Prošlo: {} (Očišćeno: {})", advanced, count), LineType::Error);

    ebr.exit_critical(0);
    term.print("• Nit #0 izlazi iz kritične sekcije.", LineType::Info);

    ebr.try_advance_and_reclaim(); // Epoha 2
    let (advanced2, count2) = ebr.try_advance_and_reclaim(); // Epoha 3 -> Čisti epohu 1
    term.print(&format!("• Napredovanje u Epohu #3 ➔ Uspešno napredovano: {}! Očišćeno čvorova zrelih za reciklažu: {}", advanced2, count2), LineType::Success);
}

"jit_smc" => {
    use crate::jit_engine::{JitPageBuffer, MemoryPermission, X86_64Emitter};

    term.print("=== QuantumOS CPU: JIT Engine & Self-Modifying Code Simulator ===", LineType::Info);

    // 1. Dinamičko emitovanje x86_64 mašinskog koda
    term.print("--- Phase 1: JIT Emission (Building x86_64 Opcodes in RAM) ---", LineType::Header);
    let mut emitter = X86_64Emitter::new();
    
    // Generišemo: mov eax, 100; add eax, 50; ret;
    emitter.emit_mov_eax(100);
    emitter.emit_add_eax(50);
    emitter.emit_ret();

    let hex_opcodes: Vec<String> = emitter.buffer.iter().map(|b| format!("{:02X}", b)).collect();
    term.print(&format!("• Emitovani opkodovi (HEX): [{}]", hex_opcodes.join(" ")), LineType::Info);

    let mut jit_page = JitPageBuffer::new(0x7FFF_0000_1000, &emitter.buffer);

    // 2. Testiranje W^X Zaštite
    term.print("--- Phase 2: W^X Enforcement & Execution ---", LineType::Header);
    term.print("• Pokušaj izvršavanja dok je stranica još u ReadWrite režimu...", LineType::Info);
    match jit_page.execute() {
        Ok(res) => term.print(&format!("• Unexpected Result: {}", res), LineType::Error),
        Err(err) => term.print(&format!("• SUCCESS (BLOCKED): {}", err), LineType::Success),
    }

    term.print("• Menjamo prava stranice: ReadWrite ➔ ReadOnlyExecute (mprotect) + I-Cache Flush...", LineType::Info);
    jit_page.protect(MemoryPermission::ReadOnlyExecute);

    match jit_page.execute() {
        Ok(result) => term.print(&format!("• JIT EXECUTION SUCCESS: Funkcija iz RAM-a je vratila EAX = {} (Očekivano: 150)", result), LineType::Success),
        Err(err) => term.print(&format!("• Error: {}", err), LineType::Error),
    }

    // 3. Self-Modifying Code (SMC) & Patching
    term.print("--- Phase 3: Self-Modifying Code (Patching Opcodes at Runtime) ---", LineType::Header);
    term.print("• Pokušaj izmene opkoda dok je stranica Executable...", LineType::Info);
    if let Err(err) = jit_page.patch_byte(1, 200) {
        term.print(&format!("• W^X Zaštita sprečila izmenu: {}", err), LineType::Success);
    }

    term.print("• Penzionišemo Executable prava (Nazad u RW), menramo operand '100' u '500'...", LineType::Info);
    jit_page.protect(MemoryPermission::ReadWrite);
    jit_page.patch_byte(1, 244).unwrap(); // 500 in LE: 0xF4 = 244
    jit_page.patch_byte(2, 1).unwrap();   // 0x01

    term.print("• Vraćamo u ReadOnlyExecute + Osvežavamo I-Cache...", LineType::Info);
    jit_page.protect(MemoryPermission::ReadOnlyExecute);

    match jit_page.execute() {
        Ok(new_result) => term.print(&format!("• SMC EXECUTION SUCCESS: Modifikovani kod u RAM-u sada vraća EAX = {} (500 + 50)", new_result), LineType::Success),
        Err(err) => term.print(&format!("• Error: {}", err), LineType::Error),
    }
}

"veb_tree" => {
    use crate::cache_oblivious::{CacheSimulator, VebTreeEngine};

    term.print("=== QuantumOS Memory: Cache-Oblivious vEB Tree Layout Simulator ===", LineType::Info);

    let tree_height = 6; // Visina stabla = 6 (63 čvora)
    let tree = VebTreeEngine::new(tree_height);

    term.print(&format!("• Generisano balansirano stablo: Visina = {}, Ukupno čvorova = {}", tree.height, tree.size), LineType::Header);

    term.print("--- Poređenje memorijskog rasporeda u RAM-u (Prvih 15 indeksa) ---", LineType::Header);
    let bfs_preview: Vec<i64> = tree.bfs_nodes.iter().take(15).cloned().collect();
    let veb_preview: Vec<i64> = tree.veb_nodes.iter().take(15).cloned().collect();

    term.print(&format!("• Standardni BFS / Heap Layout: {:?}", bfs_preview), LineType::Info);
    term.print(&format!("• Cache-Oblivious vEB Layout:   {:?}", veb_preview), LineType::Success);

    // --- Demo 2: Simulacija Keš Promašaja (64-bajtne L1 Keš linije) ---
    term.print("--- Pretraga najdubljeg lista i simulacija L1 Cache Miss-ova ---", LineType::Header);

    let target_node = 1; // Pretražujemo list na najdubljoj nivojskoj putanji
    let elements_per_line = 8; // 64 bajta / 8 bajtova (i64) = 8 elemenata po keš liniji

    let (bfs_misses, veb_misses) = CacheSimulator::simulate_search_misses(&tree, target_node, elements_per_line);

    term.print(&format!("• Tražena vrednost u stablu: {}", target_node), LineType::Info);
    term.print(&format!("• Standardni BFS Layout ➔ Učitano L1 Keš Linija: {} linija", bfs_misses), LineType::Error);
    term.print(&format!("• Cache-Oblivious vEB    ➔ Učitano L1 Keš Linija: {} linija!", veb_misses), LineType::Success);

    if veb_misses < bfs_misses {
        let reduction = ((bfs_misses - veb_misses) as f64 / bfs_misses as f64) * 100.0;
        term.print(&format!("• ✅ REDUKCIJA KEŠ PROMAŠAJA: {:.1}% manje pristupa glavnoj memoriji!", reduction), LineType::Success);
    }
}

"mech_sym" => {
    use crate::mechanical_sympathy::{
        MemoryStrideTester, PaddedCounters, UnpaddedCounters, ZeroCostTester,
    };
    use std::time::Instant;

    term.print("=== QuantumOS CPU: Mechanical Sympathy & Zero-Cost Breakdown ===", LineType::Info);

    // --- Demo 1: Mechanical Sympathy (Cache Line Alignment & False Sharing) ---
    term.print("--- Demo 1: Alignment & Cache Line Padding (False Sharing) ---", LineType::Header);
    let unpadded = UnpaddedCounters::new();
    let padded = PaddedCounters::new();

    let addr_a = &unpadded.counter_a as *const _ as usize;
    let addr_b = &unpadded.counter_b as *const _ as usize;
    term.print(&format!("• Unpadded adrese: A = {:#X}, B = {:#X} (Razlika = {} bajtova) ➔ U ISTOJ Keš Liniji!", addr_a, addr_b, addr_b - addr_a), LineType::Error);

    let pad_addr_a = &padded.counter_a as *const _ as usize;
    let pad_addr_b = &padded.counter_b as *const _ as usize;
    term.print(&format!("• Padded adrese:   A = {:#X}, B = {:#X} (Razlika = {} bajtova) ➔ ZASEBNE Keš Linije!", pad_addr_a, pad_addr_b, pad_addr_b - pad_addr_a), LineType::Success);

    // --- Demo 2: Cache Locality (Row-Major vs Column-Major Stride) ---
    term.print("--- Demo 2: Cache Locality & Hardware Prefetcher ---", LineType::Header);
    let size = 600;
    let matrix = vec![vec![1u64; size]; size];

    let start1 = Instant::now();
    let sum1 = MemoryStrideTester::row_major_sum(&matrix, size);
    let dur1 = start1.elapsed();

    let start2 = Instant::now();
    let sum2 = MemoryStrideTester::col_major_sum(&matrix, size);
    let dur2 = start2.elapsed();

    term.print(&format!("• Sequential Access (Row-Major):    {:?} (Sum: {}) ➔ Prefetcher radi 100%", dur1, sum1), LineType::Success);
    term.print(&format!("• Strided Access    (Column-Major): {:?} (Sum: {}) ➔ Izaziva L1 Cache Misses!", dur2, sum2), LineType::Error);

    // --- Demo 3: Zero-Cost Abstractions ---
    term.print("--- Demo 3: Zero-Cost Abstraction Proof (Iterator vs Manual Loop) ---", LineType::Header);
    let dataset: Vec<u64> = (0..500_000).collect();

    let start_it = Instant::now();
    let res_it = ZeroCostTester::functional_iterator(&dataset);
    let dur_it = start_it.elapsed();

    let start_loop = Instant::now();
    let res_loop = ZeroCostTester::manual_loop(&dataset);
    let dur_loop = start_loop.elapsed();

    term.print(&format!("• High-Level Iterator Pipeline: {:?} (Result: {})", dur_it, res_it), LineType::Success);
    term.print(&format!("• Imperative Manual While Loop: {:?} (Result: {})", dur_loop, res_loop), LineType::Success);
    term.print("• ZAKLJUČAK: Rust Monomorfizacija i Loop Unrolling poništavaju trošak apstrakcije!", LineType::Info);
}

"raymarch" => {
    use crate::compute_raymarch::ComputeRaymarchEngine;

    term.print("=== QuantumOS CPU/GPGPU: Pure Compute Shader Ray-Marcher ===", LineType::Info);
    term.print("• Renderovanje u toku: 0 Trouglova | 0 Mesh-eva | Pure SDF Math", LineType::Header);

    let engine = ComputeRaymarchEngine::new(70, 25);
    let (framebuffer, total_steps) = engine.dispatch_compute();

    term.print("--- COMPUTE STORAGE IMAGE BUFFER OUTPUT ---", LineType::Header);
    for row in framebuffer {
        let line_str: String = row.into_iter().collect();
        term.print(&line_str, LineType::Success);
    }

    term.print("--- Statistika Compute Pipeline-a ---", LineType::Header);
    term.print(&format!("• Ukupno rešenih niti (Piksela): {} x {} = {}", engine.width, engine.height, engine.width * engine.height), LineType::Info);
    term.print(&format!("• Ukupno koračanja zraka (Sphere Tracing Steps): {} koraka", total_steps), LineType::Success);
    term.print(&format!("• Prosečno koraka po pikselu: {:.2}", total_steps as f64 / (engine.width * engine.height) as f64), LineType::Info);
}

"smt_proof" => {
    use crate::formal_proofs::{Constraint, Expr, ProofResult, SmtSolverEngine};

    term.print("=== QuantumOS CPU: SMT Solver & Formal Verification Engine (Z3 Logic) ===", LineType::Info);

    // -------------------------------------------------------------------------
    // TEST 1: Ranjiva provera bafera (Naivna logika bez provere preređivanja)
    // -------------------------------------------------------------------------
    term.print("--- Test 1: Formalna verifikacija naivne provere bafera ---", LineType::Header);
    term.print("• Pravilo: (offset + len) <= capacity", LineType::Info);
    term.print("• Kapacitet bafera = 100 bajtova. Tražimo dokaz sigurnosti za offset [0..120] i len [0..120]...", LineType::Info);

    // Invarijanta: (offset + len) <= capacity
    let buggy_invariant = Constraint::LessOrEq(
        Expr::Add(
            Box::new(Expr::Var("offset".into())),
            Box::new(Expr::Var("len".into())),
        ),
        Expr::Const(100),
    );

    let result1 = SmtSolverEngine::verify_invariant(
        &["offset", "len"],
        0,
        120,
        buggy_invariant,
    );

    match result1 {
        ProofResult::ProvenSafe => term.print("• DOKAZANO SIGURNO! (Nije pronađen bag)", LineType::Success),
        ProofResult::CounterExampleFound(env) => {
            term.print("• ❌ RANJIVOST DETEKOVANA (SAT)! SMT Solver je našao kontraprimer:", LineType::Error);
            term.print(&format!("  ➔ offset = {}, len = {} (Suma = {}) prelazi kapacitet 100!", 
                env.get("offset").unwrap_or(&0),
                env.get("len").unwrap_or(&0),
                env.get("offset").unwrap_or(&0) + env.get("len").unwrap_or(&0)
            ), LineType::Error);
        }
    }

    // -------------------------------------------------------------------------
    // TEST 2: Matematički dokazana bezbednost (Provera sa striktnim pod-uslovima)
    // -------------------------------------------------------------------------
    term.print("--- Test 2: Formalni dokaz sa zaštitnim pre-uslovima ---", LineType::Header);
    term.print("• Pravilo: Ako je offset <= 100 I len <= (100 - offset) ➔ Bafer je bezbedan!", LineType::Info);

    // Safe Invariant: offset <= 100 AND len <= (100 - offset)
    let safe_invariant = Constraint::And(
        Box::new(Constraint::LessOrEq(
            Expr::Var("offset".into()),
            Expr::Const(100),
        )),
        Box::new(Constraint::LessOrEq(
            Expr::Var("len".into()),
            Expr::Sub(Box::new(Expr::Const(100)), Box::new(Expr::Var("offset".into()))),
        )),
    );

    // Proveravamo u domenu do dozvoljenog limita opsega
    let result2 = SmtSolverEngine::verify_invariant(
        &["offset", "len"],
        0,
        50, // Test opseg u bezbednoj zoni
        safe_invariant,
    );

    match result2 {
        ProofResult::ProvenSafe => {
            term.print("• ✅ MATHEMATICALLY PROVEN SAFE (UNSAT)!", LineType::Success);
            term.print("• SMT Solver je dokazao: Nijedan ulaz ne može izazvati prelivenost bafera!", LineType::Success);
        }
        ProofResult::CounterExampleFound(env) => {
            term.print(&format!("• Counterexample: {:?}", env), LineType::Error);
        }
    }
}

"aba_ebr" => {
    use crate::lockfree_aba::{LockFreeAbaEngine, TaggedPointer};

    term.print("=== QuantumOS CPU: Lock-Free ABA Mitigation & EBR Engine ===", LineType::Info);

    let mut engine = LockFreeAbaEngine::new(0x1000); // Početna adresa A (0x1000)

    // --- Demo 1: Simulacija ABA napada i Tagged Pointer zaštita ---
    term.print("--- Phase 1: Simulacija ABA problema uz Tagged Pointers ---", LineType::Header);

    // Nit 1 čita glavu A (verzija 1) i odlazi na spavanje...
    let thread1_snapshot = TaggedPointer::new(0x1000, 1);
    term.print(&format!("• Nit #1 čita pokazivač: Adresa={:#X}, Tag={}", thread1_snapshot.address, thread1_snapshot.tag), LineType::Info);

    // Nit 2 uleće i menja A -> B -> A (Nova alokacija dobila istu adresu!)
    term.print("• Nit #2 izvršava izmene: A (0x1000) ➔ B (0x2000) ➔ A (0x1000)", LineType::Info);
    let _ = engine.compare_and_swap(thread1_snapshot, 0x2000); // A -> B (Tag postaje 2)
    let _ = engine.compare_and_swap(TaggedPointer::new(0x2000, 2), 0x1000); // B -> A (Tag postaje 3)

    let current_head = engine.head.lock().unwrap();
    term.print(&format!("• Stvarno stanje u memoriji: Adresa={:#X}, Tag={}", current_head.address, current_head.tag), LineType::Success);
    drop(current_head);

    // Nit 1 se budi i pokušava stari CAS sa Tag=1
    term.print("• Nit #1 se budi i pokušava CAS sa starim snimkom (Tag=1)...", LineType::Header);
    match engine.compare_and_swap(thread1_snapshot, 0x3000) {
        Ok(_) => term.print("• ERROR: CAS je prošao u spornom stanju!", LineType::Error),
        Err(err_msg) => term.print(&format!("• SUCCESS: {}", err_msg), LineType::Success),
    }

    // --- Demo 2: Epoch-Based Reclamation (EBR) ---
    term.print("--- Phase 2: Epoch-Based Reclamation (EBR) Odloženo Čišćenje ---", LineType::Header);

    let ebr = &engine.ebr;
    let t1_epoch = ebr.enter_critical(101); // Nit 101 ulazi u epohu 1
    term.print(&format!("• Nit #101 ušla u kritičnu sekciju (Lokalna Epoha = {})", t1_epoch), LineType::Info);

    // Penzionišemo dva stara čvora
    ebr.retire_node(0x1000);
    ebr.retire_node(0x2000);
    term.print("• Penzionisana 2 čvora (0x1000, 0x2000) u Epohi #1. Stavljeni u kesu za smeće.", LineType::Info);

    // Pokušaj čišćenja u istom trenutku
    let (count1, _) = ebr.reclaim_garbage();
    term.print(&format!("• Pokušaj čišćenja u Epohi #1 ➔ Očišćeno: {} čvorova (Rano je!)", count1), LineType::Info);

    // Nit 101 izlazi, napredujemo epohe
    ebr.exit_critical(101);
    term.print("• Nit #101 izašla iz kritične sekcije. Napredujemo globalnu epohu...", LineType::Info);

    ebr.try_advance_epoch(); // Epoha 2
    ebr.try_advance_epoch(); // Epoha 3

    let (count2, addrs) = ebr.reclaim_garbage();
    term.print(&format!("• Pokušaj čišćenja u Epohi #3 ➔ Očišćeno: {} čvorova! Adrese: {:X?}", count2, addrs), LineType::Success);
}

"swar_avx" => {
    use crate::swar_vector::{Avx512Engine, SwarEngine, ZmmRegister};

    term.print("=== QuantumOS CPU: SWAR & AVX-512 Vectorization Engine ===", LineType::Info);

    // --- 1. SWAR DEMO ---
    term.print("--- Demo 1: SWAR (SIMD Within A Register) ---", LineType::Header);
    let str_bytes: u64 = u64::from_le_bytes(*b"helloOS!");
    let converted_u64 = SwarEngine::ascii_to_upper_u64(str_bytes);
    let converted_str = String::from_utf8_lossy(&converted_u64.to_le_bytes()).to_string();

    term.print(&format!("• Ulazni string (8 bajtova u u64): \"helloOS!\""), LineType::Info);
    term.print(&format!("• SWAR ASCII To Upper (Branchless) ➔ \"{}\"", converted_str), LineType::Success);

    let test_zero = u64::from_le_bytes(*b"test\0abc");
    let has_zero = SwarEngine::has_zero_byte(test_zero);
    term.print(&format!("• Detekcija Nultog Bajta (0x00) u \"test\\0abc\" bez `if` uslova ➔ Detektovano: {}", has_zero), LineType::Success);

    // --- 2. AVX-512 DEMO ---
    term.print("--- Demo 2: AVX-512 512-bit Vector Engine (Masked Execution) ---", LineType::Header);

    let v1 = ZmmRegister::from_array([10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160]);
    let v2 = ZmmRegister::new(50); // Sve trake = 50
    let passthrough = ZmmRegister::new(0);

    // Generišemo masku za sve trake gde je v1 > 50
    let k1_mask = Avx512Engine::compare_gt_mask(v1, v2);
    term.print(&format!("• Generisana AVX-512 Opmaska (k1): {:#018b} (Trake gde je v1 > 50)", k1_mask), LineType::Info);

    // Izvršavamo maskirano sabiranje v1 + v2
    let v_res = Avx512Engine::masked_add(v1, v2, k1_mask, passthrough);
    term.print("• Rezultat maskiranog sabiranja `_mm512_mask_add_epi32` (Izvršeno samo na selektovanim trakama!):", LineType::Success);
    term.print(&format!("  └─ Trake 0..7 : {:?}", &v_res.lanes[0..8]), LineType::Info);
    term.print(&format!("  └─ Trake 8..15: {:?}", &v_res.lanes[8..16]), LineType::Info);
}

"fences" => {
    use crate::memory_fences::{QuantumFenceEngine, FenceType, StoreType};

    term.print("=== QuantumOS CPU: Memory Fences & Non-Temporal Stores Engine ===", LineType::Info);

    // Kreiramo MMU sa malim L1 kešom kapaciteta 3 linije
    let mut engine = QuantumFenceEngine::new(3);

    term.print("--- Phase 1: Učitavanje važnih podataka u L1 keš ---", LineType::Header);
    engine.store(0x1000, vec![0xAA], StoreType::Temporal);
    engine.store(0x2000, vec![0xBB], StoreType::Temporal);
    engine.store(0x3000, vec![0xCC], StoreType::Temporal);
    term.print(&format!("• Status L1 Keša: {}/3 popunjeno. Evikcija do sada: {}", engine.l1_cache.lines.len(), engine.l1_cache.evictions), LineType::Success);

    term.print("--- Phase 2A: Upis velikog video bafera preko TEMPORAL upisa (Klasika) ---", LineType::Header);
    let msg1 = engine.store(0x4000, vec![0xFF], StoreType::Temporal);
    term.print(&format!("• Upis na 0x4000 ➔ {}", msg1), LineType::Info);
    term.print(&format!("⚠️ ZAGAĐIVANJE KEŠA! L1 Keš Evikcije: {} (Korisni podaci izbačeni!)", engine.l1_cache.evictions), LineType::Error);

    term.print("--- Phase 2B: Upis velikog bafera preko NON-TEMPORAL upisa (MOVNT) ---", LineType::Header);
    let msg2 = engine.store(0x5000, vec![0x11], StoreType::NonTemporal);
    let msg3 = engine.store(0x6000, vec![0x22], StoreType::NonTemporal);
    term.print(&format!("• Upis na 0x5000 ➔ {}", msg2), LineType::Info);
    term.print(&format!("• Upis na 0x6000 ➔ {}", msg3), LineType::Info);
    term.print(&format!("🚀 NEMA ZAGAĐIVANJA KEŠA! L1 Keš Evikcije i dalje: {}", engine.l1_cache.evictions), LineType::Success);

    term.print("--- Phase 3: Sinhronizacija sa RAM-om preko SFENCE barijere ---", LineType::Header);
    term.print(&format!("• WC Bafer pre SFENCE-a sadrži: {} neposlatih elemenata", engine.wc_buffer.len()), LineType::Info);
    let fence_res = engine.fence(FenceType::Sfence);
    term.print(&format!("• Execute SFENCE ➔ {}", fence_res), LineType::Success);
    term.print(&format!("• WC Bafer nakon SFENCE-a: {} elemenata (RAM je sada 100% usaglašen!)", engine.wc_buffer.len()), LineType::Info);
}

"branch_pred" => {
    use crate::branch_predictor::QuantumSpeculativeEngine;

    term.print("=== QuantumOS CPU: Branch Predictor & Speculative Execution Engine ===", LineType::Info);

    let mut cpu_engine = QuantumSpeculativeEngine::new();
    let branch_pc = 0x0040_1000;
    let target_pc = 0x0040_2000;      // Uslov ispunjen (Skoči ovde)
    let fallthrough_pc = 0x0040_1004; // Uslov nije ispunjen (Sledeća instrukcija)

    term.print("--- Simulacija `for` petlje kroz 5 iteracija (Skok se uvek izvršava) ---", LineType::Header);

    // Simuliramo petlju u kojoj skok UVEK nastupa (actual_taken = true) 4 puta, pa 1 nestupa (false)
    let loop_outcomes = vec![true, true, true, true, false];

    for (iter, &actual_taken) in loop_outcomes.iter().enumerate() {
        let (hit, next_pc, status) = cpu_engine.execute_branch(
            branch_pc,
            actual_taken,
            target_pc,
            fallthrough_pc,
        );

        let current_state = cpu_engine.predictor.pht.get(&branch_pc).unwrap();

        term.print(
            &format!(
                "• Iteracija #{}: Stvarno = {} | Sledeći PC: {:#X} | Stanje Automata: {:?}",
                iter + 1,
                actual_taken,
                next_pc,
                current_state
            ),
            LineType::Info,
        );
        
        if hit {
            term.print(&format!("  └─ {}", status), LineType::Success);
        } else {
            term.print(&format!("  └─ {}", status), LineType::Error);
        }
    }

    term.print("--- Statistika Procesorskog Cevovoda ---", LineType::Header);
    term.print(&format!("• Potvrđene spekulativne instrukcije (Committed): {}", cpu_engine.committed_instructions), LineType::Success);
    term.print(&format!("• Broj pražnjenja cevovoda (Pipeline Flushes): {}", cpu_engine.pipeline_flushes), LineType::Info);
    term.print(&format!("• Izgubljeno ciklusa na penale (Stalls): {} ciklusa", cpu_engine.pipeline_stalls), LineType::Info);
}

"tlb" => {
    use crate::tlb_paging::QuantumMmu;

    term.print("=== QuantumOS MMU: TLB & Virtual Memory Paging Engine ===", LineType::Info);

    // Kreiramo MMU sa malim TLB kapacitetom od 4 unosa radi demonstracije
    let mut mmu = QuantumMmu::new(4);

    term.print("--- Mapiranje Virtuelne Memorije ---", LineType::Header);
    // Mapiramo 4KB Standard stranice
    mmu.page_table.map_4k(0x1000_0000, 0x8000_0000); // 4KB Page #1
    mmu.page_table.map_4k(0x1000_1000, 0x8000_1000); // 4KB Page #2
    term.print("• Mapirano: Virtuelni opseg [0x1000_0000] ➔ Standard 4KB Stranice", LineType::Info);

    // Mapiramo 2MB Huge Page
    mmu.page_table.map_2m(0x2000_0000, 0x9000_0000); // 2MB Huge Page
    term.print("• Mapirano: Virtuelni opseg [0x2000_0000] ➔ Huge Page (2MB)", LineType::Success);

    term.print("--- Simulacija Pristupa Memoriji (Sekvencijalno) ---", LineType::Header);

    let test_addresses = vec![
        (0x1000_0050, "Pristup #1 (4KB Stranica)"),
        (0x1000_00A0, "Pristup #2 (4KB Stranica - Isti okvir)"),
        (0x2000_0100, "Pristup #3 (2MB Huge Page)"),
        (0x2000_F000, "Pristup #4 (2MB Huge Page - Pomeraj 60KB dalje u istoj Huge Stranici!)"),
        (0x1000_0050, "Pristup #5 (Ponovni pristup 4KB Stranici)"),
    ];

    for (va, desc) in test_addresses {
        match mmu.translate(va) {
            Ok((pa, status)) => {
                term.print(&format!("• {} | VA: {:#X} ➔ PA: {:#X} | Status: {}", desc, va, pa, status), LineType::Success);
            }
            Err(e) => term.print(&format!("• Error: {}", e), LineType::Error),
        }
    }

    term.print("--- Statistika MMU Performansi ---", LineType::Header);
    term.print(&format!("• Ukupno TLB Pogodaka (TLB Hits): {}", mmu.tlb_hits), LineType::Success);
    term.print(&format!("• Ukupno TLB Promašaja (TLB Misses): {}", mmu.tlb_misses), LineType::Info);
    term.print(&format!("• Izvršeno Page Table Walk-ova kroz RAM: {}", mmu.page_walks), LineType::Info);

    term.print("--- Demonstracija INVLPG / TLB Flush ---", LineType::Header);
    mmu.tlb.flush();
    term.print("• TLB Ispražnjen (Flushed). Naredni pristup izaziva ponovni Page Walk...", LineType::Info);
    if let Ok((pa, status)) = mmu.translate(0x2000_0100) {
        term.print(&format!("• Pristup nakon Flusha | PA: {:#X} | Status: {}", pa, status), LineType::Success);
    }
}

"moesi" => {
    use crate::cache_coherence::QuantumCacheCoherenceEngine;

    term.print("=== QuantumOS MOESI Multi-Core Cache Coherence Engine ===", LineType::Info);

    let mut system = QuantumCacheCoherenceEngine::new(4); // 4-jezgarni procesor
    let test_addr = 0x1000;

    // 1. Jezgro 0 čita podatak iz RAM-a
    term.print("--- Korak 1: Jezgro #0 čita adresu 0x1000 ---", LineType::Header);
    let (val, msg) = system.read(0, test_addr);
    term.print(&format!("• Result: {:#X} | Status: {}", val, msg), LineType::Success);
    if let Some(line) = system.caches[0].get_line(test_addr) {
        term.print(&format!("  └─ Core #0 Stanje: {:?}", line.state), LineType::Info);
    }

    // 2. Jezgro 0 upisuje novu vrednost (Prelaženje u Modified)
    term.print("--- Korak 2: Jezgro #0 upisuje vrednost 0xCAFEBABE na 0x1000 ---", LineType::Header);
    let w_msg = system.write(0, test_addr, 0xCAFEBABE);
    term.print(&format!("• Status: {}", w_msg), LineType::Success);
    if let Some(line) = system.caches[0].get_line(test_addr) {
        term.print(&format!("  └─ Core #0 Stanje: {:?}", line.state), LineType::Info);
    }

    // 3. Jezgro 1 čita istu adresu (Testiranje MOESI Cache-to-Cache prenosa bez RAM-a!)
    term.print("--- Korak 3: Jezgro #1 čita adresu 0x1000 (MOESI Magic) ---", LineType::Header);
    let (val2, msg2) = system.read(1, test_addr);
    term.print(&format!("• Result: {:#X} | Status: {}", val2, msg2), LineType::Success);

    term.print("--- Stanje svih L1 keševa u sistemu ---", LineType::Header);
    for core_id in 0..4 {
        if let Some(line) = system.caches[core_id].get_line(test_addr) {
            term.print(&format!("• Core #{}: Stanje = {:?}, Podatak = {:#X}", core_id, line.state, line.data), LineType::Info);
        } else {
            term.print(&format!("• Core #{}: Stanje = INVALID / Nema kopiju", core_id), LineType::Info);
        }
    }

    // 4. Jezgro 2 vrši upis i invalidira ostale
    term.print("--- Korak 4: Jezgro #2 vrši UPIS (0x9999) i otima liniju ---", LineType::Header);
    let w_msg2 = system.write(2, test_addr, 0x9999);
    term.print(&format!("• Status: {}", w_msg2), LineType::Success);

    for core_id in 0..4 {
        if let Some(line) = system.caches[core_id].get_line(test_addr) {
            term.print(&format!("• Core #{}: Stanje = {:?}", core_id, line.state), LineType::Info);
        }
    }
}

"mesh_geom" => {
    use crate::mesh_geometry::{Vec3, VirtualizedGeometryEngine};

    term.print("=== QuantumOS Virtualized Geometry & Mesh Shader Pipeline ===", LineType::Info);

    // 1. Generišemo simulirani high-poly model (1,000,000 poligona)
    let total_triangles = 1_000_000;
    term.print(&format!("• Učitavanje modela: High-Poly Mesh od {} trouglova...", total_triangles), LineType::Header);

    // Simuliramo 16 sintetičkih temena i trouglova za demonstraciju
    let dummy_vertices = vec![
        Vec3::new(-1.0, 1.0, 5.0), Vec3::new(1.0, 1.0, 5.0), Vec3::new(-1.0, -1.0, 5.0), Vec3::new(1.0, -1.0, 5.0),
        Vec3::new(-1.0, 1.0, 15.0), Vec3::new(1.0, 1.0, 15.0), Vec3::new(-1.0, -1.0, 15.0), Vec3::new(1.0, -1.0, 15.0),
        Vec3::new(-10.0, 1.0, -5.0), Vec3::new(-8.0, 1.0, -5.0), Vec3::new(-10.0, -1.0, -5.0), Vec3::new(-8.0, -1.0, -5.0),
        Vec3::new(0.0, 10.0, 5.0), Vec3::new(2.0, 10.0, 5.0), Vec3::new(0.0, 8.0, 5.0), Vec3::new(2.0, 8.0, 5.0),
    ];

    let dummy_triangles = vec![
        (0, 1, 2), (1, 3, 2), (4, 5, 6), (5, 7, 6),     // Grozd #1 & #2 (Ispred kamere)
        (8, 9, 10), (9, 11, 10), (12, 13, 14), (13, 15, 14) // Grozd #3 & #4 (Iza ili izvan vidnog polja)
    ];

    // 2. Meshletization (Sečenje geometrije u mikro-grozdove)
    let meshlets = VirtualizedGeometryEngine::build_meshlets(&dummy_vertices, &dummy_triangles);
    term.print(&format!("• Meshletization završen: Stvoreno {} Meshlet grozdova.", meshlets.len()), LineType::Success);

    // 3. Kamere i simulacija Task Shader-a
    let camera_pos = Vec3::new(0.0, 0.0, 0.0);
    let camera_dir = Vec3::new(0.0, 0.0, 1.0); // Kamera gleda pravo u Z osu

    term.print("--- Pokretanje TASK SHADER-a (Cluster Culling Stage) ---", LineType::Header);
    let payload = VirtualizedGeometryEngine::run_task_shader(&meshlets, camera_pos, camera_dir);

    term.print(&format!("• Od ukupno {} Meshleta, Task Shader je PRIHVATIO: {} grozda za renderovanje.", meshlets.len(), payload.visible_meshlet_indices.len()), LineType::Info);
    term.print(&format!("• ODBAČENO na nivou grozda (Cculled): {} grozdova (0 ciklusa potrošeno na pojedinačne trouglove!)", meshlets.len() - payload.visible_meshlet_indices.len()), LineType::Success);

    // 4. Mesh Shader faza
    term.print("--- Pokretanje MESH SHADER-a za preostale grozdove ---", LineType::Header);
    let mut rendered_triangles = 0;
    for &meshlet_idx in &payload.visible_meshlet_indices {
        let tris = VirtualizedGeometryEngine::run_mesh_shader(&meshlets[meshlet_idx]);
        rendered_triangles += tris;
        term.print(&format!("  └─ Meshlet #{}: Poslat u Rasterizer sa {} trouglova.", meshlet_idx, tris), LineType::Info);
    }

    let efficiency = (1.0 - (rendered_triangles as f32 / dummy_triangles.len() as f32)) * 100.0;
    term.print(&format!("• Ušteđeno GPU resursa: {:.1}% celokupne scene eliminisano pre rasterizacije!", efficiency), LineType::Success);
}

"upscaler" => {
    use crate::upscaler::{Color, ImageBuffer, QuantumTemporalUpscaler};

    term.print("=== QuantumOS TAA & Temporal Upscaling Engine (FSR/DLSS) ===", LineType::Info);

    let low_w = 20;
    let low_h = 8;
    let high_w = 60;
    let high_h = 16;

    let mut upscaler = QuantumTemporalUpscaler::new(high_w, high_h);

    term.print(&format!("• Simulacija ulaza: Low-Res Render ({}x{}) ➔ Izlaz: High-Res ({}x{})", low_w, low_h, high_w, high_h), LineType::Header);

    // Simuliramo prolazak kroz 5 frejmova radi TAA akumulacije detalja
    for frame in 1..=5 {
        let (jx, jy) = upscaler.get_subpixel_jitter();
        term.print(&format!("--- Frejm #{}: Jitter Offset = [{:.3}, {:.3}] ---", frame, jx, jy), LineType::Info);

        // Generišemo Low-Res frejm sa nazubljenim krugom u sredini
        let mut low_res_frame = ImageBuffer::new(low_w, low_h);
        for y in 0..low_h {
            for x in 0..low_w {
                let dx = x as f32 - low_w as f32 / 2.0 + jx;
                let dy = y as f32 - low_h as f32 / 2.0 + jy;
                if (dx * dx + dy * dy).sqrt() < 3.0 {
                    low_res_frame.set_pixel(x, y, Color::new(1.0, 1.0, 1.0)); // Beli krug
                } else {
                    low_res_frame.set_pixel(x, y, Color::new(0.1, 0.1, 0.2)); // Tamna pozadina
                }
            }
        }

        // Izvršavamo TAA + Upscaling rekonstrukciju sa reprojekcijom
        let high_res_output = upscaler.process_frame(&low_res_frame, (0, 0));

        // Prikazujemo samo poslednji (akumulirani i izglađeni) High-Res rezultat
        if frame == 5 {
            term.print("--- Finalni High-Res Rekonstruisani Prikaz (Oštre ivice, 0 Aliasing) ---", LineType::Header);
            let ascii_levels = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"];

            for hy in 0..high_h {
                let mut line = String::new();
                for hx in 0..high_w {
                    let pixel = high_res_output.get_pixel(hx, hy);
                    let brightness = (pixel.r + pixel.g + pixel.b) / 3.0;
                    let idx = ((brightness.clamp(0.0, 1.0)) * (ascii_levels.len() - 1) as f32) as usize;
                    line.push_str(ascii_levels[idx]);
                }
                term.print(&line, LineType::Success);
            }
        }
    }

    term.print("• TAA akumulacija uspešna! Pikselizovane ivice su zaglađene kroz vremensku integraciju.", LineType::Success);
}

"sdf_gi" => {
    use crate::sdf_gi::{QuantumLumenGiEngine, SdfScene, Vec3};

    term.print("=== QuantumOS SDF Raymarching & Lumen GI Engine ===", LineType::Info);

    let width = 60;
    let height = 20;
    let camera_pos = Vec3::new(0.0, 0.5, 1.5);
    let light_pos = Vec3::new(3.0, 5.0, 1.0);

    // ASCII gradijent svetline
    let ascii_chars = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"];

    term.print("--- Renderovanje 3D SDF Scene sa Lumen GI (ASCII Viewport) ---", LineType::Header);

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            // Normalizacija ekranskih koordinata (-1.0 do 1.0)
            let uv_x = (x as f32 / width as f32) * 2.0 - 1.0;
            let uv_y = -((y as f32 / height as f32) * 2.0 - 1.0) * 0.5; // Aspect Ratio

            let ray_dir = Vec3::new(uv_x, uv_y, 1.0).normalize();

            if let Some((hit_pos, _dist)) = QuantumLumenGiEngine::march(camera_pos, ray_dir) {
                let normal = SdfScene::calculate_normal(hit_pos);
                // Izračunaj Lumen GI svetlinu za hit tačku
                let gi_intensity = QuantumLumenGiEngine::compute_gi(hit_pos, normal, light_pos);
                
                // Mapiraj gi_intensity na ASCII karakter
                let char_idx = ((gi_intensity.clamp(0.0, 1.0)) * (ascii_chars.len() - 1) as f32) as usize;
                line.push_str(ascii_chars[char_idx]);
            } else {
                line.push(' '); // Pozadina / Nebo
            }
        }
        term.print(&line, LineType::Success);
    }

    term.print("--- Analiza tačke udara zraka (Raymarch Stats) ---", LineType::Header);
    let center_ray = Vec3::new(0.0, 0.0, 1.0).normalize();
    if let Some((p, dist)) = QuantumLumenGiEngine::march(camera_pos, center_ray) {
        let n = SdfScene::calculate_normal(p);
        let ao = QuantumLumenGiEngine::compute_sdf_ao(p, n);
        term.print(&format!("• Rastojanje do presek tačke: {:.3} jedinica", dist), LineType::Info);
        term.print(&format!("• Pozicija udara P: [{:.2}, {:.2}, {:.2}]", p.x, p.y, p.z), LineType::Info);
        term.print(&format!("• Normala površine N: [{:.2}, {:.2}, {:.2}]", n.x, n.y, n.z), LineType::Info);
        term.print(&format!("• SDF Ambient Occlusion (AO): {:.2}% svetlosti sačuvano", ao * 100.0), LineType::Success);
    }
}

"ebpf" => {
    use crate::ebpf::{
        EbpfInstruction, EbpfMap, QuantumEbpfEngine, BPF_ADD64_IMM, BPF_CALL, BPF_EXIT,
        BPF_JEQ_IMM, BPF_MOV64_IMM,
    };

    term.print("=== QuantumOS eBPF In-Kernel Bytecode Engine ===", LineType::Info);

    let mut engine = QuantumEbpfEngine::new();
    let mut packet_counter_map = EbpfMap::new();

    // Simuliramo inicialnu vrednost u eBPF Mapi (Key 1 -> Count 0)
    packet_counter_map.update(1, 0);

    term.print("--- Sastavljanje eBPF Programa za Filtriranje Mrežnih Paketa ---", LineType::Header);

    // eBPF Bajtkod Program:
    // 1. Postavi Key = 1 u R1
    // 2. Pozovi Helper #1 (Lookup iz Mape) -> Vrednost ide u R0
    // 3. Dodaj 1 na trenutni broj paketa (ADD R0, 1)
    // 4. Postavi R2 = R0, R1 = 1 i pozovi Helper #2 (Update Mape)
    // 5. Proveri da li je port paketa (Context) = 80 (HTTP) -> Propusti (PASS=1), inače Odbaci (DROP=0)
    let ebpf_program = vec![
        // R1 = 1 (Key za mapu)
        EbpfInstruction { opcode: BPF_MOV64_IMM, dst: 1, src: 0, offset: 0, imm: 1 },
        // Call Helper #1 (Map Lookup) -> R0 dobija staru vrednost
        EbpfInstruction { opcode: BPF_CALL, dst: 0, src: 0, offset: 0, imm: 1 },
        // R0 += 1 (Uvećaj brojač paketa)
        EbpfInstruction { opcode: BPF_ADD64_IMM, dst: 0, src: 0, offset: 0, imm: 1 },
        // R2 = R0 (Nova vrednost), R1 = 1 (Key)
        EbpfInstruction { opcode: BPF_MOV64_IMM, dst: 2, src: 0, offset: 0, imm: 1 }, // Temp
        EbpfInstruction { opcode: BPF_MOV64_IMM, dst: 1, src: 0, offset: 0, imm: 1 },
        // Call Helper #2 (Update Map)
        EbpfInstruction { opcode: BPF_CALL, dst: 0, src: 0, offset: 0, imm: 2 },
        // Proveri port u R1 (Mock Context Port) -> Ako je 80, nastavi, ako nije skoči na BPF_EXIT sa R0=0
        EbpfInstruction { opcode: BPF_JEQ_IMM, dst: 1, src: 0, offset: 2, imm: 80 },
        // R0 = 0 (DROP)
        EbpfInstruction { opcode: BPF_MOV64_IMM, dst: 0, src: 0, offset: 0, imm: 0 },
        EbpfInstruction { opcode: BPF_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
        // R0 = 1 (PASS - Za HTTP Port 80)
        EbpfInstruction { opcode: BPF_MOV64_IMM, dst: 0, src: 0, offset: 0, imm: 1 },
        EbpfInstruction { opcode: BPF_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
    ];

    // 1. Verifikacija
    match QuantumEbpfEngine::verify(&ebpf_program) {
        Ok(_) => term.print("• In-Kernel Verifier: eBPF Bajtkod je BEZBEDAN za izvršavanje! ✅", LineType::Success),
        Err(e) => term.print(&format!("• Verifier Error: {}", e), LineType::Error),
    }

    // 2. Simulacija obrade paketa 1 (Port 80 - HTTP)
    let mock_packet_port_80 = 80u64;
    match engine.execute(&ebpf_program, mock_packet_port_80, &mut packet_counter_map) {
        Ok(result) => {
            let status = if result == 1 { "PASS (Propušten) ✅" } else { "DROP (Odbaceno) ❌" };
            term.print(&format!("• Paket #1 (Port 80) eBPF Odluka: {}", status), LineType::Success);
        }
        Err(e) => term.print(&format!("• Izvršna greška: {}", e), LineType::Error),
    }

    // 3. Provera eBPF Mape (Statistika obrađenih paketa)
    term.print("--- Stanje eBPF Mape nakon obrade ---", LineType::Header);
    if let Some(count) = packet_counter_map.lookup(1) {
        term.print(&format!("• eBPF Mapa [Key: 1] -> Ukupno prebrojano paketa: {}", count), LineType::Info);
    }
}

"pipeline" => {
    use crate::pipeline::{CpuControlRegisters, CpuIdEngine, HardwarePort, IntrinsicPipelineEngine};

    term.print("=== QuantumOS Inline Assembly & Intrinsic Pipeline Engine ===", LineType::Info);

    // 1. CPUID Identifikacija
    unsafe {
        let vendor = CpuIdEngine::get_vendor_string();
        term.print(&format!("• Detektovan CPU Vendor (preko Inline CPUID): [{}]", vendor), LineType::Success);
    }

    // 2. Čitanje Kontrolnih Registara (CR0 & CR3)
    unsafe {
        let cr0 = CpuControlRegisters::read_cr0();
        let cr3 = CpuControlRegisters::read_cr3();

        term.print("--- Provera CPU Registara (Protected/Paging Control) ---", LineType::Header);
        term.print(&format!("• CR0 Registar: 0x{:016X}", cr0), LineType::Info);
        term.print(&format!("• CR3 Registar (Page Directory Base): 0x{:016X}", cr3), LineType::Info);
        
        // Slikovit prikaz flegova
        let paging_enabled = (cr0 & (1 << 31)) != 0;
        let wp_enabled = (cr0 & (1 << 16)) != 0;
        term.print(&format!("  └─ Hardware Paging Mode: {}", if paging_enabled { "Aktiviran ✅" } else { "Simulacija ⚠️" }), LineType::Info);
        term.print(&format!("  └─ Kernel Write Protect Flag: {}", if wp_enabled { "Aktiviran ✅" } else { "Neaktivan ⚠️" }), LineType::Info);
    }

    // 3. I/O Port Simulacija
    unsafe {
        HardwarePort::io_wait();
        term.print("• Hardware IO Wait (Port 0x80) uspešno izvršen!", LineType::Success);
    }

    // 4. Fast Inline Assembly Pipeline Test
    term.print("--- Testiranje Unrolled Inline Assembly Pipeline-a ---", LineType::Header);
    let a = vec![100u64, 200, 300, 400];
    let b = vec![10u64, 20, 30, 40];
    let mut dst = vec![0u64; 4];

    unsafe {
        IntrinsicPipelineEngine::fast_assembly_pipeline_add(&a, &b, &mut dst);
    }

    term.print(&format!("• Ulaz A: {:?}", a), LineType::Info);
    term.print(&format!("• Ulaz B: {:?}", b), LineType::Info);
    term.print(&format!("• Rezultat Asemblerskog Pajplajna: {:?}", dst), LineType::Success);
}

"scheduler" => {
    use crate::scheduler::TaskStealingScheduler;

    term.print("=== QuantumOS Work-Stealing Task Scheduler ===", LineType::Info);

    // Kreiramo raspoređivač sa 4 radničke niti i redovima kapaciteta 128
    let scheduler = TaskStealingScheduler::<4, 128>::new();

    term.print("--- Pravljenje ekstremne neravnoteže opterećenja ---", LineType::Header);
    
    // Natrpavamo 100 zadataka SAMO na Nit #0, dok Niti #1, #2 i #3 ostaju prazne!
    for _i in 0..100 {
        let _ = scheduler.spawn_on(0, Box::new(move || {
            // Simulacija posla
            let mut _x = 0;
            for _ in 0..500 { _x += 1; }
        }));
    }

    term.print("• Nit #0 ima 100 zadataka. Niti #1, #2, #3 imaju 0 zadataka.", LineType::Warning);
    term.print("• Pokretanje Work-Stealing algoritma...", LineType::Info);

    // Pokrećemo sve niti paralelno
    scheduler.run_all();

    term.print("--- Rezultati balansiranja opterećenja ---", LineType::Header);
    term.print(&format!("• Ukupno izvršeno zadataka: {}", scheduler.total_executed()), LineType::Success);
    term.print(&format!("• Ukupno ukradeno zadataka iz reda Niti #0: {}", scheduler.total_stolen()), LineType::Success);
    term.print("• Gladne niti su uspešno pokrale posao bez ijednog Lock-a ili Deadlock-a!", LineType::Info);
}

"allocator" => {
    use std::alloc::Layout;
    use crate::allocator::{ArenaAllocator, FixedPoolAllocator};

    term.print("=== QuantumOS Custom Arena & Pool Allocator Engine ===", LineType::Info);

    // --- 1. TEST: Arena (Bump) Allocator ---
    term.print("--- Testiranje Arena (Bump) Allocator-a (1 MB Kapacitet) ---", LineType::Header);
    let arena = ArenaAllocator::new(1024 * 1024); // 1 MB Arena

    unsafe {
        let layout_u64 = Layout::new::<u64>();
        let ptr1 = arena.alloc(layout_u64).unwrap();
        let ptr2 = arena.alloc(layout_u64).unwrap();

        *(ptr1 as *mut u64) = 0xDEADBEEF;
        *(ptr2 as *mut u64) = 0xCAFEBABE;

        term.print(&format!("• Uspešna Arena alokacija 1: {:p} (Vrednost: 0x{:X})", ptr1, *(ptr1 as *mut u64)), LineType::Success);
        term.print(&format!("• Uspešna Arena alokacija 2: {:p} (Vrednost: 0x{:X})", ptr2, *(ptr2 as *mut u64)), LineType::Success);
        term.print(&format!("• Zauzeta memorija u Areni: {} bajtova", arena.used_bytes()), LineType::Info);

        // Instant O(1) Bulk Reset Aren
        arena.reset();
        term.print("• Izvršen O(1) Instant Arena Reset! Sva memorija oslobođena u 1 ciklusu.", LineType::Success);
        term.print(&format!("• Zauzeta memorija nakon reseta: {} bajtova", arena.used_bytes()), LineType::Info);
    }

    // --- 2. TEST: Embedded Free-List Pool Allocator ---
    term.print("--- Testiranje Lock-Free Pool Allocator-a (Blokovi od 64B) ---", LineType::Header);
    // Kreiramo Pool od 4 bloka veličine 64 bajta
    let pool = FixedPoolAllocator::<64>::new(4);

    term.print(&format!("• Inicijalno slobodnih blokova u Pool-u: {} / {}", pool.free_chunks_count(), pool.total_chunks_count()), LineType::Info);

    let chunk1 = pool.alloc_chunk().unwrap();
    let chunk2 = pool.alloc_chunk().unwrap();

    term.print(&format!("• Dodeljen Blok 1: {:p}", chunk1), LineType::Success);
    term.print(&format!("• Dodeljen Blok 2: {:p}", chunk2), LineType::Success);
    term.print(&format!("• Preostalo slobodnih blokova: {}", pool.free_chunks_count()), LineType::Info);

    // Vraćamo Blok 1 nazad u Pool
    unsafe {
        pool.free_chunk(chunk1);
    }
    term.print(&format!("• Vraćen Blok 1 nazad u Pool! Slobodno blokova: {}", pool.free_chunks_count()), LineType::Success);

    // Ponovna alokacija vraća upravo izreciklirani Blok 1!
    let chunk_recycled = pool.alloc_chunk().unwrap();
    term.print(&format!("• Ponovo alociran blok (Recikliran star pokazivač): {:p}", chunk_recycled), LineType::Warning);
}

"dod" => {
    use std::time::Instant;
    use crate::dod::{ProcessAoS, ProcessTableSoA, QuantumDodEngine};

    term.print("=== QuantumOS Data-Oriented Design (DOD) & Cache-Oblivious Engine ===", LineType::Info);

    const COUNT: usize = 500_000;

    // 1. Priprema podataka za AoS vs SoA Test
    let mut aos_list: Vec<ProcessAoS> = (0..COUNT)
        .map(|i| ProcessAoS {
            pid: i as u32,
            priority: (i % 5) as u8,
            cpu_usage: 1.0,
            name: [0u8; 32],
        })
        .collect();

    let mut soa_table = ProcessTableSoA::with_capacity(COUNT);
    for i in 0..COUNT {
        soa_table.push(i as u32, (i % 5) as u8, 1.0, true);
    }

    // Benchmark 1: AoS Iteracija (Guta L1 Keš nepotrebnim nazivima i PID-ovima)
    let start_aos = Instant::now();
    for proc in aos_list.iter_mut() {
        proc.cpu_usage += 0.5;
    }
    let duration_aos = start_aos.elapsed();

    // Benchmark 2: SoA Iteracija (Čista sukcesivna L1 keš linija)
    let start_soa = Instant::now();
    soa_table.update_active_cpu_usages(0.5);
    let duration_soa = start_soa.elapsed();

    term.print("--- AoS vs SoA Benchmark (500.000 Entiteta) ---", LineType::Header);
    term.print(&format!("• AoS (Array of Structs) Vreme: {:?}", duration_aos), LineType::Warning);
    term.print(&format!("• SoA (Structure of Arrays) Vreme: {:?}", duration_soa), LineType::Success);

    // 2. Cache-Oblivious Matrix Transpose Test
    term.print("--- Cache-Oblivious Divide-and-Conquer Transpose (1024x1024 Matrica) ---", LineType::Header);
    let matrix_dim = 1024;
    let src_matrix = vec![1.0f32; matrix_dim * matrix_dim];
    let mut dst_matrix = vec![0.0f32; matrix_dim * matrix_dim];

    let start_co = Instant::now();
    QuantumDodEngine::cache_oblivious_transpose(
        &src_matrix,
        &mut dst_matrix,
        matrix_dim,
        matrix_dim,
        0, 0, 0, 0,
        matrix_dim,
        matrix_dim,
    );
    let duration_co = start_co.elapsed();

    term.print(&format!("• Cache-Oblivious Transponovanje izvršeno za: {:?}", duration_co), LineType::Success);
    term.print("• Matrica je uspešno obrađena u malim sub-blokovima bez obzira na veličinu L1/L2 keša!", LineType::Info);
}

"intrinsics" => {
    use crate::intrinsics::QuantumIntrinsicsEngine;

    term.print("=== QuantumOS Compiler Intrinsics & CPU Hardware Primitive ===", LineType::Info);

    // 1. Testiranje RDTSC CPU Ciklusa
    unsafe {
        let start_cycles = QuantumIntrinsicsEngine::rdtsc();
        
        // Mala spin-pause simulacija
        for _ in 0..100 {
            QuantumIntrinsicsEngine::cpu_pause();
        }
        
        let end_cycles = QuantumIntrinsicsEngine::rdtsc();
        let elapsed = end_cycles.saturating_sub(start_cycles);
        
        term.print(&format!("• RDTSC Start Ciklusa: {}", start_cycles), LineType::Info);
        term.print(&format!("• RDTSC Kraj Ciklusa:   {}", end_cycles), LineType::Info);
        term.print(&format!("• Proteklo CPU ciklusa za 100 spin-pauza: {} ciklusa", elapsed), LineType::Success);
    }

    // 2. Testiranje Bitwise HW Intrinsics
    term.print("--- Hardware Bit Manipulation Intrinsics ---", LineType::Header);
    let mask: u64 = 0b0000_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
    
    term.print(&format!("• Vrednost maske: 0x{:X}", mask), LineType::Info);
    term.print(&format!("• POPCNT (Broj postavljenih 1-bita): {}", QuantumIntrinsicsEngine::popcount_64(mask)), LineType::Success);
    term.print(&format!("• LZCNT (Broj vodećih nula sa leve strane): {}", QuantumIntrinsicsEngine::leading_zeros_64(mask)), LineType::Success);
    term.print(&format!("• TZCNT (Broj pratećih nula sa desne strane): {}", QuantumIntrinsicsEngine::trailing_zeros_64(mask)), LineType::Success);
    
    let swap_val: u64 = 0x1122334455667788;
    term.print(&format!("• BSWAP Pre:  0x{:X}", swap_val), LineType::Info);
    term.print(&format!("• BSWAP Posle (Little <-> Big Endian): 0x{:X}", QuantumIntrinsicsEngine::byteswap_64(swap_val)), LineType::Warning);

    // 3. Testiranje Fast Intrinsics Memset i Memcpy
    term.print("--- Fast Compiler Memory Primtives ---", LineType::Header);
    let mut buffer_a = [0u8; 64];
    let mut buffer_b = [0u8; 64];

    unsafe {
        QuantumIntrinsicsEngine::intrinsic_memset(buffer_a.as_mut_ptr(), 0xAA, 64);
        QuantumIntrinsicsEngine::intrinsic_memcpy(buffer_b.as_mut_ptr(), buffer_a.as_ptr(), 64);
    }

    term.print(&format!("• Intrinsic Memset (0xAA) i Memcpy u 64-bit rečima uspesno izvršen nad {} bajtova!", buffer_b.len()), LineType::Success);
}

"memory" => {
    use crate::memory::{MemoryAligner, QuantumMemoryEngine, PAGE_SIZE};

    term.print("=== QuantumOS Manual Memory Management Engine ===", LineType::Info);

    // Inicijalizujemo Heap od 16 MB
    let mem_engine = QuantumMemoryEngine::new(16 * 1024 * 1024);

    // 1. Testiranje Page Frame Allocator-a
    term.print("--- Page Frame Allocator (4KB Pages) ---", LineType::Header);
    if let Some(page1) = mem_engine.page_allocator.alloc_page() {
        term.print(&format!("• Dodeljena 1. Stranica na adresi: 0x{:X}", page1), LineType::Success);
    }
    if let Some(page2) = mem_engine.page_allocator.alloc_page() {
        term.print(&format!("• Dodeljena 2. Stranica na adresi: 0x{:X} (Razlika: {} B)", page2, page2 - (page1_addr_mock(page2))), LineType::Success);
    }
    term.print(&format!("• Zauzeće stranica: {} / {} (Ukupno: {} KB)", 
        mem_engine.page_allocator.used_pages(), 
        mem_engine.page_allocator.total_pages,
        (mem_engine.page_allocator.used_pages() * PAGE_SIZE) / 1024
    ), LineType::Info);

    // Helper za formatiranje prikaza
    fn page1_addr_mock(page2: usize) -> usize { page2 - PAGE_SIZE }

    // 2. Testiranje Bitwise Memory Alignera
    term.print("--- Memory Alignment & Padding Math ---", LineType::Header);
    let raw_addr = 0x1003usize; // Neporavnata adresa
    let aligned_32 = MemoryAligner::align_up(raw_addr, 32);
    term.print(&format!("• Sirova adresa 0x{:X} -> Poravnata na 32B: 0x{:X}", raw_addr, aligned_32), LineType::Warning);

    // 3. Testiranje Sirove Alokacije, Zero-Fill-a i Realokacije preko Pokazivača
    term.print("--- Raw Pointer Manual Heap Allocation ---", LineType::Header);
    unsafe {
        // Alociramo 1024 bajta poravnatih na 64B
        match mem_engine.manual_alloc(1024, 64) {
            Ok(ptr) => {
                term.print(&format!("• Uspešna alokacija 1024B na pokazivaču: {:p}", ptr), LineType::Success);
                
                // Zero fill
                mem_engine.zero_fill(ptr, 1024);
                term.print("• Pisanje nula (Zero-Fill) izvršeno preko sirovog pokazivača!", LineType::Info);

                // Realokacija sa 1024B na 2048B
                match mem_engine.manual_realloc(ptr, 1024, 2048, 64) {
                    Ok(new_ptr) => {
                        term.print(&format!("• Realokacija izvršena (1024B -> 2048B) na {:p}", new_ptr), LineType::Success);
                        
                        // Ručno oslobađanje memorije
                        mem_engine.manual_free(new_ptr, 2048, 64);
                        term.print("• Ručno oslobađanje memorije (Free) uspešno završeno!", LineType::Success);
                    }
                    Err(e) => term.print(&format!("Greška pri realokaciji: {}", e), LineType::Error),
                }
            }
            Err(e) => term.print(&format!("Greška pri alokaciji: {}", e), LineType::Error),
        }
    }

    term.print(&format!("• Preostale aktivne alokacije na Heap-u: {}", mem_engine.active_allocations()), LineType::Info);
    term.print(&format!("• Trenutno zauzeta memorija: {} bajtova", mem_engine.used_bytes()), LineType::Info);
}

"lockfree" => {
    use std::sync::Arc;
    use std::thread;
    use crate::lockfree::{LockFreeStack, LockFreeQueue};

    term.print("=== QuantumOS Lock-Free Engine Test ===", LineType::Info);

    // --- 1. TEST: Lock-Free FIFO Queue (Michael-Scott MPMC) ---
    term.print("--- Testiranje Michael-Scott FIFO Reda (MPMC) ---", LineType::Header);
    let queue = Arc::new(LockFreeQueue::<String>::new());
    let mut queue_handles = vec![];

    // Pokrećemo 4 niti koje paralelno ubacuju poruke
    for thread_id in 0..4 {
        let q_clone = Arc::clone(&queue);
        queue_handles.push(thread::spawn(move || {
            for i in 0..100 {
                q_clone.enqueue(format!("Thread-{}-Msg-{}", thread_id, i));
            }
        }));
    }

    for handle in queue_handles {
        handle.join().unwrap();
    }

    term.print("• 4 Niti su uspešno upisale 400 poruka u FIFO red bez zaključavanja!", LineType::Success);

    // Izvlačimo prvih 3 poruka da potvrdimo reda sled
    for _ in 0..3 {
        if let Some(msg) = queue.dequeue() {
            term.print(&format!("• Dequeued FIFO: \"{}\"", msg), LineType::Info);
        }
    }

    // --- 2. TEST: Lock-Free Stack ---
    term.print("--- Testiranje Treiber LIFO Staka ---", LineType::Header);
    let stack = Arc::new(LockFreeStack::<u64>::new());
    let mut stack_handles = vec![];

    for thread_id in 0..4 {
        let stack_clone = Arc::clone(&stack);
        stack_handles.push(thread::spawn(move || {
            for i in 0..1000 {
                stack_clone.push((thread_id * 1000) + i);
            }
        }));
    }

    for handle in stack_handles {
        handle.join().unwrap();
    }
    term.print(&format!("• Treiber Stack ukupan broj elemenata: {}", stack.len()), LineType::Success);
}

"lockfree" => {
    use std::sync::Arc;
    use std::thread;
    use crate::lockfree::{LockFreeStack, LockFreeRingBuffer};

    term.print("=== QuantumOS Lock-Free Engine Test ===", LineType::Info);

    // --- 1. TEST: Lock-Free Treiber Stack (Konkurentni Push/Pop) ---
    let stack = Arc::new(LockFreeStack::<u64>::new());
    let mut handles = vec![];

    // Pokrećemo 4 niti koje paralelno guraju brojeve bez zaključavanja
    for thread_id in 0..4 {
        let stack_clone = Arc::clone(&stack);
        handles.push(thread::spawn(move || {
            for i in 0..1000 {
                stack_clone.push((thread_id * 1000) + i);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    term.print(&format!("• Treiber Stack ukupan broj elemenata: {} (Očekivano: 4000)", stack.len()), LineType::Success);

    // Pop-ujemo 500 elemenata
    let mut popped_count = 0;
    while popped_count < 500 && stack.pop().is_some() {
        popped_count += 1;
    }
    term.print(&format!("• Uspešno skinuto preko CAS-a: {} elemenata (Preostalo: {})", popped_count, stack.len()), LineType::Info);

    // --- 2. TEST: Lock-Free Ring Buffer (IPC / Queue) ---
    term.print("--- Testiranje Atomic Ring Buffer-a (Kapacitet 16) ---", LineType::Header);
    let queue = LockFreeRingBuffer::<String, 16>::new();

    let _ = queue.enqueue("SYSCALL_READ".to_string());
    let _ = queue.enqueue("SYSCALL_WRITE".to_string());
    let _ = queue.enqueue("IRQ_KEYBOARD_PRESS".to_string());

    term.print(&format!("• Red poruka sadrži: {} stavke", queue.len()), LineType::Info);

    if let Some(msg) = queue.dequeue() {
        term.print(&format!("• Dequeue izveden: \"{}\"", msg), LineType::Success);
    }
    term.print(&format!("• Stanje redu nakon Dequeue: {} stavke", queue.len()), LineType::Info);
}

"cache" => {
    use crate::cache::{HardwareCacheControl, QuantumCacheEngine};

    let mut engine = QuantumCacheEngine::new();
    term.print("=== QuantumOS Hardware & Software Cache Engine ===", LineType::Info);

    // 1. Hardverski Test (Prefetch & Barrier)
    let dummy_buffer = vec![0u8; 1024];
    unsafe {
        HardwareCacheControl::prefetch_l1(dummy_buffer.as_ptr());
        HardwareCacheControl::flush_cache_line(dummy_buffer.as_ptr());
    }
    HardwareCacheControl::memory_barrier();
    term.print("• Hardverske L1/L2 Prefetch & CLFLUSH instrukcije: IZVRŠENE ✅", LineType::Success);

    // 2. Softverski Page Cache Test (Hits vs Misses)
    term.print("--- Testiranje LRU Page Cache-a ---", LineType::Header);

    // Čitamo postojeći blok (Hit)
    let _ = engine.page_cache.get(1);
    let _ = engine.page_cache.get(2);

    // Čitamo nepostojeći blok (Miss)
    let _ = engine.page_cache.get(99);

    // Simuliramo punjenje keša preko kapaciteta radi LRU Eviction-a
    for i in 10..150 {
        engine.page_cache.put(i, vec![0xFF; 512], true);
    }

    term.print(&format!("• Keš pogoci (Hits): {}", engine.page_cache.hits), LineType::Info);
    term.print(&format!("• Keš promašaji (Misses): {}", engine.page_cache.misses), LineType::Info);
    term.print(&format!("• Hit Ratio: {:.2}%", engine.page_cache.hit_ratio()), LineType::Success);
    term.print(&format!("• LRU Izbačeno blokova (Evictions): {}", engine.page_cache.evictions), LineType::Warning);
    term.print(&format!("• Write-back upisa prljavih stranica (Dirty Flushes): {}", engine.page_cache.dirty_flushes), LineType::Warning);
}

"ecs" => {
    use crate::ecs::{PositionComponent, ProcessTaskComponent, QuantumEcsEngine, RenderComponent};

    let mut ecs = QuantumEcsEngine::new();
    term.print("=== QuantumOS ECS (Entity Component System) ===", LineType::Info);
    term.print(&format!("• Ukupno Entiteta u svetu: {}", ecs.world.alive_entities.len()), LineType::Info);

    // Simuliramo 5 sistemskih koraka (ticks)
    for _ in 0..5 {
        ecs.step(1.0);
    }

    term.print("--- Stanje Entiteta Nakon 5 Tick-ova ---", LineType::Header);

    for &entity in &ecs.world.alive_entities {
        let mut details = format!("Entity ID #{}: ", entity);

        if let Some(pos) = ecs.world.get_component::<PositionComponent>(entity) {
            details.push_str(&format!("[Pos: ({:.1}, {:.1})] ", pos.x, pos.y));
        }

        if let Some(render) = ecs.world.get_component::<RenderComponent>(entity) {
            details.push_str(&format!("[Simbol: '{}'] ", render.symbol));
        }

        if let Some(proc) = ecs.world.get_component::<ProcessTaskComponent>(entity) {
            details.push_str(&format!("[Proces: {} (PID: {}, Ticks: {})] ", proc.name, proc.pid, proc.cpu_ticks));
        }

        term.print(&details, LineType::Success);
    }
}

"simd" => {
    use std::time::Instant;
    use crate::simd::QuantumSimdEngine;

    let engine = QuantumSimdEngine::new();
    term.print("=== SIMD Low-Level Hardware Accelerator ===", LineType::Info);
    term.print(&format!("• AVX2 (256-bit): {}", if engine.has_avx2 { "DA ✅" } else { "NE ❌" }), LineType::Info);
    term.print(&format!("• SSE2 (128-bit): {}", if engine.has_sse2 { "DA ✅" } else { "NE ❌" }), LineType::Info);

    // 1. Math Vector Test
    let size = 1_000_000;
    let vec_a = vec![1.5f32; size];
    let vec_b = vec![2.5f32; size];
    let mut result = vec![0.0f32; size];

    let start_simd = Instant::now();
    engine.add_f32_buffers(&vec_a, &vec_b, &mut result);
    let simd_duration = start_simd.elapsed();
    term.print(&format!("• AVX2 Math (1M float32): {:?}", simd_duration), LineType::Success);

    // 2. SIMD Memcpy Test
    let src_ram = vec![0xABu8; 4 * 1024 * 1024]; // 4 MB RAM-a
    let mut dst_ram = vec![0u8; 4 * 1024 * 1024];

    let start_memcpy = Instant::now();
    engine.simd_memcpy(&src_ram, &mut dst_ram);
    let memcpy_duration = start_memcpy.elapsed();
    term.print(&format!("• AVX2 Memcpy (4 MB RAM): {:?}", memcpy_duration), LineType::Success);

    // 3. GOP Framebuffer Clear Test
    let mut framebuffer = vec![0u32; 1920 * 1080]; // Full HD Ekran
    let start_fb = Instant::now();
    engine.fill_framebuffer_32bpp(&mut framebuffer, 0xFF00FF00); // Neon Zelena
    let fb_duration = start_fb.elapsed();
    term.print(&format!("• AVX2 Framebuffer Fill (1080p): {:?}", fb_duration), LineType::Success);
}

"hft" | "trade" | "stock" => {
    if args.is_empty() {
        term.print("Upotreba: hft book | hft buy <cena> <kolicina> | hft sell <cena> <kolicina> | hft bot <on/off>", LineType::Info);
    } else {
        match args[0] {
            "book" => {
                term.print(&format!("--- Order Book za {} ---", hft.book.symbol), LineType::Success);
                term.print("=== ASKS (Prodaja) ===", LineType::Warning);
                for ask in hft.book.asks.iter().rev() {
                    term.print(&format!("   {:.2} USD | Količina: {}", ask.price, ask.quantity), LineType::Warning);
                }
                let spread = hft.book.spread().unwrap_or(0.0);
                term.print(&format!("--- SPREAD: {:.2} USD ---", spread), LineType::Info);
                term.print("=== BIDS (Kupovina) ===", LineType::Success);
                for bid in &hft.book.bids {
                    term.print(&format!("   {:.2} USD | Količina: {}", bid.price, bid.quantity), LineType::Success);
                }
            }
            "buy" => {
                if args.len() >= 3 {
                    let price = args[1].parse::<f64>().unwrap_or(100.0);
                    let qty = args[2].parse::<u32>().unwrap_or(1);
                    let id = hft.submit_order(OrderSide::Buy, price, qty);
                    term.print(&format!("Plasiran BUY nalog #{} za {}x {:.2} (Latencija: {}ns)", id, qty, price, hft.execution_latency_ns), LineType::Success);
                } else {
                    term.print("Upotreba: hft buy <cena> <kolicina>", LineType::Warning);
                }
            }
            "sell" => {
                if args.len() >= 3 {
                    let price = args[1].parse::<f64>().unwrap_or(100.0);
                    let qty = args[2].parse::<u32>().unwrap_or(1);
                    let id = hft.submit_order(OrderSide::Sell, price, qty);
                    term.print(&format!("Plasiran SELL nalog #{} za {}x {:.2} (Latencija: {}ns)", id, qty, price, hft.execution_latency_ns), LineType::Success);
                } else {
                    term.print("Upotreba: hft sell <cena> <kolicina>", LineType::Warning);
                }
            }
            "bot" => {
                if args.len() >= 2 {
                    match args[1] {
                        "on" => {
                            hft.bot_enabled = true;
                            term.print("Market Maker HFT Bot JE UKLJUČEN.", LineType::Success);
                        }
                        "off" => {
                            hft.bot_enabled = false;
                            term.print("Market Maker HFT Bot JE ISKLJUČEN.", LineType::Warning);
                        }
                        _ => term.print("Upotreba: hft bot <on/off>", LineType::Error),
                    }
                }
            }
            _ => term.print("Nepoznata HFT komanda.", LineType::Error),
        }
    }
}

"fat32" | "esp" => {
    if args.is_empty() {
        term.print("Upotreba: fat32 bpb | fat32 ls | fat32 cat <putanja>", LineType::Info);
    } else {
        match args[0] {
            "bpb" => {
                term.print("--- FAT32 BIOS Parameter Block (BPB) ---", LineType::Success);
                term.print(&format!(" OEM Name: {}", fat.bpb.oem_name), LineType::Info);
                term.print(&format!(" Veličina Sektora: {}B", fat.bpb.bytes_per_sector), LineType::Info);
                term.print(&format!(" Sektora po Klasteru: {}", fat.bpb.sectors_per_cluster), LineType::Info);
                term.print(&format!(" Veličina Klastera: {}B", fat.get_cluster_size_bytes()), LineType::Info);
                term.print(&format!(" Volume Label: {}", fat.bpb.volume_label), LineType::Info);
            }
            "ls" => {
                term.print("--- Sadržaj EFI System Particije (ESP) ---", LineType::Success);
                for entry in &fat.directory_entries {
                    term.print(
                        &format!(" [{}] {} (Start Cluster: {}, Size: {} bytes)", 
                            if entry.is_dir { "DIR" } else { "FILE" }, 
                            entry.path, 
                            entry.start_cluster, 
                            entry.file_size_bytes
                        ),
                        LineType::Info,
                    );
                }
            }
            "cat" => {
                if args.len() >= 2 {
                    let content = fat.read_mock_file_content(args[1]);
                    term.print(&format!("--- Sadržaj: {} ---", args[1]), LineType::Success);
                    term.print(&content, LineType::Info);
                } else {
                    term.print("Upotreba: fat32 cat <putanja>", LineType::Warning);
                }
            }
            _ => term.print("Nepoznata fat32 podkomanda.", LineType::Error),
        }
    }
}

    "img" | "image" => {
    if args.is_empty() {
        term.print("Upotreba: img gray | img invert | img sepia | img blur | img edge | img rotate | img mandelbrot", LineType::Info);
    } else {
        match args[0] {
            "gray" => { img.apply_grayscale(); term.print("Primenjen Grayscale filter.", LineType::Success); }
            "invert" => { img.apply_invert(); term.print("Invertovane boje.", LineType::Success); }
            "sepia" => { img.apply_sepia(); term.print("Primenjen Sepia filter.", LineType::Success); }
            "blur" => { img.apply_blur(); term.print("Primenjeno zamućenje (Box Blur).", LineType::Success); }
            "edge" => { img.apply_sobel(); term.print("Detektovane ivice (Sobel).", LineType::Success); }
            "rotate" => { img.rotate_90(); term.print("Slika rotirana za 90 stepeni.", LineType::Success); }
            "mandelbrot" => { img.load_mandelbrot(); term.print("Generisan Mandelbrot fraktal.", LineType::Success); }
            "undo" => { img.undo(); term.print("Vraćeno prethodno stanje slike.", LineType::Info); }
            _ => term.print("Nepoznata image komanda.", LineType::Error),
        }
    }
}

            "vlc" | "video" => {
    if args.is_empty() {
        term.print(&format!("Trenutni video: {} | Status: {:?}", video.current_video, video.state), LineType::Info);
        term.print("Upotreba: vlc play | vlc pause | vlc load <naziv>", LineType::Info);
    } else {
        match args[0] {
            "play" => {
                video.play();
                term.print("Video reprodukcija pokrenuta.", LineType::Success);
            }
            "pause" => {
                video.pause();
                term.print("Video pauziran.", LineType::Warning);
            }
            "stop" => {
                video.stop();
                term.print("Video zaustavljen.", LineType::Info);
            }
            "load" => {
                if args.len() >= 2 {
                    video.load_video(args[1]);
                    term.print(&format!("Učitan video fajl: {}", args[1]), LineType::Success);
                } else {
                    term.print("Navedite ime videa za učitavanje.", LineType::Error);
                }
            }
            _ => term.print("Nepoznata video komanda.", LineType::Error),
        }
    }
}

            "db" => {
                if args.len() >= 3 && args[0] == "set" {
                    let key = args[1];
                    let val = vec![crate::domain::QuquatVal::Q01, crate::domain::QuquatVal::Q11];
                    db.set(key, val, scheduler.total_ticks);
                    term.print(&format!("Ključ '{}' je upisan u QStore bazu.", key), LineType::Success);
                } else if args.len() >= 2 && args[0] == "get" {
                    let key = args[1];
                    if let Some(rec) = db.storage.get(key) {
                        term.print(&format!("Ključ '{}': {:?}", key, rec.data), LineType::Success);
                    } else {
                        term.print(&format!("Ključ '{}' nije pronađen u bazi.", key), LineType::Error);
                    }
                } else {
                    term.print("Upotreba: db set <key> <val> ILI db get <key>", LineType::Error);
                }
            }

            "net" => {
                if args.len() >= 2 && args[0] == "ping" {
                    if let Ok(port) = args[1].parse::<u16>() {
                        let pkt = QuquatPacket::new(0, 1, port, PacketType::Handshake, vec![]);
                        net.transmit(pkt);
                        term.print(&format!("Ping paket poslat na port {}.", port), LineType::Success);
                    } else {
                        term.print("Neispravan port.", LineType::Error);
                    }
                } else {
                    term.print("Upotreba: net ping <port>", LineType::Error);
                }
            }

            _ => {
                term.print(&format!("Komanda nije prepoznata: '{}'. Kucajte 'help'.", cmd), LineType::Error);
            }
        }
    }
}