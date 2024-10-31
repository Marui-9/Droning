use crate::routing::SourceRoutingHeader;

struct Fragment{ //package defined as the entity which travels between drones: msg, ask, err, drop
    fe: FragmentType,
    source_routing_header: SourceRoutingHeader, // so that drones only deal with dropping and
    //sending or receiving
    //sourcerouting header is inverted if necessary.
}

struct MsgFragment{ // fragment defined as part of a message.
    header: FragmentHeader, //e.g. 5 of 12
    data: FragmentData,
    //frag_type: FragmentType,
}
enum FragmentType {
    MsgFrag(MsgFragment), ErrorFrag(Error), AckFrag(Ack), DroppedFrag(Dropped)
}

struct Error {
    //source_routing_header: SourceRoutingHeader,
    session_id: u64, //changed from char
    /// ID of drone, server of client that is not a neighbor:
    id_not_neighbor: String,
    ttl: u32,
}
struct Dropped {
    //source_routing_header:
        session_id: u64,
}
pub struct Ack(AckInner);

struct AckInner {
    session_id: u64,
    when: std::time::Instant,
    //add: BACKTRACE - list of nodes to go back to client
}
pub struct FragmentHeader {
    /// Identifies the session to which this fragment belongs.
    session_id: u64,
    /// Total number of fragments, must be equal or greater than 1.
    total_n_fragments: u64,
    /// Index of the packet, from 0 up to total_n_fragments - 1.
    fragment_index: u64, //TO BE U64! NO STRING
    //next_fragment: NextFragment, removed! not necessary since
    // we already have current number and total number of frags
}
struct FragmentData{
    data: [u8; 80], //6possible to use .into_bytes() so that images can also be encoded->[u8, 80]
    length: u8 // assembler will fragment/defragment data into bytes.
}

type NextFragment = Option<Box<FragmentHeader>>;
