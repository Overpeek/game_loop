use super::input::Input;
use crate::event::Event;
use std::ops::{Deref, DerefMut};
use winit::event::{KeyboardInput, VirtualKeyCode, WindowEvent};

//

#[derive(Debug, Clone, Default)]
pub struct KeyboardState {
    inner: Input<VirtualKeyCode>,
}

//

impl KeyboardState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event(&mut self, event: &Event) {
        if let Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(virtual_keycode),
                            ..
                        },
                    ..
                },
            ..
        } = event
        {
            self.inner.event(*state, *virtual_keycode);
        }
    }
}

impl Deref for KeyboardState {
    type Target = Input<VirtualKeyCode>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for KeyboardState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
