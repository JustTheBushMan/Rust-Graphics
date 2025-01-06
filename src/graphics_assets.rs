use crate::constants::*;
use crate::vector_functions;

//Vertex
pub(crate) struct Vertex {
    pub(crate) position: [f64; 3],
}

//Triangle
pub(crate) struct Triangle {
    pub(crate) vertices: [Vertex; 3],
    pub(crate) color: [i32; 3]
}
impl Triangle {
    fn changeVertices(&mut self,vertices:[Vertex; 3]) {
        self.vertices = vertices;
    }
    fn changeColor(&mut self,color:[i32; 3]) {
        self.color = color;
    }
    fn translate(&mut self,translation:[f64; 3]) {
        for vertex in &mut self.vertices {
            vertex.position = vector_functions::vectorAdd(translation, vertex.position);
        }
    }
}

//Mesh
pub(crate) struct Mesh {
    pub(crate) triangles: Vec<Triangle>
}
impl Mesh {
    pub(crate) fn changeAllTriangles(&mut self,triangles:Vec<Triangle>) {
        self.triangles = triangles;
    }
    fn changeOneTriangle(&mut self,index:usize,newTriangle:Triangle) {
        self.triangles[index]=newTriangle
    }
    fn addTriangles(&mut self,triangles:Vec<Triangle>) {
        self.triangles.extend(triangles)
    }
    fn translate(&mut self, translation: [f64; 3]) {
        for mut triangle in self.triangles.iter_mut() {
            triangle.translate(translation);
        }
    }
}

