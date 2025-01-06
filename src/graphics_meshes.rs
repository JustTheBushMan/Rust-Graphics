use crate::constants;
use crate::graphics_assets;
use crate::vector_functions;
use crate::vector_functions::vectorAdd;
use crate::vector_functions::vectorSubtract;

fn vertexify3Points(points:[[f64;3]; 3]) -> [graphics_assets::Vertex; 3] {
    [graphics_assets::Vertex{position:points[0]},graphics_assets::Vertex{position:points[1]},graphics_assets::Vertex{position:points[2]}]
}

fn createSquareMesh(pointA:[f64; 3], pointB:[f64; 3], pointC:[f64; 3]) -> graphics_assets::Mesh {
    let mut squareMeshTriangles : Vec<graphics_assets::Triangle> = vec![];
    squareMeshTriangles.push(graphics_assets::Triangle{vertices:vertexify3Points([pointA,pointB,pointC]),color:[0,0,0]});
    squareMeshTriangles.push(graphics_assets::Triangle{vertices:vertexify3Points([pointA,vectorAdd(vectorSubtract(pointC,pointB),pointA),pointC]),color:[0,0,0]});
    graphics_assets::Mesh{triangles:squareMeshTriangles}
}

