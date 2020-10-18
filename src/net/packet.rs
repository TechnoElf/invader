use std::convert::TryInto;
use std::ops::Deref;
use std::fmt::{Display, Debug, Formatter};
use std::fmt;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use invader_macro::DefaultConstructor;
use crate::physics::TransformCom;
use crate::misc::Vector;

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Packet {
    Empty,
    ConRequest(ConRequestPacket),
    ConAcknowledge(ConAcknowledgePacket),
    ConNew(ConNewPacket),
    ConDelete(ConDeletePacket),
    ConRedirect(ConRedirectPacket),
    ConHeartbeat(ConHeartbeatPacket),
    Transform(TransformPacket)
}

impl Packet {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();

        match self {
            Packet::ConRequest(_) => {
                packet.push(1);
            },
            Packet::ConAcknowledge(p) => {
                packet.push(2);
                packet.extend_from_slice(&p.origin_id.to_bytes());
                packet.extend_from_slice(&p.assigned_id.to_bytes());
            },
            Packet::ConNew(p) => {
                packet.push(3);
                packet.extend_from_slice(&p.peer_id.to_bytes());
                match p.socket {
                    SocketAddr::V4(s) => {
                        packet.extend_from_slice(&s.ip().octets());
                        packet.extend_from_slice(&s.port().to_le_bytes());
                    },
                    SocketAddr::V6(_) => panic!("Ipv6 is not supported")
                }
            },
            Packet::ConDelete(p) => {
                packet.push(4);
                packet.extend_from_slice(&p.peer_id.to_bytes());
            },
            Packet::ConRedirect(p) => {
                packet.push(5);
                match p.host_socket {
                    SocketAddr::V4(s) => {
                        packet.extend_from_slice(&s.ip().octets());
                        packet.extend_from_slice(&s.port().to_le_bytes());
                    },
                    SocketAddr::V6(_) => panic!("Ipv6 is not supported")
                }
            },
            Packet::ConHeartbeat(p) => {
                packet.push(6);
                packet.extend_from_slice(&p.origin_id.to_bytes());
            },
            Packet::Transform(p) => {
                packet.push(7);
                packet.extend_from_slice(&p.origin_id.to_bytes());
                packet.extend_from_slice(&p.transform.pos.x.to_le_bytes());
                packet.extend_from_slice(&p.transform.pos.y.to_le_bytes());
            },
            _ => packet.push(0)
        }

        packet
    }

    pub fn from_bytes(packet: Vec<u8>) -> Self {
        match packet[0] {
            1 => Packet::ConRequest(ConRequestPacket::new()),
            2 => Packet::ConAcknowledge(ConAcknowledgePacket {
                origin_id: NetID::from_bytes(&packet[1..=4]),
                assigned_id: NetID::from_bytes(&packet[5..=8])
            }),
            3 => Packet::ConNew(ConNewPacket {
                peer_id: NetID::from_bytes(&packet[1..=4]),
                socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(packet[5], packet[6], packet[7], packet[8])), u16::from_le_bytes(packet[9..=10].try_into().unwrap()))
            }),
            4 => Packet::ConDelete(ConDeletePacket::new(
                NetID::from_bytes(&packet[1..=4])
            )),
            5 => Packet::ConRedirect(ConRedirectPacket::new(
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(packet[1], packet[2], packet[3], packet[4])), u16::from_le_bytes(packet[5..=6].try_into().unwrap()))
            )),
            6 => Packet::ConHeartbeat(ConHeartbeatPacket::new(
                NetID::from_bytes(&packet[1..=4])
            )),
            7 => Packet::Transform(TransformPacket {
                origin_id: NetID::from_bytes(&packet[1..=4]),
                transform: TransformCom::new(
                    Vector::new(
                        f32::from_le_bytes(packet[5..=8].try_into().unwrap()),
                        f32::from_le_bytes(packet[9..=12].try_into().unwrap())
                    )
                )
            }),
            _ => Packet::Empty
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct NetID(Option<u32>);

impl NetID {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn init(self, id: u32) -> Self {
        Self(Some(id))
    }

    pub fn is_shared(&self) -> bool {
        self.0.is_some()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        NetID::new().init(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn to_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}

impl Deref for NetID {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        match self.0.is_some() {
            true => unsafe { &*((&self.0.unwrap()) as *const u32) },
            false => &0
        }
    }
}

impl Display for NetID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#010x}", self.0.or(Some(0)).unwrap())
    }
}

impl Debug for NetID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#010x}", self.0.or(Some(0)).unwrap())
    }
}

#[derive(Copy, Clone, Debug, DefaultConstructor)]
pub struct ConRequestPacket;

#[derive(Copy, Clone, Debug, DefaultConstructor)]
pub struct ConAcknowledgePacket {
    pub origin_id: NetID,
    pub assigned_id: NetID
}

#[derive(Copy, Clone, Debug, DefaultConstructor)]
pub struct ConNewPacket {
    pub peer_id: NetID,
    pub socket: SocketAddr
}

#[derive(Copy, Clone, Debug, DefaultConstructor)]
pub struct ConRedirectPacket {
    pub host_socket: SocketAddr
}

#[derive(Copy, Clone, Debug, DefaultConstructor)]
pub struct ConDeletePacket {
    pub peer_id: NetID
}

#[derive(Copy, Clone, Debug, DefaultConstructor)]
pub struct ConHeartbeatPacket {
    pub origin_id: NetID
}

#[derive(Clone, Debug, DefaultConstructor)]
pub struct TransformPacket {
    pub origin_id: NetID,
    pub transform: TransformCom
}
