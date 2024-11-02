/*pub enum NodeType{
    Client(u64), MediaServer(u64), TextServer(u64), Drone(u64)
}
 */
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::fragment::{Fragment, Packet};
use crate::message::Message;
use crate::routing::NodeType;

pub struct Node{
    name: &'static str,
    node_type: NodeType,
    neighbors: HashMap<&'static str, NodeRef>, //node ids
}
type NodeRef = Rc<RefCell<Node>>;
//impl NetAble for NodeRef{...}
struct Route{
    path: Vec<NodeRef>
}
type Path = Vec<&'static str>; //collection of node ids, which are neighbor keys to refs
pub trait NetAble{
    fn send_packet(&self, path: &Path, hop: usize, packet: Packet ) -> bool;
    // forwards a packet and sends back an acknowledgement to the sender.
    fn find_routes(&self) -> Vec<Route>;

}