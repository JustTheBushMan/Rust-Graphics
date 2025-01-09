use crate::constants;
use crate::graphics_assets;
use crate::vector_functions;
use crate::vector_functions::{vectorAdd, vectorMultiply, vectorDivide, vectorAverage,floatToArray3};
use crate::vector_functions::vectorSubtract;
use crate::vector_functions::vectorRange;
use crate::vector_functions::vectorDistance;

fn vertexify3Points(points:[[f64;3]; 3]) -> [graphics_assets::Vertex; 3] {
    [graphics_assets::Vertex{position:points[0], triangles: vec![] },graphics_assets::Vertex{position:points[1], triangles: vec![] },graphics_assets::Vertex{position:points[2], triangles: vec![] }]
}

fn createSquareMesh(pointA:[f64; 3], pointB:[f64; 3], pointC:[f64; 3]) -> graphics_assets::Mesh {
    let mut squareMeshTriangles : Vec<graphics_assets::Triangle> = vec![];
    squareMeshTriangles.push(graphics_assets::Triangle{vertices:[pointA,pointB,pointC],color:[0,0,0]});
    squareMeshTriangles.push(graphics_assets::Triangle{vertices:[pointA,vectorAdd(vectorSubtract(pointC,pointB),pointA),pointC],color:[0,0,0]});
    graphics_assets::Mesh{triangles:squareMeshTriangles}
}

fn createCircleMesh(pointA:[f64; 3], pointB:[f64; 3], pointC:[f64;3]) -> graphics_assets::Mesh {
    let CIRCLE_POINTS : i32 = 24;
    let radius:f64 = vectorDistance(pointA,pointB)/ 2.0;
    let mut circleMeshTriangles : Vec<graphics_assets::Triangle> = vec![];
    let mut outerSquare : Vec<[f64;3]>;
    let pointD:[f64; 3] = vectorAdd(pointA,vectorSubtract(pointC,pointB));
    let center:[f64;3] = vectorAverage(Vec::from([pointA, pointB, pointC, pointD]));
    outerSquare.extend(vectorRange(pointA,pointB,CIRCLE_POINTS/4));
    outerSquare.extend(vectorRange(pointB,pointC,CIRCLE_POINTS/4));
    outerSquare.extend(vectorRange(pointC,pointD,CIRCLE_POINTS/4));
    outerSquare.extend(vectorRange(pointD,pointA,CIRCLE_POINTS/4));
    //Normalize square into triangle
    let outerCircle:Vec<[f64;3]> = outerSquare.iter().map([coord] vectorAdd(coord,vectorDivide(vectorSubtract(center,coord),floatToArray3(radius/vectorDistance(coord,center)))).collect();
}

