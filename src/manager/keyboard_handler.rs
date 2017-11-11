//! Handler for keyboards

use events::key_events::KeyEvent;
use libc;
use types::keyboard::KeyboardHandle;
use wlroots_sys::{wlr_event_keyboard_key, wlr_input_device};

pub trait KeyboardHandler {
    /// Callback that is triggered when a key is pressed.
    fn on_key(&mut self, &mut KeyboardHandle, &mut KeyEvent) {}
}

wayland_listener!(KeyboardWrapper, (KeyboardHandle, Box<KeyboardHandler>), [
    key_listener => key_notify: |this: &mut KeyboardWrapper, data: *mut libc::c_void,| unsafe {
        let (ref mut keyboard, ref mut keyboard_handler) = this.data;
        let xkb_state = (*keyboard.to_ptr()).xkb_state;
        let mut key = KeyEvent::new(data as *mut wlr_event_keyboard_key, xkb_state);

        keyboard_handler.on_key(keyboard, &mut key)
    };
]);

impl KeyboardWrapper {
    pub unsafe fn input_device(&self) -> *mut wlr_input_device {
        self.data.0.input_device()
    }
}
