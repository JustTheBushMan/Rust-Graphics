use crate::constants::*;
use crate::vector_functions;

pub(crate)<T> struct Matrix {
    pub(crate) rows: u32,
    pub(crate) columns: u32,
    pub(crate) elements: Vec<Vec<T>>
}

//Vertex
pub(crate) struct Vertex {
    pub(crate) position: [f64; 3],
    pub(crate) triangles: Vec<[f64; 3]>
}

//Triangle
pub(crate) struct Triangle {
    pub(crate) vertices: [[f64; 3]; 3],
    pub(crate) color: [i32; 3]
}
impl Triangle {
    fn changeVertices(&mut self,vertices:[[f64; 3]; 3]) {
        self.vertices = vertices;
    }
    fn changeColor(&mut self,color:[i32; 3]) {
        self.color = color;
    }
    fn translate(&mut self,translation:[f64; 3]) {
        for vertex in &mut self.vertices {
            *vertex = vector_functions::vectorAdd(translation, *vertex);
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

