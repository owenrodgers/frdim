use std::ops::{MulAssign, DivAssign, AddAssign, SubAssign};
use std::ops::{Sub, Add, Mul, Div};
/* ----- Vec3f ----- */

pub struct Vec3f{
    pub e : [f32; 3],
}
impl Vec3f{
    pub fn new(data: [f32; 3]) -> Vec3f {
        Vec3f{e: data}
    }
    pub fn from(a: f32, b: f32, c: f32) -> Vec3f {
        Vec3f{e: [a,b,c]}
    }
    pub fn normalize(&self) -> Vec3f {
        // returns another vector, doesnt mutate self
        let mag = self.magnitude();
        Vec3f{e: [self.e[0] / mag, self.e[1] / mag, self.e[2] / mag]}
    }
    pub fn magnitude(&self) -> f32 {
        let mag_squared: f32 = self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
        return mag_squared.sqrt();
    }
}

impl Sub for Vec3f{
    type Output = Vec3f;
    fn sub(self, v1: Vec3f) -> Vec3f{
        Vec3f{e : [self.e[0]-v1.e[0], self.e[1]-v1.e[1], self.e[2]-v1.e[2]]}
    }
}
impl Add for Vec3f{
    type Output = Vec3f;
    fn add(self, v1: Vec3f) -> Vec3f{
        Vec3f{e : [self.e[0]+v1.e[0], self.e[1]+v1.e[1], self.e[2]+v1.e[2]]}
    }
}
impl Mul<f32> for Vec3f{
    type Output = Vec3f;
    fn mul(self, scalar: f32) -> Vec3f{
        Vec3f{e : [self.e[0]*scalar, self.e[1]*scalar, self.e[2]*scalar]}
    }
}
impl Div<f32> for Vec3f{
    type Output = Vec3f;
    fn div(self, scalar: f32) -> Vec3f{
        Vec3f{e : [self.e[0]/scalar, self.e[1]/scalar, self.e[2]/scalar]}
    }
}

impl MulAssign<f32> for Vec3f {
    fn mul_assign(&mut self, scalar: f32){
        self.e[0] *= scalar;
        self.e[1] *= scalar;
        self.e[2] *= scalar;
    }
}

use crate::matrix::mat3x3::Mat3x3;
impl Mul<&Mat3x3> for Vec3f {
    type Output = Vec3f;
    fn mul(self, matrix: &Mat3x3) -> Vec3f {
        let mut d = [0.0; 3];
        d[0] = self.e[0]*matrix.e[0][0] + self.e[1]*matrix.e[0][1] + self.e[2]*matrix.e[0][2];
        d[1] = self.e[0]*matrix.e[1][0] + self.e[1]*matrix.e[1][1] + self.e[2]*matrix.e[1][2];
        d[2] = self.e[0]*matrix.e[2][0] + self.e[1]*matrix.e[2][1] + self.e[2]*matrix.e[2][2];
        Vec3f{ e : d }
    }
}
impl MulAssign<&Mat3x3> for Vec3f {
    fn mul_assign(&mut self, matrix: &Mat3x3) {
        self.e[0] = self.e[0]*matrix.e[0][0] + self.e[1]*matrix.e[0][1] + self.e[2]*matrix.e[0][2];
        self.e[1] = self.e[0]*matrix.e[1][0] + self.e[1]*matrix.e[1][1] + self.e[2]*matrix.e[1][2];
        self.e[2] = self.e[0]*matrix.e[2][0] + self.e[1]*matrix.e[2][1] + self.e[2]*matrix.e[2][2];
    }
}

impl DivAssign<f32> for Vec3f {
    fn div_assign(&mut self, scalar: f32){
        self.e[0] /= scalar;
        self.e[1] /= scalar;
        self.e[2] /= scalar;
    }
}
impl AddAssign<f32> for Vec3f {
    fn add_assign(&mut self, scalar: f32){
        self.e[0] += scalar;
        self.e[1] += scalar;
        self.e[2] += scalar;
    }
}
impl SubAssign<f32> for Vec3f {
    fn sub_assign(&mut self, scalar: f32){
        self.e[0] -= scalar;
        self.e[1] -= scalar;
        self.e[2] -= scalar;
    }
}

impl AddAssign<Vec3f> for Vec3f {
    fn add_assign(&mut self, vec2: Vec3f) {
        self.e[0] += vec2.e[0];
        self.e[1] += vec2.e[1];
        self.e[2] += vec2.e[2];
    }
}
impl SubAssign<Vec3f> for Vec3f {
    fn sub_assign(&mut self, vec2: Vec3f) {
        self.e[0] -= vec2.e[0];
        self.e[1] -= vec2.e[1];
        self.e[2] -= vec2.e[2];
    }
}
    

pub fn vec3f_dot(vec_a: Vec3f, vec_b: Vec3f) -> f32 {
    return vec_a.e[0] * vec_b.e[0] + vec_a.e[1] * vec_b.e[1] + vec_a.e[2] * vec_b.e[2];
}

pub fn vec3f_cross(vec_a: Vec3f, vec_b: Vec3f) -> Vec3f {
    let nx = vec_a.e[1]*vec_b.e[2] - vec_a.e[2]*vec_b.e[1];
    let ny = vec_a.e[2]*vec_b.e[0] - vec_a.e[0]*vec_b.e[2];
    let nz = vec_a.e[0]*vec_b.e[1] - vec_a.e[1]*vec_b.e[0];
    return Vec3f::from(nx, ny, nz);
}


use crate::matrix::mat4x4::Mat4x4;

impl MulAssign<&Mat4x4> for Vec3f {
    fn mul_assign(&mut self, matrix: &Mat4x4) {
        let w: f32;
        let mut d = [0.0; 3];
        d[0] = self.e[0]*matrix.e[0][0] + self.e[1]*matrix.e[1][0] + self.e[2]*matrix.e[2][0] + matrix.e[3][0];
        d[1] = self.e[0]*matrix.e[0][1] + self.e[1]*matrix.e[1][1] + self.e[2]*matrix.e[2][1] + matrix.e[3][1];
        d[2] = self.e[0]*matrix.e[0][2] + self.e[1]*matrix.e[1][2] + self.e[2]*matrix.e[2][2] + matrix.e[3][2];
        w = d[0]*matrix.e[0][3] + d[1]*matrix.e[1][3] + d[2]*matrix.e[2][3] + matrix.e[3][3];
        
        if w != 0.0{
            d[0] /= w; d[1] /= w; d[2] /= w;
        }
        self.e = d;
    }
}
pub fn projection_multiply(vector: &Vec3f, matrix: &Mat4x4) -> Vec3f {
    // multiplies and normalizes
    let w: f32;
    let mut d = [0.0; 3];
    d[0] = vector.e[0]*matrix.e[0][0] + vector.e[1]*matrix.e[1][0] + vector.e[2]*matrix.e[2][0] + matrix.e[3][0];
    d[1] = vector.e[0]*matrix.e[0][1] + vector.e[1]*matrix.e[1][1] + vector.e[2]*matrix.e[2][1] + matrix.e[3][1];
    d[2] = vector.e[0]*matrix.e[0][2] + vector.e[1]*matrix.e[1][2] + vector.e[2]*matrix.e[2][2] + matrix.e[3][2];
    w = d[0]*matrix.e[0][3] + d[1]*matrix.e[1][3] + d[2]*matrix.e[2][3] + matrix.e[3][3];
    
    if w != 0.0{
        d[0] /= w; d[1] /= w; d[2] /= w;
    }
    return Vec3f{ e : d }
}


