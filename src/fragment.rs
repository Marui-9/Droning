use crate::routing::SourceRoutingHeader;

pub struct Packet { //fragment defined as entity exchanged by the drones.
    pt: PacketType,
    source_routing_header: SourceRoutingHeader,
    session_id: u64
    //sourcerouting header is inverted if necessary.
}

enum PacketType {
    Fragment(Fragment), Error(String), Ack, Dropped
}

pub struct Fragment{ // fragment defined as part of a message.
    header: FragmentHeader,
    data: FragmentData,
}

pub struct FragmentHeader {
    /// Total number of fragments, must be equal or greater than 1.
    total_n_fragments: u64,
    /// Index of the packet, from 0 up to total_n_fragments - 1.
    fragment_index: u64,
}

struct FragmentData {
    data: [char; 20], //it's possible to use .into_bytes() so that images
    //can also be encoded->[u8, 80]
    length: u8 // assembler will fragment/defragment data into bytes.
//  ^^^^^^^^^^ is this really necessary? wouldn't the last chars just be \0\0\0\0\0\0?
}
