use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type Entity = u32;

// =============================================================================
// 1. KOMPONENTE (Čisti podaci za QuantumOS Entitete)
// =============================================================================

#[derive(Debug, Clone)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct VelocityComponent {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Clone)]
pub struct ProcessTaskComponent {
    pub pid: u32,
    pub name: String,
    pub priority: u8,
    pub cpu_ticks: u64,
}

#[derive(Debug, Clone)]
pub struct RenderComponent {
    pub symbol: char,
    pub color_rgb: u32,
}

// =============================================================================
// 2. ECS WORLD & STORAGE ENGINE
// =============================================================================

pub struct World {
    next_entity: Entity,
    pub alive_entities: Vec<Entity>,
    // Mapiranje: TypeId komponente -> Mapiranje (Entity -> Skladište komponente)
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity: 0,
            alive_entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    /// Pravi novi Entitet i vraća njegov jedinstveni ID
    pub fn spawn(&mut self) -> Entity {
        let entity = self.next_entity;
        self.next_entity += 1;
        self.alive_entities.push(entity);
        entity
    }

    /// Briše entitet i sve njegove prikačene komponente
    pub fn despawn(&mut self, entity: Entity) {
        if let Some(pos) = self.alive_entities.iter().position(|&e| e == entity) {
            self.alive_entities.swap_remove(pos);
            for storage in self.components.values_mut() {
                storage.remove(&entity);
            }
        }
    }

    /// Kači komponentu bilo kog tipa na zadati entitet
    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        self.components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(component));
    }

    /// Dohvata referencu na komponentu entiteta (Immutable)
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let storage = self.components.get(&type_id)?;
        let boxed = storage.get(&entity)?;
        boxed.downcast_ref::<T>()
    }

    /// Dohvata izmenjivu referencu na komponentu entiteta (Mutable)
    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let storage = self.components.get_mut(&type_id)?;
        let boxed = storage.get_mut(&entity)?;
        boxed.downcast_mut::<T>()
    }

    /// Proverava da li entitet poseduje određenu komponentu
    pub fn has_component<T: 'static>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.components.get(&type_id) {
            storage.contains_key(&entity)
        } else {
            false
        }
    }

    /// Vraća listu svih entiteta koji poseduju zadati tip komponente
    pub fn query_entities<T: 'static>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.components.get(&type_id) {
            storage.keys().copied().collect()
        } else {
            Vec::new()
        }
    }
}

// =============================================================================
// 3. GLAVNI QUANTUM ECS ENGINE & SYSTEM LOGIC
// =============================================================================

pub struct QuantumEcsEngine {
    pub world: World,
    pub system_ticks: u64,
}

impl QuantumEcsEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            world: World::new(),
            system_ticks: 0,
        };

        engine.seed_demo_entities();
        engine
    }

    /// Popunjava ECS svet početnim entitetima (procesi, GUI elementi, objekti)
    pub fn seed_demo_entities(&mut self) {
        // Entitet 1: Prozor ili ikonica sa pozicijom i brzinom
        let icon = self.world.spawn();
        self.world.add_component(icon, PositionComponent { x: 100.0, y: 150.0 });
        self.world.add_component(icon, VelocityComponent { dx: 1.5, dy: -0.5 });
        self.world.add_component(icon, RenderComponent { symbol: '🖵', color_rgb: 0x00FF88 });

        // Entitet 2: Pozadinski kernel zadatak
        let process = self.world.spawn();
        self.world.add_component(
            process,
            ProcessTaskComponent {
                pid: 1042,
                name: "quantum_daemon".to_string(),
                priority: 1,
                cpu_ticks: 0,
            },
        );

        // Entitet 3: Druga ikonica u pokretu
        let widget = self.world.spawn();
        self.world.add_component(widget, PositionComponent { x: 0.0, y: 0.0 });
        self.world.add_component(widget, VelocityComponent { dx: 2.0, dy: 2.0 });
        self.world.add_component(widget, RenderComponent { symbol: '⚙', color_rgb: 0xFF5500 });
    }

    // --- SISTEMI (Logika koja se izvršava u svakom frame-u / tick-u) ---

    /// Movement System: Ažurira poziciju na osnovu brzine za sve relevantne entitete
    pub fn run_movement_system(&mut self, delta_time: f32) {
        let entities = self.world.alive_entities.clone();
        for entity in entities {
            if self.world.has_component::<PositionComponent>(entity)
                && self.world.has_component::<VelocityComponent>(entity)
            {
                let (dx, dy) = {
                    let vel = self.world.get_component::<VelocityComponent>(entity).unwrap();
                    (vel.dx, vel.dy)
                };

                if let Some(pos) = self.world.get_component_mut::<PositionComponent>(entity) {
                    pos.x += dx * delta_time;
                    pos.y += dy * delta_time;
                }
            }
        }
    }

    /// Scheduler/Process System: Povećava CPU cikluse procesima
    pub fn run_process_scheduler_system(&mut self) {
        let entities = self.world.query_entities::<ProcessTaskComponent>();
        for entity in entities {
            if let Some(task) = self.world.get_component_mut::<ProcessTaskComponent>(entity) {
                task.cpu_ticks += 10;
            }
        }
    }

    /// Okida sve sisteme u jednom tick-u
    pub fn step(&mut self, delta_time: f32) {
        self.system_ticks += 1;
        self.run_movement_system(delta_time);
        self.run_process_scheduler_system();
    }
}