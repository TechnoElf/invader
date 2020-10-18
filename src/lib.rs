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
