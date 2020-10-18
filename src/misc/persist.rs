use std::fs::File;

use serde::{Serialize, Deserialize};

use nphysics2d::object::*;
use ncollide2d::shape::*;
use nalgebra::geometry::*;

use specs::*;
use specs::saveload::*;

use invader_macro::DefaultConstructor;
use crate::physics::{TransformCom, RigidBodyCom, ColliderCom, PhysicsRes};
use crate::render::{SpriteCom, TextCom};
use crate::misc::Vector;

event_queue! {
    PersistRequestQueue: pub enum PersistRequest {
        SaveStage(String),
        LoadStage(String)
    }
}

#[derive(DefaultConstructor)]
pub struct PersistSys;

impl<'a> System<'a> for PersistSys {
    type SystemData = (Entities<'a>,
        specs::Write<'a, PersistRequestQueue>,
        WriteStorage<'a, StageMarker>,
        specs::Write<'a, StageMarkerAllocator>,
        specs::Write<'a, PhysicsRes>,
        WriteStorage<'a, TransformCom>,
        WriteStorage<'a, SpriteCom>,
        WriteStorage<'a, TextCom>,
        WriteStorage<'a, RigidBodyCom>,
        WriteStorage<'a, ColliderCom>);

    fn run(&mut self, (entities, mut requests, mut stage_markers, mut stage_marker_alloc, mut physics, mut transforms, mut sprites, mut texts, mut bodies, mut colliders): Self::SystemData) {
        for request in requests.iter() {
            match request {
                PersistRequest::SaveStage(file) => {
                    let file = File::create(file).unwrap();

                    let mut elements: Vec<StageEntity> = Vec::new();
                    for (_marker, transform, sprite, text, body, collider) in (&stage_markers, (&transforms).maybe(), (&sprites).maybe(), (&texts).maybe(), (&bodies).maybe(), (&colliders).maybe()).join() {
                        elements.push(StageEntity {
                            transform: transform.map(|c| c.clone()),
                            sprite: sprite.map(|c| c.clone()),
                            text: text.map(|c| c.clone()),
                            body: body.map(|c| physics.read_rigid_body(c).unwrap().into()),
                            collider: collider.map(|c| physics.read_collider(c).unwrap().into())
                        });
                    }

                    bincode::serialize_into(&file, &elements).unwrap();
                },
                PersistRequest::LoadStage(file) => {
                    let file = match File::open(file) {
                        Ok(file) => file,
                        Err(_) => { println!("Could not open {}", file); continue; }
                    };

                    let elements: Vec<StageEntity> = bincode::deserialize_from(&file).unwrap();

                    for (_marker, entity) in (&stage_markers, &entities).join() {
                        transforms.remove(entity);
                        sprites.remove(entity);
                        bodies.get(entity).map(|body| physics.bodies.remove(body.0));
                        bodies.remove(entity);
                        colliders.get(entity).map(|collider| physics.colliders.remove(collider.0));
                        colliders.remove(entity);
                    }
                    stage_markers.clear();

                    for element in elements.iter() {
                        let entity = entities.create();

                        let mut rb: Option<DefaultBodyHandle> = None;

                        if let Some(transform) = &element.transform {
                            transforms.insert(entity, transform.clone()).unwrap();
                        }
                        if let Some(sprite) = &element.sprite {
                            sprites.insert(entity, sprite.clone()).unwrap();
                        }
                        if let Some(text) = &element.text {
                            texts.insert(entity, text.clone()).unwrap();
                        }
                        if let Some(body) = &element.body {
                            let com = physics.register_rigid_body(body.clone().into());
                            rb = Some(com.0);
                            bodies.insert(entity, com).unwrap();
                        }
                        if let Some(collider) = &element.collider {
                            if let Some(rb) = rb {
                                colliders.insert(entity, physics.register_collider(collider.clone().into_collider(&rb))).unwrap();
                            }
                        }

                        stage_marker_alloc.mark(entity, &mut stage_markers);
                    }
                }
            }
        }
        requests.clear();
    }
}

pub struct StageMarkerType;
pub type StageMarker = SimpleMarker<StageMarkerType>;
pub type StageMarkerAllocator = SimpleMarkerAllocator<StageMarkerType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StageEntity {
    transform: Option<TransformCom>,
    sprite: Option<SpriteCom>,
    text: Option<TextCom>,
    body: Option<PersistentRigidBody>,
    collider: Option<PersistentCollider>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistentRigidBody {
    #[serde(with = "BodyStatusDef")]
    status: BodyStatus
}

impl From<&RigidBody<f32>> for PersistentRigidBody {
    fn from(rb: &RigidBody<f32>) -> Self {
        Self {
            status: rb.status()
        }
    }
}

impl Into<RigidBody<f32>> for PersistentRigidBody {
    fn into(self) -> RigidBody<f32> {
        RigidBodyDesc::new().status(self.status.into()).mass(1.0).build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistentCollider {
    shape: ShapeDef,
    position: IsometryDef
}

impl From<&Collider<f32, DefaultBodyHandle>> for PersistentCollider {
    fn from(c: &Collider<f32, DefaultBodyHandle>) -> Self {
        Self {
            shape: c.shape_handle().into(),
            position: c.position_wrt_body().into()
        }
    }
}

impl PersistentCollider {
    fn into_collider(self, rb: &DefaultBodyHandle) -> Collider<f32, DefaultBodyHandle> {
        ColliderDesc::new(self.shape.into()).position(self.position.into()).build(BodyPartHandle(rb.clone(), 0))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "nphysics2d::object::BodyStatus")]
enum BodyStatusDef {
    Disabled,
    Static,
    Dynamic,
    Kinematic
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ShapeDef {
    Cuboid(CuboidDef),
    ConvexPolygon(ConvexPolygonDef)
}

impl From<&ShapeHandle<f32>> for ShapeDef {
    fn from(s: &ShapeHandle<f32>) -> Self {
        if s.is_shape::<Cuboid<f32>>() {
            ShapeDef::Cuboid(s.as_shape::<Cuboid<f32>>().unwrap().into())
        } else if s.is_shape::<ConvexPolygon<f32>>() {
            ShapeDef::ConvexPolygon(s.as_shape::<ConvexPolygon<f32>>().unwrap().into())
        } else {
            panic!("attempted to serialize unknown shape");
        }
    }
}

impl Into<ShapeHandle<f32>> for ShapeDef {
    fn into(self) -> ShapeHandle<f32> {
        match self {
            ShapeDef::Cuboid(s) => ShapeHandle::new::<Cuboid<f32>>(s.into()),
            ShapeDef::ConvexPolygon(s) => ShapeHandle::new::<ConvexPolygon<f32>>(s.into())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CuboidDef {
    half_extents: Vector
}

impl From<&Cuboid<f32>> for CuboidDef {
    fn from(c: &Cuboid<f32>) -> Self {
        Self {
            half_extents: c.half_extents.into()
        }
    }
}

impl Into<Cuboid<f32>> for CuboidDef {
    fn into(self) -> Cuboid<f32> {
        Cuboid::new(*self.half_extents)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConvexPolygonDef {
    points: Vec<Vector>
}

impl From<&ConvexPolygon<f32>> for ConvexPolygonDef {
    fn from(p: &ConvexPolygon<f32>) -> Self {
        Self {
            points: p.points().iter().map(|p| Vector::from(p.coords)).collect()
        }
    }
}

impl Into<ConvexPolygon<f32>> for ConvexPolygonDef {
    fn into(self) -> ConvexPolygon<f32> {
        ConvexPolygon::try_new(self.points.iter().map(|p| Point::from(p.0)).collect()).unwrap()
    }
}

type Isometryf = nphysics2d::math::Isometry<f32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IsometryDef {
    translation: Vector,
    rotation: f32
}

impl From<Isometryf> for IsometryDef {
    fn from(i: Isometryf) -> Self {
        Self {
            translation: i.translation.vector.into(),
            rotation: i.rotation.angle()
        }
    }
}

impl Into<Isometryf> for IsometryDef {
    fn into(self) -> Isometryf {
        Isometryf::from_parts((*self.translation).into(), UnitComplex::from_angle(self.rotation))
    }
}
