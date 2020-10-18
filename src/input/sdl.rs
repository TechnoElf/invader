use sdl2::*;
use sdl2::event::*;
use sdl2::keyboard::*;

use nalgebra::Vector2;

use crate::input::{InputEventQueue, InputEvent};
use crate::input::key::{KeysRes, Key};
use crate::misc::{StateRes, AppState};
use crate::render::CameraRes;

pub struct SDLInputImpl {
    context: SDLContext
}

impl SDLInputImpl {
    pub fn input(&mut self, state: &mut StateRes, camera: &mut CameraRes, keys: &mut KeysRes, input_queue: &mut InputEventQueue) {
        for event in self.context.events.poll_iter() {
            match event {
                Event::Quit {..} => state.insert("app", AppState::Stopping),
                Event::Window { win_event: e, .. } => {
                    match e {
                        WindowEvent::Resized(w, h) => camera.screen = Vector2::new(w as u32, h as u32),
                        _ => {}
                    }
                },
                Event::KeyDown { keycode: Some(k), .. } => {
                    keys.press(sdl_to_key(k));
                    input_queue.push(InputEvent::KeyDown(sdl_to_key(k)));
                },
                Event::KeyUp { keycode: Some(k), .. } => {
                    keys.release(sdl_to_key(k));
                    input_queue.push(InputEvent::KeyUp(sdl_to_key(k)));
                },
                Event::MouseButtonDown { x, y, .. } => input_queue.push(InputEvent::MouseDown(Vector2::new(x as u32, y as u32))),
                Event::MouseButtonUp { x, y, .. } => input_queue.push(InputEvent::MouseUp(Vector2::new(x as u32, y as u32))),
                _ => {}
            }
        }
    }

    pub fn init(sdl_context: &Sdl) -> Self {
        let events = sdl_context.event_pump().unwrap();

        let context = SDLContext {
            events: events
        };

        Self {
            context: context
        }
    } 
}

fn sdl_to_key(k: Keycode) -> Key {
    match k {
        Keycode::A => Key::A,
        Keycode::B => Key::B,
        Keycode::C => Key::C,
        Keycode::D => Key::D,
        Keycode::E => Key::E,
        Keycode::F => Key::F,
        Keycode::G => Key::G,
        Keycode::H => Key::H,
        Keycode::I => Key::I,
        Keycode::J => Key::J,
        Keycode::K => Key::K,
        Keycode::L => Key::L,
        Keycode::M => Key::M,
        Keycode::N => Key::N,
        Keycode::O => Key::O,
        Keycode::P => Key::P,
        Keycode::Q => Key::Q,
        Keycode::R => Key::R,
        Keycode::S => Key::S,
        Keycode::T => Key::T,
        Keycode::U => Key::U,
        Keycode::V => Key::V,
        Keycode::W => Key::W,
        Keycode::X => Key::X,
        Keycode::Y => Key::Y,
        Keycode::Z => Key::Z,
        Keycode::Num1 => Key::One,
        Keycode::Num2 => Key::Two,
        Keycode::Num3 => Key::Three,
        Keycode::Num4 => Key::Four,
        Keycode::Num5 => Key::Five,
        Keycode::Num6 => Key::Six,
        Keycode::Num7 => Key::Seven,
        Keycode::Num8 => Key::Eight,
        Keycode::Num9 => Key::Nine,
        Keycode::Num0 => Key::Zero,
        Keycode::Minus => Key::Minus,
        Keycode::Equals => Key::Equals,
        Keycode::LeftBracket => Key::OpenBracket,
        Keycode::RightBracket => Key::CloseBracket,
        Keycode::Backslash => Key::Backslash,
        Keycode::Semicolon => Key::Semicolon,
        Keycode::Quote => Key::Apostrophe,
        Keycode::Comma => Key::Comma,
        Keycode::Period => Key::Dot,
        Keycode::Slash => Key::Slash,
        Keycode::Backquote => Key::Backtick,
        Keycode::Backspace => Key::Backspace,
        Keycode::Tab => Key::Tab,
        Keycode::Return => Key::Return,
        Keycode::LShift => Key::Shift,
        Keycode::RShift => Key::Shift,
        Keycode::LCtrl => Key::Control,
        Keycode::RCtrl => Key::Control,
        Keycode::LAlt => Key::Opt,
        Keycode::RAlt => Key::Opt,
        Keycode::Space => Key::Space,
        Keycode::Up => Key::Up,
        Keycode::Down => Key::Down,
        Keycode::Left => Key::Left,
        Keycode::Right => Key::Right,
        _ => Key::Unknown
    }
}

struct SDLContext {
    events: EventPump,
}
