pub mod cpu;
pub mod gpu;

use cpu::{CpuRayTracer, Sphere, Vec3};
use gpu::{GpuBackendType, GpuRayTracer};

pub enum RenderBackendMode {
    CpuSoftware,
    GpuHardware(GpuBackendType),
}

pub struct RayTracingEngine {
    pub mode: RenderBackendMode,
    pub cpu_tracer: CpuRayTracer,
    pub gpu_tracer: GpuRayTracer,
}

impl RayTracingEngine {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            mode: RenderBackendMode::CpuSoftware,
            cpu_tracer: CpuRayTracer::new(width, height),
            gpu_tracer: GpuRayTracer::new(width as u32, height as u32, GpuBackendType::Vulkan),
        }
    }

    /// Pokreće rendering u zavisnosti od izabranog režima (CPU ili GPU)
    pub fn render(&mut self, framebuffer: &mut [u32]) -> &'static str {
        let test_scene = [
            Sphere { center: Vec3::new(0.0, 0.0, -3.0), radius: 1.0, color_rgb: (220, 60, 60) },   // Crvena sfera
            Sphere { center: Vec3::new(2.0, 0.5, -4.0), radius: 1.2, color_rgb: (60, 220, 80) },   // Zelena sfera
            Sphere { center: Vec3::new(-2.0, -0.2, -2.5), radius: 0.8, color_rgb: (60, 100, 240) },// Plava sfera
        ];

        match self.mode {
            RenderBackendMode::CpuSoftware => {
                self.cpu_tracer.render_frame(framebuffer, &test_scene);
                "CPU Rendering završen uspešno (Software Raster/Trace)."
            }
            RenderBackendMode::GpuHardware(_) => {
                match self.gpu_tracer.dispatch_compute(test_scene.len()) {
                    Ok(_) => "GPU Compute Dispatch uspešan. Slika je baferovana u VRAM-u.",
                    Err(e) => e,
                }
            }
        }
    }
}