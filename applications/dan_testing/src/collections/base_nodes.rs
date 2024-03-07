use std::collections::HashMap;

use super::collection::Collection;
use crate::processes::base_node::BaseNode;

pub struct BaseNodes {
    nodes: HashMap<String, BaseNode>,
}

impl Collection<BaseNode> for BaseNodes {
    fn new() -> Self {
        BaseNodes { nodes: HashMap::new() }
    }

    fn items(&self) -> &HashMap<String, BaseNode> {
        return &self.nodes;
    }

    fn items_mut(&mut self) -> &mut HashMap<String, BaseNode> {
        return &mut self.nodes;
    }
}

impl BaseNodes {
    pub fn get_addresses(&self) -> Vec<String> {
        self.nodes.values().map(|node| node.get_address()).collect()
    }

    pub fn add(&mut self) {
        let new_name = format!("BaseNode{}", self.nodes.len());
        let base_node = BaseNode::new(&new_name, self.get_addresses());
        self.nodes.insert(new_name, base_node);
    }
}
