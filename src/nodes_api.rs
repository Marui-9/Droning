/*pub enum NodeType{
    Client(u64), MediaServer(u64), TextServer(u64), Drone(u64)
}
 */
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::fragment::Packet;
use crate::routing::NodeType;

pub struct Node{
    name: &'static str,
    node_type: NodeType,
    neighbors: HashMap<&'static str, NodeRef>,
}
type NodeRef = Rc<RefCell<Node>>;
struct Route{
    path: Vec<NodeRef>
}
type Path = Vec<&'static str>; //collection of node ids
pub trait Netable{
    fn send_packet(&self, path: &Path, hop: usize, packet: Packet ) -> bool;
    fn find_routes(&self) -> Route;
}