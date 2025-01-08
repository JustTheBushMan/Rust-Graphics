use std::f64::consts::PI;

fn degreeToRadians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn radianToDegrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

pub(crate) fn vectorSubtract(a: [f64; 3],b: [f64; 3]) -> [f64; 3] {
    [a[0]-b[0],a[1]-b[1],a[2]-b[2]]
}

pub(crate) fn vectorAdd(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0]+b[0],a[1]+b[1],a[2]+b[2]]
}

pub(crate) fn vectorMultiply(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    let mut newVec: [f64; 3] = a.clone();
    for index in 0..3 {
        newVec[index] *= b[index]
    }
    newVec
}

pub(crate) fn vectorAverage(vectors: Vec<[f64; 3]>) -> [f64; 3] {
    let mut vecSum: [f64; 3] = [0.0;3];
    for vector in &vectors {
        for index in 0..3 {
            vecSum[index] += vector[index];
        }
    }
    [vecSum[0]/vectors.len() as f64,vecSum[1]/vectors.len() as f64,vecSum[2]/vectors.len() as f64]
}

fn float_to_array3(value: f64) -> [f64; 3] {
    [value, value, value]
}

fn vectDistance(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0]-b[0]).powi(2)+(a[1]-b[1]).powi(2)+(a[2]-b[2]).powi(2)).powf(0.5)
}

fn rotateCoord(coordinate:Vec<f64>,degree:f64,center:Vec<f64>) -> Vec<f64>{
    vec![center[0] + (coordinate[0] - center[0]) * degreeToRadians(degree).cos() - (coordinate[2] - center[2]) * degreeToRadians(degree).sin(), coordinate[1], center[2] + (coordinate[0] - center[0]) * degreeToRadians(degree).sin() + (coordinate[2] - center[2]) * degreeToRadians(degree).cos()]
}

fn vectorRange(coordA:[f64; 3],coordB:[f64; 3],numPoints:i32) -> Vec<f64>{
    let list: vec<[f64; 3]>
    list.extend(coordA)
}