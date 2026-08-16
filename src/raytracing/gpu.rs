use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GpuSphereData {
    pub position_radius: [f32; 4], // x, y, z, radius
    pub color: [f32; 4],           // r, g, b, unused
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuBackendType {
    Vulkan,
    Metal,
    DirectX12,
    SoftwareEmulatedGpu,
}

pub struct GpuRayTracer {
    pub backend: GpuBackendType,
    pub compute_workgroups: (u32, u32, u32),
    pub dispatched_passes: AtomicU64,
    pub is_pipeline_ready: bool,
}

impl GpuRayTracer {
    pub fn new(width: u32, height: u32, backend: GpuBackendType) -> Self {
        // Radne grupe od 16x16 niti po Compute Shader-u
        let wg_x = (width + 15) / 16;
        let wg_y = (height + 15) / 16;

        Self {
            backend,
            compute_workgroups: (wg_x, wg_y, 1),
            dispatched_passes: AtomicU64::new(0),
            is_pipeline_ready: true,
        }
    }

    /// Simulacija komandnog bafera koji dispatch-uje Compute Shader na GPU
    pub fn dispatch_compute(&mut self, sphere_count: usize) -> Result<u64, &'static str> {
        if !self.is_pipeline_ready {
            return Err("GPU_ERROR: Compute pipeline nije kompajliran ili izostaje WGSL/SPIR-V shader!");
        }

        self.dispatched_passes.fetch_add(1, Ordering::Relaxed);
        let total_threads = self.compute_workgroups.0 * self.compute_workgroups.1 * 256;

        Ok(total_threads as u64)
    }

    /// Prikazuje kod WGSL compute shader-a koji se izvršava direktno na GPU jezgrima
    pub fn get_embedded_wgsl_shader(&self) -> &'static str {
        r#"
struct Sphere {
    center_radius: vec4<f32>,
    color: vec4<f32>,
};

@group(0) @binding(0) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(1) var output_tex: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let uv = vec2<f32>(id.xy) / vec2<f32>(1920.0, 1080.0);
    // [GPU Paratrace logic u WGSL...]
    textureStore(output_tex, id.xy, vec4<f32>(uv.x, uv.y, 0.5, 1.0));
}
        "#
    }
}