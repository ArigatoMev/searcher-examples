use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::str::FromStr;

use bincode::serialize;
use solana_sdk::transaction::VersionedTransaction;

use crate::packet::{Meta as ProtoMeta, Packet as ProtoPacket};
use crate::shared::Socket;

/// Converts a VersionedTransaction to a protobuf packet
pub fn proto_packet_from_versioned_tx(tx: &VersionedTransaction) -> ProtoPacket {
    let data = serialize(tx).expect("serializes");
    let size = data.len() as u64;
    ProtoPacket {
        data,
        meta: Some(ProtoMeta {
            size,
            addr: "".to_string(),
            port: 0,
            flags: None,
            sender_stake: 0,
        }),
    }
}

/// Converts a GRPC Socket to stdlib SocketAddr
impl TryFrom<&Socket> for SocketAddr {
    type Error = AddrParseError;

    fn try_from(value: &Socket) -> Result<Self, Self::Error> {
        IpAddr::from_str(&value.ip).map(|ip| SocketAddr::new(ip, value.port as u16))
    }
}
