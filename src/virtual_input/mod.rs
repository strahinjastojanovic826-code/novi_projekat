pub mod device;

use device::{InputEventType, VirtualDeviceType, VirtualInputEvent};

pub struct QuantumVirtualInputEngine {
    // Stanje miša
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_left_down: bool,
    pub mouse_right_down: bool,

    // Stanje tastature
    pub last_key_pressed: String,
    pub shift_active: bool,

    // Stanje Gamepad-a
    pub gamepad_axis_x: f32,
    pub gamepad_axis_y: f32,
    pub gamepad_btn_a: bool,
    pub gamepad_btn_b: bool,

    // Statistika i bafer događaja
    pub events_queue: Vec<VirtualInputEvent>,
    pub total_events_count: u64,
    pub logs: Vec<String>,
}

impl QuantumVirtualInputEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            mouse_x: 400.0,
            mouse_y: 300.0,
            mouse_left_down: false,
            mouse_right_down: false,
            last_key_pressed: "None".into(),
            shift_active: false,
            gamepad_axis_x: 0.0,
            gamepad_axis_y: 0.0,
            gamepad_btn_a: false,
            gamepad_btn_b: false,
            events_queue: Vec::new(),
            total_events_count: 0,
            logs: Vec::new(),
        };

        engine.logs.push("Virtual Input Engine Inicijalizovan [IRQ 1, IRQ 12 emulacija aktivna].".into());
        engine
    }

    pub fn inject_key(&mut self, key: &str, pressed: bool) {
        self.total_events_count += 1;
        self.last_key_pressed = key.to_string();

        let evt = VirtualInputEvent {
            id: self.total_events_count,
            device_type: VirtualDeviceType::Keyboard,
            event_data: InputEventType::KeyPress {
                key: key.to_string(),
                pressed,
            },
            timestamp_ms: self.total_events_count * 10,
        };

        self.events_queue.push(evt);
        self.logs.push(format!("Tastatura [IRQ 1]: Taster '{}' -> {}", key, if pressed { "Pritisnut" } else { "Pušten" }));
    }

    pub fn move_mouse(&mut self, new_x: f32, new_y: f32) {
        let dx = new_x - self.mouse_x;
        let dy = new_y - self.mouse_y;
        self.mouse_x = new_x;
        self.mouse_y = new_y;

        self.total_events_count += 1;
        let evt = VirtualInputEvent {
            id: self.total_events_count,
            device_type: VirtualDeviceType::Mouse,
            event_data: InputEventType::MouseMove {
                x: new_x,
                y: new_y,
                dx,
                dy,
            },
            timestamp_ms: self.total_events_count * 10,
        };

        self.events_queue.push(evt);
    }

    pub fn click_mouse_button(&mut self, button: &str, pressed: bool) {
        if button == "left" {
            self.mouse_left_down = pressed;
        } else if button == "right" {
            self.mouse_right_down = pressed;
        }

        self.total_events_count += 1;
        let evt = VirtualInputEvent {
            id: self.total_events_count,
            device_type: VirtualDeviceType::Mouse,
            event_data: InputEventType::MouseButton {
                button: button.to_string(),
                pressed,
            },
            timestamp_ms: self.total_events_count * 10,
        };

        self.events_queue.push(evt);
        self.logs.push(format!("Miš [IRQ 12]: Dugme '{}' -> {}", button, if pressed { "Kliknuto" } else { "Otpušteno" }));
    }

    pub fn update_gamepad(&mut self, axis_x: f32, axis_y: f32, btn_a: bool, btn_b: bool) {
        self.gamepad_axis_x = axis_x;
        self.gamepad_axis_y = axis_y;
        self.gamepad_btn_a = btn_a;
        self.gamepad_btn_b = btn_b;

        self.total_events_count += 1;
        let evt = VirtualInputEvent {
            id: self.total_events_count,
            device_type: VirtualDeviceType::Gamepad,
            event_data: InputEventType::GamepadAxis { axis_x, axis_y },
            timestamp_ms: self.total_events_count * 10,
        };

        self.events_queue.push(evt);
    }

    pub fn clear_queue(&mut self) {
        self.events_queue.clear();
        self.logs.push("Očišćen bafer ulaznih događaja.".into());
    }
}