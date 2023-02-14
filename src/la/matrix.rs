// new matrix container


// ---- 2x2 Matrix ----
// could be seen as unnecessary but with a struct for 2x2, linear algebra goes under the hood

// eigenvectors
// eigenvalues
// multiply operator: (Mat2x2 * Mat2x2) and (scalar * Mat2x2)
// transpose
use std::ops::{Mul, MulAssign};
use crate::Vec3f;

pub struct Mat2x2 {
    pub e: [f32; 4],
}
impl Mat2x2 {
    pub fn new(data: [f32; 4]) -> Mat2x2 {
        Mat2x2{ e: data }
    }
    pub fn to_array(self) -> [f32; 4] {
        [self.e[0], self.e[1], self.e[2], self.e[3]]
    }
    pub fn eigenvectors(&self, lambda1: f32, lambda2: f32) -> ([f32;2], [f32; 2]) {
        if self.e[2] != 0.0 {
            return ([(lambda1 - self.e[3])/self.e[1] , self.e[2]/self.e[1]], [(lambda2 - self.e[3])/self.e[1], self.e[2]/self.e[1]]);
        } else if self.e[1] != 0.0 {
            return ([1.0 , (lambda1 - self.e[0])/self.e[1]], [1.0, (lambda2 - self.e[0])/self.e[1]]);
        } else if self.e[2] == 0.0 && self.e[1] == 0.0 {
            return ([1.0, 0.0], [0.0, 1.0]);
        } else {
            // something is very wrong if this happens
            // result here
            return ([0.0, 0.0], [0.0, 0.0]);
        }
    }
    pub fn eigenvalues(&self) -> (f32, f32) {
        let characteristic_polynomial = [1.0, -1.0 * (self.e[0] + self.e[3]), (self.e[0]*self.e[3]-self.e[1]*self.e[2])];
        Self::polynomial_roots(characteristic_polynomial)
    }
    fn polynomial_roots(coefs: [f32; 3]) -> (f32, f32) {
        // coefs = a,b,c => ax^2 + bx + c
        let discriminant = coefs[1]*coefs[1] - 4.0 * coefs[0] * coefs[2];
        let positive_root = ((-1.0 * coefs[1]) + discriminant.sqrt()) / 2.0 * coefs[0];
        let negative_root = ((-1.0 * coefs[1]) - discriminant.sqrt()) / 2.0 * coefs[0];
        (positive_root, negative_root)
    }
    pub fn transpose(&mut self) -> Mat2x2 {
        // a, b, c, d -> a, c, b, d
        Mat2x2{ e: [self.e[0], self.e[2], self.e[1], self.e[3]]}
    }
}

// override for matrix * matrix
impl Mul<Mat2x2> for Mat2x2 {
    type Output = Mat2x2;
    fn mul(self, m1: Mat2x2) -> Mat2x2 {
        Mat2x2{ e: [self.e[0] * m1.e[0] + self.e[1] * m1.e[2], 
                    self.e[0] * m1.e[1] + self.e[1] * m1.e[3],
                    self.e[2] * m1.e[0] + self.e[3] * m1.e[2],
                    self.e[2] * m1.e[1] + self.e[3] * m1.e[3]] }
    }
}
// override for matrix * scalar
impl Mul<f32> for Mat2x2 {
    type Output = Mat2x2;
    fn mul(self, scalar: f32) -> Mat2x2 {
        Mat2x2{ e : [self.e[0] * scalar, self.e[1] * scalar, self.e[2] * scalar, self.e[3] * scalar]}
    }
}

impl MulAssign<f32> for Mat2x2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.e[0] *= scalar; self.e[1] *= scalar; 
        self.e[2] *= scalar; self.e[3] *= scalar;
    }
}


// ---- 3x3 Matrices ----
#[derive(Copy, Clone)]
pub struct Mat3x3{
    pub e: [f32; 9],
}
impl Mat3x3{
    pub fn new() -> Mat3x3{
        let dat: [f32; 9] = [0.0; 9];
        Mat3x3{ e: dat}
    }
    pub fn rotation_x(&mut self, theta: &f32){
        self.e[0] = 1.0;
        self.e[4] = theta.cos();
        self.e[5] = -1.0 * theta.sin();
        self.e[7] = theta.sin();
        self.e[8] = theta.cos();
    }
    pub fn rotation_y(&mut self, theta: f32){
        self.e[0] = theta.cos();
        self.e[2] = theta.sin();
        self.e[4] = 1.0;
        self.e[6] = -1.0 * theta.sin();
        self.e[8] = theta.cos();
    }
    pub fn rotation_z(&mut self, theta: &f32){
        self.e[0] = theta.cos();
        self.e[1] = -1.0 * theta.sin();
        self.e[3] = theta.sin();
        self.e[4] = theta.cos();
        self.e[8] = 1.0;
    }
}

impl Mul<Vec3f> for Mat3x3 {
    type Output = Vec3f;
    fn mul(self, v: Vec3f) -> Vec3f {
        Vec3f::new(&[self.e[0]*v.e[0] + self.e[1]*v.e[1] + self.e[2]*v.e[2],
                    self.e[3]*v.e[0] + self.e[4]*v.e[1] + self.e[5]*v.e[2],
                    self.e[6]*v.e[0] + self.e[7]*v.e[1] + self.e[8]*v.e[2]])
    }
}

// ---- 4x4 Matrices ----
pub struct Mat4x4{
    pub e: [f32; 16],
}
impl Mat4x4 {
    pub fn new() -> Mat4x4{
        let dat: [f32; 16] = [0.0; 16];
        Mat4x4{ e : dat }
    }
    pub fn projection(&mut self, h: &f32, w: &f32, fov: &f32, zfar: &f32, znear: &f32){
        let a: f32 = w / h;
        let f: f32 = 1.0 / (fov*0.5).tan();
        let q: f32 = zfar / (zfar - znear);
    
        self.e[0] = a * f;
        self.e[5] = f;
        self.e[10] = q;
        self.e[11] = 1.0;
        self.e[14] = -1.0 * znear * q;
    }
}