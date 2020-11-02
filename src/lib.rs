#[macro_use]
pub mod misc;
pub mod render;
pub mod input;
pub mod physics;
pub mod net;
pub mod sound;

pub mod macros {
    pub use invader_macro::DefaultConstructor;
}

pub mod ecs {
    pub use specs::World;
    pub use specs::WorldExt;
    pub use specs::Builder;
    pub use specs::DispatcherBuilder;

    pub use specs::System;
    pub use specs::SystemData;
    pub use specs::Join;
    pub use specs::Entities;
    pub use specs::Write as WriteResource;
    pub use specs::Read as ReadResource;
    pub use specs::WriteStorage;
    pub use specs::ReadStorage;

    pub use specs::Component;
    pub use specs::NullStorage;
    pub use specs::VecStorage;
    pub use specs::DenseVecStorage;
}

use std::time::Instant;

use specs::*;

use misc::*;
use misc::persist::*;
use render::*;
use render::sdl::*;
use physics::*;
use net::*;
use net::imp::*;
use input::*;
use input::sdl::*;
use sound::*;
use sound::imp::*;

const TARGET_FRAME_RATE: f32 = 60.0;
const TARGET_FRAME_TIME: f32 = 1.0 / TARGET_FRAME_RATE;

pub struct InvaderBuilder<'a, 'b> {
    dispatcher: DispatcherBuilder<'a, 'b>,
    stage: String,
    render: SDLRenderImpl<'b>,
    input: SDLInputImpl
}

impl<'a, 'b> InvaderBuilder<'a, 'b> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();

        Self {
            dispatcher: DispatcherBuilder::new(),
            stage: String::new(),
            render: SDLRenderImpl::init(&sdl_context, Vector::new(800.0, 600.0).convert()),
            input: SDLInputImpl::init(&sdl_context)
        }
    }

    pub fn add_system(mut self, sys: impl for<'c> System<'c> + Send + 'a) -> Self {
        self.dispatcher = self.dispatcher.with(sys, "", &[]);
        self
    }

    pub fn set_stage(mut self, stage: &str) -> Self {
        self.stage = stage.to_string();
        self
    }

    pub fn add_sprite(mut self, key: &str, path: &str) -> Self {
        self.render.add_sprite(key, path);
        self
    }

    pub fn add_font(mut self, key: &str, path: &str, size: u16, color_r: u8, color_g: u8, color_b: u8) -> Self {
        self.render.add_font(key, path, size, color_r, color_g, color_b);
        self
    }

    pub fn build(self) -> Invader<'a, 'b> {
        let render_sys = RenderSys::new(self.render);
        let input_sys = InputSys::new(self.input);
        let network_sys = NetworkSyncSys::new(NetworkImp::new());
        let physics_sys = PhysicsSys::new();
        let sound_sys = SoundSys::new(SoundImp::new());
        let persist_sys = PersistSys::new();

        let mut dispatcher = self.dispatcher
            .with(persist_sys, "perist", &[])
            .with(network_sys, "network_sync", &[])
            .with(physics_sys, "physics", &["network_sync"])
            .with_thread_local(input_sys)
            .with_thread_local(sound_sys)
            .with_thread_local(render_sys)
            .build();

        let mut world = World::new();
        render::register(&mut world);
        misc::register(&mut world);
        dispatcher.setup(&mut world);

        world.insert(PhysicsRes::new());

        world.write_resource::<PersistRequestQueue>().push(PersistRequest::LoadStage(self.stage));

        Invader {
            world,
            dispatcher
        }
    }
}

pub struct Invader<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>
}

impl<'a, 'b> Invader<'a, 'b> {
    pub fn run(mut self) {
        let mut time = Instant::now();
        let mut delta_time;

        self.world.write_resource::<StateRes>().insert("app", AppState::Running);

        while self.world.read_resource::<StateRes>().get::<AppState>("app").unwrap() == &AppState::Running {
            // Calculate the time since the last frame and wait to lock to 60fps, if necessary
            delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
            while delta_time < TARGET_FRAME_TIME {
                delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
            }
            time = Instant::now();

            if delta_time > TARGET_FRAME_TIME * 2.0 {
                //println!("dt={}", delta_time);
                delta_time = 0.0;
            }

            self.world.write_resource::<PhysicsRes>().delta_time = delta_time;

            // Run the game
            self.dispatcher.dispatch(&mut self.world);
            self.world.maintain();
        }
    }
}
