// =============================================================================
// 1. VEKTORSKA I SCENSKA MATEMATIKA
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

    pub fn sub(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
            Vec3::new(self.x / len, self.y / len, self.z / len)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

// =============================================================================
// 2. MESHLET STRUKTURA (Mikro-grozdovi geometrije)
// =============================================================================

/// Predstavlja mali grozd trouglova (npr. max 64 verteksa, 126 trouglova u realnom GPU-u)
#[derive(Debug, Clone)]
pub struct Meshlet {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<(usize, usize, usize)>, // Trouglovi unutar meshleta
    pub bounding_center: Vec3,
    pub bounding_radius: f32,
    pub cone_normal: Vec3,                   // Normala cele grupe (za Backface Culling)
}

// =============================================================================
// 3. TASK SHADER & MESH SHADER PIPELINE
// =============================================================================

pub struct TaskShaderPayload {
    pub visible_meshlet_indices: Vec<usize>,
}

pub struct VirtualizedGeometryEngine;

impl VirtualizedGeometryEngine {
    /// Deljenje velike geometrije na Meshlet grozdove (Meshletization)
    pub fn build_meshlets(all_vertices: &[Vec3], triangles: &[(usize, usize, usize)]) -> Vec<Meshlet> {
        let mut meshlets = Vec::new();
        let meshlet_size = 4; // Zbog demonstracije, po 4 trougla po grozdu

        for chunk in triangles.chunks(meshlet_size) {
            let mut local_vertices = Vec::new();
            let mut local_indices = Vec::new();
            let mut center = Vec3::new(0.0, 0.0, 0.0);

            for &(i1, i2, i3) in chunk {
                let v1 = all_vertices[i1];
                let v2 = all_vertices[i2];
                let v3 = all_vertices[i3];

                local_vertices.push(v1);
                local_vertices.push(v2);
                local_vertices.push(v3);

                let len = local_vertices.len();
                local_indices.push((len - 3, len - 2, len - 1));

                center = Vec3::new(
                    center.x + (v1.x + v2.x + v3.x) / 3.0,
                    center.y + (v1.y + v2.y + v3.y) / 3.0,
                    center.z + (v1.z + v2.z + v3.z) / 3.0,
                );
            }

            let num_tris = chunk.len() as f32;
            center = Vec3::new(center.x / num_tris, center.y / num_tris, center.z / num_tris);

            // Računanje Bounding Radius-a grozda
            let mut max_r: f32 = 0.0;
            for v in &local_vertices {
                let dist = v.sub(center).length();
                if dist > max_r {
                    max_r = dist;
                }
            }

            // Simulacija zbirne normale grozda za Backface Cluster Culling
            let cone_normal = Vec3::new(0.0, 0.0, -1.0);

            meshlets.push(Meshlet {
                vertices: local_vertices,
                indices: local_indices,
                bounding_center: center,
                bounding_radius: max_r,
                cone_normal,
            });
        }

        meshlets
    }

    /// TASK SHADER: Izvršava se po grozdu (Cluster Level Culling)
    /// Odbacuje cele grozdove koji su izvan vidnog polja ili okrenuti leđima
    pub fn run_task_shader(
        meshlets: &[Meshlet],
        camera_pos: Vec3,
        camera_dir: Vec3,
    ) -> TaskShaderPayload {
        let mut visible_indices = Vec::new();

        for (idx, meshlet) in meshlets.iter().enumerate() {
            let to_cluster = meshlet.bounding_center.sub(camera_pos);
            let dist = to_cluster.length();

            // 1. Frustum Culling (Da li je grozd iza kamere?)
            let view_dot = to_cluster.normalize().dot(camera_dir);
            if view_dot < 0.2 && dist > meshlet.bounding_radius {
                continue; // Odbaci ceo grozd!
            }

            // 2. Cluster Backface Culling (Da li ceo grozd gleda od kamere?)
            let backface_dot = meshlet.cone_normal.dot(camera_dir);
            if backface_dot > 0.6 {
                continue; // Odbaci! Ne vidimo zadnju stranu objekta
            }

            visible_indices.push(idx);
        }

        TaskShaderPayload {
            visible_meshlet_indices: visible_indices,
        }
    }

    /// MESH SHADER: Pokreće se SAMO za grozdove koji su prošli Task Shader
    /// Generiše konačne primitive za rasterizaciju
    pub fn run_mesh_shader(meshlet: &Meshlet) -> usize {
        // Obrađuje temena i indeksnu listu direktno u lokalnoj memoriji GPU-a
        meshlet.indices.len()
    }
}