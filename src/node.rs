use NID;

/// Node inside a network that is just there to wrap around the sigmoid function and eventually other ones later
#[derive(Debug)]
pub struct Node {
    /// Locally unique ID for a node
    pub id: NID
}

impl Node {
    pub fn new(id: NID) -> Node {
        Node {
            id: id
        }
    }
}
