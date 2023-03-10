use winapi::um::winuser;

use crate::virtual_key::Vk;

// Holds the structure that stores information needed by send_inputs.
#[derive(Clone)]
#[repr(transparent)]
pub struct Input(winuser::INPUT);

impl Input {
    pub fn new(vk: Vk, event: Event) -> Input {
        unsafe {
            let mut input: winuser::INPUT = std::mem::zeroed();
            input.type_ = winuser::INPUT_KEYBOARD;
            let ki = input.u.ki_mut();
            ki.wVk = vk as u16;
            ki.wScan = 0;
            ki.dwFlags = match event {
                Event::KeyDown => 0,
                Event::KeyUp => winuser::KEYEVENTF_KEYUP,
            };
            // The system will provide a timestamp
            ki.time = 0;

            Self(input)
        }
    }
}

// Send given inputs to the foreground window.
pub fn send_inputs<T: AsRef<[Input]>>(inputs: T) {
    use std::mem;

    unsafe {
        winuser::SendInput(
            inputs.as_ref().len() as _,
            inputs.as_ref().as_ptr() as _,
            mem::size_of::<winuser::INPUT>() as _,
        )
    };
}

// The event to sens for a given keycode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Event {
    KeyDown,
    KeyUp,
}
