use serde::{Serialize, Deserialize};

use specs::*;
use specs::storage::ComponentEvent;

use nalgebra::{Isometry2, Vector2, Point};
use nphysics2d::object::*;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Shape, Cuboid, ConvexPolygon};

pub use nphysics2d::math::{Force, ForceType};
pub use nphysics2d::object::Body;

use invader_macro::DefaultConstructor;

use crate::misc::Vector;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct RigidBodyCom(pub DefaultBodyHandle);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ColliderCom(pub DefaultColliderHandle);

#[derive(Debug, Clone, DefaultConstructor, Serialize, Deserialize)]
pub struct TransformCom {
    pub pos: Vector,
}

impl Component for TransformCom {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Default for RigidBodyCom {
    fn default() -> Self {
        Self(DefaultBodyHandle::from_raw_parts(0, 0))
    }
}

pub struct PhysicsSys {
    external_transforms: BitSet,
    transform_event_reader: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for PhysicsSys {
    type SystemData = (Write<'a, PhysicsRes>,
        WriteStorage<'a, TransformCom>,
        ReadStorage<'a, RigidBodyCom>,
        ReadStorage<'a, ColliderCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut physics, mut transforms, rigid_bodies, _colliders) = data;
        let delta_time = physics.delta_time;

        self.external_transforms.clear();
        let transform_events = transforms.channel().read(self.transform_event_reader.as_mut().unwrap());
        for event in transform_events {
            match event {
                ComponentEvent::Inserted(id) => { self.external_transforms.add(*id); },
                ComponentEvent::Modified(id) => { self.external_transforms.add(*id); },
                ComponentEvent::Removed(_) => ()
            }
        }
        for (transform, rigid_body, _) in (&transforms, &rigid_bodies, &self.external_transforms).join() {
            match physics.write_rigid_body(rigid_body) {
                Some(rb) => rb.set_position(Isometry2::new(*transform.pos, 0.0)),
                None => ()
            }
        }

        physics.m_world.set_timestep(delta_time);

        // All components are part of the same struct, so this should be safe
        unsafe {
            let m_world: *mut DefaultMechanicalWorld<f32> = &mut physics.m_world;
            let g_world: *mut DefaultGeometricalWorld<f32> = &mut physics.g_world;
            let bodies: *mut DefaultBodySet<f32> = &mut physics.bodies;
            let colliders: *mut DefaultColliderSet<f32> = &mut physics.colliders;
            let constraints: *mut DefaultJointConstraintSet<f32> = &mut physics.constraints;
            let forces: *mut DefaultForceGeneratorSet<f32> = &mut physics.forces;
            (*m_world).step(&mut *g_world, &mut *bodies, &mut *colliders, &mut *constraints, &mut *forces);
        }

        for (transform, rigid_body) in (&mut transforms, &rigid_bodies).join() {
            match physics.read_rigid_body(rigid_body) {
                Some(rb) => transform.pos = Vector(rb.position().translation.vector),
                None => ()
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.transform_event_reader = Some(world.write_component::<TransformCom>().register_reader());
    }
}

impl PhysicsSys {
    pub fn new() -> Self {
        Self {
            external_transforms: BitSet::new(),
            transform_event_reader: None
        }
    }
}

pub struct PhysicsRes {
    pub delta_time: f32,
    pub m_world: DefaultMechanicalWorld<f32>,
    pub g_world: DefaultGeometricalWorld<f32>,
    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    pub constraints: DefaultJointConstraintSet<f32>,
    pub forces: DefaultForceGeneratorSet<f32>
}

impl PhysicsRes {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for PhysicsRes {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            m_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            g_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            constraints: DefaultJointConstraintSet::new(),
            forces: DefaultForceGeneratorSet::new()
        }
    }
}

#[allow(dead_code)]
impl PhysicsRes {
    pub fn register_rigid_body(&mut self, rb: RigidBody<f32>) -> RigidBodyCom {
        RigidBodyCom(self.bodies.insert(rb))
    }

    pub fn register_collider(&mut self, c: Collider<f32, DefaultBodyHandle>) -> ColliderCom {
        ColliderCom(self.colliders.insert(c))
    }

    pub fn create_rigid_body(&mut self) -> RigidBodyCom {
        RigidBodyCom(self.bodies.insert(RigidBodyDesc::new().mass(1.0).build()))
    }

    pub fn create_rigid_body_static(&mut self) -> RigidBodyCom {
        RigidBodyCom(self.bodies.insert(RigidBodyDesc::new().status(BodyStatus::Static).build()))
    }

    pub fn create_collider(&mut self, shape: impl Shape<f32>, rb: &RigidBodyCom) -> ColliderCom {
        ColliderCom(self.colliders.insert(ColliderDesc::new(ShapeHandle::new(shape)).build(BodyPartHandle(rb.0, 0))))
    }

    pub fn create_collider_rectangle(&mut self, dim: Vector, offset: Vector, rb: &RigidBodyCom) -> ColliderCom {
        ColliderCom(self.colliders.insert(ColliderDesc::new(ShapeHandle::new(Cuboid::new(*dim / 2.0))).position(Isometry2::new(*dim / 2.0 + *offset, 0.0)).build(BodyPartHandle(rb.0, 0))))
    }

    pub fn create_collider_triangle(&mut self, point0: Vector, point1: Vector, point2: Vector, rb: &RigidBodyCom) -> ColliderCom {
        ColliderCom(self.colliders.insert(ColliderDesc::new(ShapeHandle::new(ConvexPolygon::try_new(vec![Point::from(*point0), Point::from(*point1), Point::from(*point2)]).unwrap())).position(Isometry2::new(Vector2::new(0.0, 0.0), 0.0)).build(BodyPartHandle(rb.0, 0))))
    }

    pub fn write_rigid_body(&mut self, rb: &RigidBodyCom) -> Option<&mut RigidBody<f32>> {
        self.bodies.rigid_body_mut(rb.0)
    }

    pub fn read_rigid_body(&self, rb: &RigidBodyCom) -> Option<&RigidBody<f32>> {
        self.bodies.rigid_body(rb.0)
    }

    pub fn read_collider(&self, c: &ColliderCom) -> Option<&Collider<f32, DefaultBodyHandle>> {
        self.colliders.get(c.0)
    }
}
