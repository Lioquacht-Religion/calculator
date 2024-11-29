//ast.rs
//

type NodesList<T> = Vec<Node<T>>;
type NodeChildren<T> = Vec<NodesList<T>>;

#[derive(Debug)]
pub struct ASTree<T>{
    pub node_children : NodeChildren<T>,
}


#[derive(Debug)]
pub struct NodeID{
    pub list_id : u32,
    pub node_id : u32,
}

impl NodeID{
    pub fn new(list_id: u32, node_id: u32) -> Self{
        Self{
            list_id,
            node_id
        }
    }

}


#[derive(Debug)]
pub struct Node<T>{
    pub value : T,
    pub parent : Option<NodeID>,
    pub children : Option<u32>, // id for list of nodes
}

impl<T> Node<T> {
    pub fn new(value: T, parent: Option<NodeID>, children: Option<u32>) -> Self{
        Self { value, parent, children}
    }
}

impl<T> ASTree<T> {
    pub fn new()->Self{
        Self{
            node_children: Vec::new(),
        }
    }

    pub fn get_node_list(&self, list_id : usize) -> Option<&NodesList<T>>{
        self.node_children.get(list_id)
    }

    pub fn get_mut_node_list(&mut self, list_id : usize) -> Option<&mut NodesList<T>>{
        self.node_children.get_mut(list_id)
    }


    pub fn get_node(&self, id : &NodeID) -> Option<&Node<T>>{
        self.node_children.get(id.list_id as usize)?.get(id.node_id as usize)
    }

    pub fn get_mut_node(&mut self, id : &NodeID) -> Option<&mut Node<T>>{
        self.node_children.get_mut(id.list_id as usize)?.get_mut(id.node_id as usize)
    }

    pub fn add_node_list(&mut self, node_list: Vec<Node<T>>) -> usize{
        self.node_children.push(node_list);
        self.node_children.len()-1
    }

    pub fn add_node_to_list(&mut self, node : Node<T>, list_id : usize) -> Result<NodeID, Node<T>>{
        if let Some(list) = self.node_children.get_mut(list_id){
            list.push(node);
            let node_id : u32 = (list.len()-1) as u32;
            Ok(NodeID::new(list_id as u32, node_id))
        }
        else{
            Err(node)
        }

    }

    // add code here
}


