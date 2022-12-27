use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use winit::event::{VirtualKeyCode, WindowEvent};

#[derive(Debug)]
pub struct InputHelper {
    current_cursor_pos: (f32, f32),
    input_events: VecDeque<InputEvent>,
    state: HashMap<InputKeyType, Change>,
}

impl InputHelper {
    pub fn new() -> Self {
        Self {
            current_cursor_pos: (0.0, 0.0),
            input_events: VecDeque::new(),
            state: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, event: InputEvent) {
        self.input_events.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<InputEvent> {
        self.input_events.pop_front()
    }

    pub fn get_state(&self, key: InputKeyType) -> Change {
        match key {
            InputKeyType::Mouse(_) => return *self.state.get(&key).unwrap_or(&Change::Released),
            InputKeyType::Keyboard(code, virtual_code) => {
                if let Some(change) = self.state.get(&InputKeyType::Keyboard(code, virtual_code)) {
                    return *change;
                }
                if let Some(change) = self.state.get(&InputKeyType::Keyboard(code, None)) {
                    return *change;
                }
                Change::Released
            }
        }
    }

    // RETURNS TRUE IF IT HAS PARSED AN EVENT
    pub fn add_window_event(&mut self, window_event: &WindowEvent) -> bool {
        match window_event {
            WindowEvent::CursorMoved { position, .. } => {
                let _delta_x = position.x as f32 - self.current_cursor_pos.0;
                let _delta_y = position.y as f32 - self.current_cursor_pos.1;
                self.current_cursor_pos = (position.x as f32, position.y as f32);
                if let Some(InputEvent::CursorMoved { delta_x, delta_y }) =
                    self.input_events.back_mut()
                {
                    *delta_x += _delta_x;
                    *delta_y += _delta_y;
                    return true;
                }

                self.add_event(InputEvent::CursorMoved {
                    delta_x: _delta_x,
                    delta_y: _delta_y,
                });
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let time = Instant::now();
                let cursor_x = self.current_cursor_pos.0;
                let cursor_y = self.current_cursor_pos.1;
                let change = Change::from(state);
                let key_type = InputKeyType::Mouse(MouseButton::from(button));

                self.state.insert(key_type, change);
                self.add_event(InputEvent::KeyInput {
                    cursor_x,
                    cursor_y,
                    change,
                    key_type,
                    time,
                });
            }

            WindowEvent::KeyboardInput { input, .. } => {
                let time = Instant::now();
                let cursor_x = self.current_cursor_pos.0;
                let cursor_y = self.current_cursor_pos.1;
                let change = Change::from(&input.state);
                let key_type = InputKeyType::Keyboard(input.scancode, input.virtual_keycode);

                self.state.insert(InputKeyType::Keyboard(input.scancode, None), change);
                self.state.insert(key_type, change);
                self.add_event(InputEvent::KeyInput {
                    cursor_x,
                    cursor_y,
                    change,
                    key_type,
                    time,
                });
            }
            _ => return false,
        }
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    CursorMoved {
        delta_x: f32,
        delta_y: f32,
    },
    KeyInput {
        cursor_x: f32,
        cursor_y: f32,
        change: Change,
        key_type: InputKeyType,
        time: Instant,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InputKeyType {
    Mouse(MouseButton),
    Keyboard(u32, Option<VirtualKeyCode>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Change {
    Pressed,
    Released,
}

impl From<&winit::event::MouseButton> for MouseButton {
    fn from(m: &winit::event::MouseButton) -> Self {
        match m {
            winit::event::MouseButton::Left => Self::Left,
            winit::event::MouseButton::Right => Self::Right,
            winit::event::MouseButton::Middle => Self::Middle,
            winit::event::MouseButton::Other(x) => Self::Other(*x),
        }
    }
}

impl From<&winit::event::ElementState> for Change {
    fn from(e: &winit::event::ElementState) -> Self {
        match e {
            winit::event::ElementState::Pressed => Self::Pressed,
            winit::event::ElementState::Released => Self::Released,
        }
    }
}
