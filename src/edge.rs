use std::ops::Deref;

use vertex::Vertex;

#[derive(Debug)]
pub struct Edge {
    pub from_vertex: *mut Vertex,
    pub to_vertex: *mut Vertex
}

pub struct EdgeProxy {
    pub edge: *mut Edge
}

impl Deref for EdgeProxy {
    type Target = Edge;
    fn deref<'a>(&'a self) -> &'a Edge {
        unsafe { &*(self.edge) }
    }
}


