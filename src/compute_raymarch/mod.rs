pub mod reversible;

// =============================================================================
// 1. 3D VECTOR MATH ZA COMPUTE SHADER
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    pub fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }

    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.00001 {
            self.scale(1.0 / len)
        } else {
            Self::new(0.0, 0.0, 0.0)
        }
    }
}

// =============================================================================
// 2. SIGNED DISTANCE FUNCTIONS (SDF MATEMATIKA)
// =============================================================================

pub struct SdfMath;

impl SdfMath {
    /// SDF za sferu na poziciji c sa poluprečnikom r
    pub fn sphere(p: Vec3, c: Vec3, r: f32) -> f32 {
        p.sub(c).length() - r
    }

    /// SDF za torus sa glavnim poluprečnikom R i debljinom r
    pub fn torus(p: Vec3, c: Vec3, r_major: f32, r_minor: f32) -> f32 {
        let local = p.sub(c);
        let q_x = (local.x * local.x + local.z * local.z).sqrt() - r_major;
        let q_y = local.y;
        (q_x * q_x + q_y * q_y).sqrt() - r_minor
    }

    /// Glatka unija (Smooth Minimum) dva objekta za organsko spajanje
    pub fn smooth_min(a: f32, b: f32, k: f32) -> f32 {
        let h = (0.5 + 0.5 * (b - a) / k).clamp(0.0, 1.0);
        (b * (1.0 - h) + a * h) - k * h * (1.0 - h)
    }
}

// =============================================================================
// 3. COMPUTE SHADER DISPATCHER & SPHERE TRACER
// =============================================================================

pub struct ComputeRaymarchEngine {
    pub width: usize,
    pub height: usize,
    pub max_steps: usize,
    pub epsilon: f32,
    pub max_dist: f32,
}

impl ComputeRaymarchEngine {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            max_steps: 64,
            epsilon: 0.001,
            max_dist: 20.0,
        }
    }

    /// Kombinovani SDF izračun kompletne 3D scene (Sfera + Torus)
    fn scene_sdf(&self, p: Vec3) -> f32 {
        let sphere_d = SdfMath::sphere(p, Vec3::new(-0.8, 0.0, 3.0), 0.9);
        let torus_d = SdfMath::torus(p, Vec3::new(0.8, 0.0, 3.0), 0.8, 0.3);
        SdfMath::smooth_min(sphere_d, torus_d, 0.5) // Organsko fuzionisanje!
    }

    /// Numeričko izračunavanje normale površine preko gradijenta (Finite Differences)
    fn estimate_normal(&self, p: Vec3) -> Vec3 {
        let e = self.epsilon;
        let nx = self.scene_sdf(Vec3::new(p.x + e, p.y, p.z)) - self.scene_sdf(Vec3::new(p.x - e, p.y, p.z));
        let ny = self.scene_sdf(Vec3::new(p.x, p.y + e, p.z)) - self.scene_sdf(Vec3::new(p.x, p.y - e, p.z));
        let nz = self.scene_sdf(Vec3::new(p.x, p.y, p.z + e)) - self.scene_sdf(Vec3::new(p.x, p.y, p.z - e));
        Vec3::new(nx, ny, nz).normalize()
    }

    /// Izvršavanje Compute Shader Dispatch-a (Simulacija Workgroup Parallel Execution)
    pub fn dispatch_compute(&self) -> (Vec<Vec<char>>, u64) {
        let mut buffer = vec![vec![' '; self.width]; self.height];
        let mut total_ray_steps = 0u64;

        let camera_pos = Vec3::new(0.0, 0.0, 0.0);
        let light_dir = Vec3::new(0.577, 0.577, -0.577).normalize(); // Lambertian svetlo

        let ascii_ramp = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

        // Simulacija paralele: Svaki piksel izračunava svoju nit (Thread ID)
        for y in 0..self.height {
            for x in 0..self.width {
                // Normalizovane koordinate ekrana [-1.0, 1.0]
                let uv_x = (x as f32 / self.width as f32) * 2.0 - 1.0;
                let uv_y = -((y as f32 / self.height as f32) * 2.0 - 1.0) * 0.5; // Aspect Ratio korekcija

                let ray_dir = Vec3::new(uv_x, uv_y, 1.0).normalize();

                // SPHERE TRACING LOOP
                let mut dist_traveled = 0.0f32;
                let mut hit = false;
                let mut hit_point = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.max_steps {
                    total_ray_steps += 1;
                    let current_p = camera_pos.add(ray_dir.scale(dist_traveled));
                    let d = self.scene_sdf(current_p);

                    if d < self.epsilon {
                        hit = true;
                        hit_point = current_p;
                        break;
                    }

                    dist_traveled += d;
                    if dist_traveled >= self.max_dist {
                        break;
                    }
                }

                if hit {
                    let normal = self.estimate_normal(hit_point);
                    let diffuse = normal.dot(light_dir).max(0.0); // Lambertian Diffuse
                    let char_idx = ((diffuse * (ascii_ramp.len() - 1) as f32) as usize)
                        .clamp(0, ascii_ramp.len() - 1);
                    buffer[y][x] = ascii_ramp[char_idx];
                }
            }
        }

        (buffer, total_ray_steps)
    }
}