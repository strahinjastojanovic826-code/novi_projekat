#[derive(Debug, Clone, PartialEq)]
pub enum VirtualDeviceType {
    Keyboard,
    Mouse,
    Gamepad,
    Touchscreen,
}

#[derive(Debug, Clone)]
pub enum InputEventType {
    KeyPress { key: String, pressed: bool },
    MouseMove { x: f32, y: f32, dx: f32, dy: f32 },
    MouseButton { button: String, pressed: bool },
    GamepadAxis { axis_x: f32, axis_y: f32 },
    GamepadButton { button: String, pressed: bool },
}

#[derive(Debug, Clone)]
pub struct VirtualInputEvent {
    pub id: u64,
    pub device_type: VirtualDeviceType,
    pub event_data: InputEventType,
    pub timestamp_ms: u64,
}