pub mod imp;
pub mod packet;

use std::net::SocketAddr;

use specs::*;
use specs::world::Builder;

use invader_macro::DefaultConstructor;
use crate::render::SpriteCom;
use crate::net::packet::*;
use crate::physics::{PhysicsRes, ColliderCom, RigidBodyCom, TransformCom};
use crate::net::imp::NetworkImp;
use crate::misc::Vector;

event_queue! {
    NetworkEventQueue: pub enum NetworkEvent {
        PeerConnected(NetID),
        PeerDisconnected(NetID),
        PeerMoved(NetID, TransformCom)
    }
}

event_queue! {
    NetworkRequestQueue: pub enum NetworkRequest {
        Open,
        Close,
        Connect(SocketAddr),
        Debug
    }
}

#[derive(DefaultConstructor)]
pub struct NetworkSyncSys {
    imp: NetworkImp
}

impl<'a> System<'a> for NetworkSyncSys {
    type SystemData = (Entities<'a>,
        Read<'a, LazyUpdate>,
        Write<'a, NetworkEventQueue>,
        Write<'a, NetworkRequestQueue>,
        Write<'a, PhysicsRes>,
        ReadStorage<'a, NetMasterTransformCom>,
        ReadStorage<'a, NetSlaveTransformCom>,
        WriteStorage<'a, TransformCom>,
        ReadStorage<'a, RigidBodyCom>,
        ReadStorage<'a, ColliderCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, updater, mut net_events, mut net_requests, mut physics, master_transform_flags, slave_transform_flags, mut transforms, rigid_bodies, colliders) = data;

        for request in net_requests.iter() {
            match request {
                NetworkRequest::Open => {
                   self.imp.close();
                    for (entity, _, rb, col) in (&entities, &slave_transform_flags, &rigid_bodies, &colliders).join() {
                        physics.bodies.remove(rb.0);
                        physics.colliders.remove(col.0);
                        updater.remove::<SpriteCom>(entity);
                        updater.remove::<TransformCom>(entity);
                        updater.remove::<NetSlaveTransformCom>(entity);
                        updater.remove::<RigidBodyCom>(entity);
                        updater.remove::<ColliderCom>(entity);
                    } 
                    self.imp.open().unwrap();
                },
                NetworkRequest::Close => {
                    self.imp.close();
                    for (entity, _, rb, col) in (&entities, &slave_transform_flags, &rigid_bodies, &colliders).join() {
                        physics.bodies.remove(rb.0);
                        physics.colliders.remove(col.0);
                        updater.remove::<SpriteCom>(entity);
                        updater.remove::<TransformCom>(entity);
                        updater.remove::<NetSlaveTransformCom>(entity);
                        updater.remove::<RigidBodyCom>(entity);
                        updater.remove::<ColliderCom>(entity);
                    }
                },
                NetworkRequest::Connect(addr) => match self.imp.connect(addr) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("could not connect to remote client: {}", e);
                        self.imp.close();
                    }
                },
                NetworkRequest::Debug => println!("Data for client {}:\n  open: {}\n  hosting: {}\n  host: {}\n  socket: {}\n  peers: {:?}", self.imp.id, self.imp.socket.is_some(), !self.imp.host_id.is_shared(), self.imp.host_id, self.imp.socket.as_ref().map(|s| format!("{:?}", s)).or(Some("".to_string())).unwrap(), self.imp.peers),
            }
        }
        net_requests.clear();

        self.imp.process(&mut net_events);
        for event in net_events.iter() {
            match event {
                NetworkEvent::PeerMoved(origin_id, t) => {
                    for (slave_transform, transform) in (&slave_transform_flags, &mut transforms).join() {
                        if origin_id == &slave_transform.0 {
                            transform.pos = t.pos;
                        }
                    }
                },
                NetworkEvent::PeerConnected(origin_id) => {
                    let rb = physics.create_rigid_body_static();
                    let col = physics.create_collider_rectangle(Vector::new(1.9, 1.9), Vector::new(0.05, 0.05), &rb);
                    updater.create_entity(&entities).with(SpriteCom::new("wizard", Vector::new(2.0, 2.0)))
                        .with(TransformCom::new(Vector::new(0.0, 0.0)))
                        .with(NetSlaveTransformCom::new(origin_id.clone()))
                        .with(rb).with(col).build();
                },
                NetworkEvent::PeerDisconnected(origin_id) => {
                    for (entity, slave_transform, rb, col) in (&entities, &slave_transform_flags, &rigid_bodies, &colliders).join() {
                        if origin_id == &slave_transform.0 {
                            physics.bodies.remove(rb.0);
                            physics.colliders.remove(col.0);
                            updater.remove::<SpriteCom>(entity);
                            updater.remove::<TransformCom>(entity);
                            updater.remove::<NetSlaveTransformCom>(entity);
                            updater.remove::<RigidBodyCom>(entity);
                            updater.remove::<ColliderCom>(entity);
                        }
                    }
                }
            }
        }

        for (_master_transform, transform) in (&master_transform_flags, &mut transforms).join() {
            self.imp.broadcast(Packet::Transform(TransformPacket::new(self.imp.id, transform.clone()))).unwrap();
        }
    }
}

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct NetMasterTransformCom;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(DenseVecStorage)]
pub struct NetSlaveTransformCom(NetID);

