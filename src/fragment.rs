use crate::routing::SourceRoutingHeader;

pub struct Packet{ //fragment defined as entity exchanged by the drones.
    pt: PacketType,
    source_routing_header: SourceRoutingHeader,
    session_id: u64
    //sourcerouting header is inverted if necessary.
}

pub struct Fragment{ // fragment defined as part of a message.
    header: FragmentHeader,
    data: FragmentData,
}
enum PacketType {
    MsgPack(Fragment), ErrorPack(Error), AckPack(Ack), DroppedPack(Dropped)
}
struct FragmentData{
    data: [u8; 80], //it's possible to use .into_bytes() so that images
    //can also be encoded->[u8, 80]
    length: u8 // assembler will fragment/defragment data into bytes.
}
pub struct FragmentHeader {
    /// Identifies the session to which this fragment belongs.
    session_id: u64,
    /// Total number of fragments, must be equal or greater than 1.
    total_n_fragments: u64,
    /// Index of the packet, from 0 up to total_n_fragments - 1.
    fragment_index: u64,
}
struct Error {
    session_id: u64,
    id_not_neighbor: String,
    ttl: u8,
}
struct Dropped {
    session_id: u64,
}
pub struct Ack {
    session_id: u64,
    when: std::time::Instant,
    length: u8,
}
