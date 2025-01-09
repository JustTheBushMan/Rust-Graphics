use crate::constants::*;
use crate::vector_functions;

//Vertex Library
pub(crate) struct vertexLibrary {
    vertexes: Vec<Vertex>
}
impl vertexLibrary {
    fn newVertexes(&mut self,vertexList: Vec<Vertex>) {
        self.vertexes.extend(vertexList)
    }
    fn newTriangles(&mut self,vertexCoord: [f64;3],newTriangles: Vec<u32>){
        self.vertexes[self.vertexes.position(vertexCoord)].triangles.extend(newTriangles)
    }
    fn removeTriangles(&mut self,vertexCoord: [f64;3],newTriangles: Vec<u32>){
        
    }
}

//Vertex
pub(crate) struct Vertex {
    pub(crate) position: [f64; 3],
    pub(crate) triangles: Vec<u32>
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

