use std::fmt::{Debug, Formatter};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut};

use serde::{Serialize, Deserialize, Serializer, Deserializer, ser::SerializeTuple, de::Visitor, de::SeqAccess};

use num_traits::cast::{NumCast, cast};

use nalgebra::Vector2;

use specs::*;

use crate::misc::persist::{StageMarker, StageMarkerAllocator};

#[derive(Default)]
pub struct StateRes(HashMap<String, Box<dyn State>>);

pub trait State: Any + Send + Sync + 'static {
    fn get_type(&self) -> TypeId;
}

impl<T: Any + Send + Sync + 'static> State for T {
    fn get_type(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

#[allow(dead_code)]
impl StateRes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<T: State>(&mut self, name: &str, val: T) {
        self.0.insert(name.to_string(), Box::new(val));
    }

    pub fn get<T: State>(&self, name: &str) -> Option<&T> {
        self.0.get(name).map(Box::as_ref).and_then(|r| if r.get_type() == TypeId::of::<T>() {
            unsafe {
                Some(&*(r as *const dyn State as *const T))
            }
        } else {
            None
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppState {
    Running,
    Stopping
}

pub trait Convertable<U: NumCast + Debug + Copy + PartialEq + 'static> {
    fn convert(self) -> Vector2<U>;
}

impl<T: NumCast + Debug + Copy + PartialEq + 'static, U: NumCast + Debug + Copy + PartialEq + 'static> Convertable<U> for Vector2<T> {
    fn convert(self) -> Vector2<U> {
        Vector2::new(cast(self.x).unwrap(), cast(self.y).unwrap())
    }
}

#[macro_export]
macro_rules! event_queue {
    ($queue_name:ident : pub $type:tt $name:ident {$($element:tt)*}) => {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub $type $name {
            $($element)+
        }

        #[allow(dead_code)]
        #[derive(Default, Clone, Debug)]
        pub struct $queue_name {
            q: std::vec::Vec<$name>,
        }

        #[allow(dead_code)]
        impl $queue_name {
            pub fn new() -> Self {
                Self {
                    q: std::vec::Vec::new()
                }
            }

            pub fn push(&mut self, item: $name) {
                self.q.push(item);
            }

            pub fn iter(&self) -> std::slice::Iter<$name> {
                self.q.iter()
            }

            fn clear(&mut self) {
                self.q.clear();
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Vector(pub Vector2<f32>);

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector2::new(x, y))
    }
}

impl Serialize for Vector {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error> 
    where S: Serializer {
        let mut tup = ser.serialize_tuple(2)?;
        tup.serialize_element(&self.x)?;
        tup.serialize_element(&self.y)?;
        tup.end()
    }
}

struct VectorVisitor;

impl<'de> Visitor<'de> for VectorVisitor {
    type Value = (f32, f32);

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str("an (f32, f32) tuple")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> 
    where A: SeqAccess<'de> {
        Ok((seq.next_element()?.unwrap(), seq.next_element()?.unwrap()))
    }
}

impl<'de> Deserialize<'de> for Vector {
    fn deserialize<D>(de: D) -> Result<Self, D::Error> 
    where D: Deserializer<'de> {
        let tuple = de.deserialize_tuple(2, VectorVisitor)?;
        Ok(Self::new(tuple.0, tuple.1))
    }
}

impl Deref for Vector {
    type Target = Vector2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vector2<f32>> for Vector {
    fn from(v: Vector2<f32>) -> Self {
        Self(v)
    }
}

impl From<&Vector2<f32>> for Vector {
    fn from(v: &Vector2<f32>) -> Self {
        Self(v.clone())
    }
}

impl Into<Vector2<f32>> for Vector {
    fn into(self) -> Vector2<f32> {
        *self
    }
}

pub fn register(world: &mut World) {
    world.insert(StateRes::new());
    world.insert(StageMarkerAllocator::new());
    world.register::<StageMarker>();
}

pub mod persist;
