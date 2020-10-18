pub mod key;
pub mod sdl;

use nalgebra::Vector2;

use specs::prelude::*;

use invader_macro::DefaultConstructor;
use crate::input::key::{KeysRes, Key};
use crate::misc::StateRes;
use crate::render::CameraRes;
use crate::input::sdl::SDLInputImpl;

event_queue! {
    InputEventQueue: pub enum InputEvent {
        KeyDown(Key),
        KeyUp(Key),
        MouseDown(Vector2<u32>),
        MouseUp(Vector2<u32>),
    }
}

#[derive(DefaultConstructor)]
pub struct InputSys {
    input: SDLInputImpl
}

impl<'a> System<'a> for InputSys {
    type SystemData = (WriteExpect<'a, StateRes>,
        Write<'a, InputEventQueue>,
        Write<'a, CameraRes>,
        Write<'a, KeysRes>);

    fn run(&mut self, (mut state, mut input_queue, mut camera, mut keys): Self::SystemData) {
        input_queue.clear();
        self.input.input(&mut state, &mut camera, &mut keys, &mut input_queue);
    }
}
