// =============================================================================
// 1. VEKTORSKA MATEMATIKA ZA 3D PROSTOR (Vec3)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn add(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    pub fn sub(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    pub fn scale(&self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.00001 {
            self.scale(1.0 / len)
        } else {
            Vec3::zero()
        }
    }

    pub fn abs(&self) -> Vec3 {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn max_elem(&self, val: f32) -> Vec3 {
        Vec3::new(self.x.max(val), self.y.max(val), self.z.max(val))
    }
}

// =============================================================================
// 2. MATEMATIČKE PRIMITIVE I CSG (Constructive Solid Geometry)
// =============================================================================

pub struct SdfPrimitives;

impl SdfPrimitives {
    /// SDF za Sferu sa centrom u c i poluprečnikom r: $d(\mathbf{p}) = \|\mathbf{p} - \mathbf{c}\| - r$
    pub fn sphere(p: Vec3, center: Vec3, radius: f32) -> f32 {
        p.sub(center).length() - radius
    }

    /// SDF za 3D Kocku/Kvadar
    pub fn box_exact(p: Vec3, center: Vec3, extents: Vec3) -> f32 {
        let q = p.sub(center).abs().sub(extents);
        let outside = q.max_elem(0.0).length();
        let inside = q.x.max(q.y.max(q.z)).min(0.0);
        outside + inside
    }

    /// Smooth Union (Blagi matematički spoj dva objekta kao živa živa/metaballs)
    pub fn smooth_union(d1: f32, d2: f32, k: f32) -> f32 {
        let h = (0.5 + 0.5 * (d2 - d1) / k).clamp(0.0, 1.0);
        (d2 * (1.0 - h) + d1 * h) - k * h * (1.0 - h)
    }
}

// =============================================================================
// 3. SCENA I RAYMARCHING ENGINE
// =============================================================================

pub struct SdfScene;

impl SdfScene {
    /// Definise celokupnu 3D scenu kroz spoj SDF funkcija
    pub fn distance(p: Vec3) -> f32 {
        // Sfera #1 na poziciji (-1.2, 0.0, 5.0)
        let s1 = SdfPrimitives::sphere(p, Vec3::new(-1.2, 0.0, 5.0), 1.0);
        // Sfera #2 na poziciji (0.8, 0.2, 4.8)
        let s2 = SdfPrimitives::sphere(p, Vec3::new(0.8, 0.2, 4.8), 0.8);
        // Spajamo dve sfere u organski oblik pomoću smooth union-a!
        let spheres = SdfPrimitives::smooth_union(s1, s2, 0.6);

        // Ravna podloga (Pod) na Y = -1.0
        let floor = p.y - (-1.0);

        spheres.min(floor)
    }

    /// Izračunava normalu površine u tački P pomoću gradijenta (Konačne razlike):
    /// $\mathbf{n} \approx \nabla f(\mathbf{p})$
    pub fn calculate_normal(p: Vec3) -> Vec3 {
        let eps = 0.001;
        let dx = Self::distance(Vec3::new(p.x + eps, p.y, p.z)) - Self::distance(Vec3::new(p.x - eps, p.y, p.z));
        let dy = Self::distance(Vec3::new(p.x, p.y + eps, p.z)) - Self::distance(Vec3::new(p.x, p.y - eps, p.z));
        let dz = Self::distance(Vec3::new(p.x, p.y, p.z + eps)) - Self::distance(Vec3::new(p.x, p.y, p.z - eps));

        Vec3::new(dx, dy, dz).normalize()
    }
}

// =============================================================================
// 4. LUMEN-LIKE GLOBAL ILLUMINATION & SDF AMBIENT OCCLUSION (AO)
// =============================================================================

pub struct QuantumLumenGiEngine;

impl QuantumLumenGiEngine {
    /// Izračunava SDF Ambient Occlusion u tački p u smeru normale
    pub fn compute_sdf_ao(p: Vec3, normal: Vec3) -> f32 {
        let mut occ = 0.0;
        let mut sca = 1.0;

        // Proveravamo 5 uzoraka duž normale u prostoru
        for i in 0..5 {
            let h = 0.01 + 0.12 * (i as f32);
            let sample_pos = p.add(normal.scale(h));
            let d = SdfScene::distance(sample_pos);

            occ += (h - d) * sca;
            sca *= 0.95;
        }

        (1.0 - 1.5 * occ).clamp(0.0, 1.0)
    }

    /// Računa indirektno osvetljenje (Global Illumination & Soft Shadows)
    pub fn compute_gi(p: Vec3, normal: Vec3, light_pos: Vec3) -> f32 {
        let light_dir = light_pos.sub(p).normalize();
        
        // 1. Direktno Lambertian osvetljenje
        let diff = normal.dot(light_dir).max(0.0);

        // 2. Meke senke preko Raymarching-a ka svetlu
        let mut shadow = 1.0_f32;
        let mut t = 0.02;
        while t < 10.0 {
            let h = SdfScene::distance(p.add(light_dir.scale(t)));
            if h < 0.001 {
                shadow = 0.0;
                break;
            }
            shadow = shadow.min(8.0 * h / t);
            t += h;
        }

        // 3. SDF Ambient Occlusion za indirektne senke u ugradnim delovima
        let ao = Self::compute_sdf_ao(p, normal);

        // Indirektno svetlo (SDF GI Bounce Simulation)
        let sky_bounce = (normal.y * 0.5 + 0.5) * 0.2; // Indirektno svetlo sa neba
        
        (diff * shadow + sky_bounce) * ao
    }

    /// Izvršava Raymarching korak kroz scenu sa kamere
    pub fn march(ro: Vec3, rd: Vec3) -> Option<(Vec3, f32)> {
        let mut t = 0.0;
        let max_t = 20.0;

        for _ in 0..100 {
            let p = ro.add(rd.scale(t));
            let d = SdfScene::distance(p);

            if d < 0.001 {
                return Some((p, t)); // Udario u površinu!
            }

            t += d;
            if t >= max_t {
                break;
            }
        }

        None // Zrak je otišao u beskonačnost (Nebo)
    }
}