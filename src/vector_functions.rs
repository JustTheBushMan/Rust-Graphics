use std::any::TypeId;
use std::f64::consts::PI;

fn degreeToRadians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn radianToDegrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

fn vectorSubtract(a: Vec<f64>,b: Vec<f64>) -> Vec<f64> {
    vec![a[0]-b[0],a[1]-b[1],a[2]-b[2]]
}

fn vectorAdd(a: Vec<f64>,b: Vec<f64>) -> Vec<f64> {
    vec![a[0]+b[0],a[1]+b[1],a[2]+b[2]]
}

fn vectorMultiply<MultType>(a: Vec<f64>, b: MultType) -> Vec<f64> {
    if TypeId::of::<MultType>() == TypeId::of::<f64>() {
        let mut newVec: Vec<f64> = a.clone();
         newVec.into_iter().map(|x| x * b);
        .collect::<Vec<f64>>()
    }
    if TypeId::of::<MultType>() == TypeId::of::<Vec<f64>>() {
        let mut newVec: Vec<f64> = a.clone();
        for index in 0..3 {
            newVec[index] *= b[index]
        }
        newVec
    }
}

fn vectDistance(a: Vec<f64>, b: Vec<f64>) -> f64 {
    ((a[0]-b[0]).powi(2)+(a[1]-b[1]).powi(2)+(a[2]-b[2]).powi(2)).powf(0.5)
}

fn rotateCoord(coordinate:Vec<f64>,degree:f64,center:Vec<f64>) -> Vec<f64>{
    vec![center[0] + (coordinate[0] - center[0]) * degreeToRadians(degree).cos() - (coordinate[2] - center[2]) * degreeToRadians(degree).sin(), coordinate[1], center[2] + (coordinate[0] - center[0]) * degreeToRadians(degree).sin() + (coordinate[2] - center[2]) * degreeToRadians(degree).cos()]
}