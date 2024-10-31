pub struct SourceRoutingHeader {
    /// ID of client or server
    pub source_id: &'static str,
    /// Number of entries in the hops field.
    /// Must be at least 1.
    pub n_hops: usize,
    /// List of nodes to which to forward the packet.
    pub  hops: [i64; 4],
    /// Index of the receiving node in the hops field.
    /// Ranges from 0 to n_hops - 1.
    pub hop_index: u64,
}
pub struct Query {
    /// Unique identifier of the flood, to prevent loops.
    flood_id: u64,
    /// ID of client or server
    initiator_id: u64,// RC<Arc<usize>>,
    /// Time To Live, decremented at each hop to limit the query's lifespan.
    ttl: u64,
    /// Records the nodes that have been traversed (to track the connections).
    path_trace: [u64; 20],
    node_types: [NodeType]
}
pub enum NodeType{
    Client(u64), MediaServer(u64), TextServer(u64), Drone(u64)
}