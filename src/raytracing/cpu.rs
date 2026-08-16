use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn dot(self, rhs: Self) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 { Self::new(self.x / len, self.y / len, self.z / len) } else { self }
    }
}

impl core::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z) }
}

impl core::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z) }
}

impl core::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self { Self::new(self.x * rhs, self.y * rhs, self.z * rhs) }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color_rgb: (u8, u8, u8),
}

pub struct CpuRayTracer {
    pub width: usize,
    pub height: usize,
    pub rendered_rays: AtomicU64,
}

impl CpuRayTracer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            rendered_rays: AtomicU64::new(0),
        }
    }

    /// Matematika preseka zraka i sfere: t^2*b.b + 2t*b.(A-C) + (A-C).(A-C) - R^2 = 0
    fn hit_sphere(&self, sphere: &Sphere, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - sphere.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.001 { Some(t) } else { None }
        }
    }

    /// Generiše sliku u RGBA baferu piksel po piksel
    pub fn render_frame(&self, buffer: &mut [u32], spheres: &[Sphere]) {
        let aspect_ratio = self.width as f32 / self.height as f32;
        let origin = Vec3::new(0.0, 0.0, 0.0);

        for y in 0..self.height {
            for x in 0..self.width {
                // Normalizovane koordinate ekrana (-1.0 do 1.0)
                let u = (x as f32 / self.width as f32) * 2.0 - 1.0;
                let v = 1.0 - (y as f32 / self.height as f32) * 2.0;

                let dir = Vec3::new(u * aspect_ratio, v, -1.0).normalize();
                let ray = Ray { origin, direction: dir };

                self.rendered_rays.fetch_add(1, Ordering::Relaxed);

                let mut pixel_color = 0xFF101018; // Pozadinski tamni gradijent
                let mut closest_t = f32::MAX;

                for sphere in spheres {
                    if let Some(t) = self.hit_sphere(sphere, &ray) {
                        if t < closest_t {
                            closest_t = t;
                            
                            // Diffuse osvetljenje na osnovu normale u tački udara
                            let hit_point = ray.point_at(t);
                            let normal = (hit_point - sphere.center).normalize();
                            let light_dir = Vec3::new(1.0, 2.0, 1.0).normalize();
                            let intensity = normal.dot(light_dir).max(0.15); // Ambient low-light

                            let r = (sphere.color_rgb.0 as f32 * intensity) as u32;
                            let g = (sphere.color_rgb.1 as f32 * intensity) as u32;
                            let b = (sphere.color_rgb.2 as f32 * intensity) as u32;

                            pixel_color = (0xFF << 24) | (r << 16) | (g << 8) | b;
                        }
                    }
                }

                let idx = y * self.width + x;
                if idx < buffer.len() {
                    buffer[idx] = pixel_color;
                }
            }
        }
    }
}