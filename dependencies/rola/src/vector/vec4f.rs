use std::ops::{MulAssign, DivAssign, AddAssign, SubAssign};
use std::ops::{Sub, Add, Mul, Div};
/* ----- Vec4f ----- */

pub struct Vec4f{
    pub e : [f32; 4],
}
impl Vec4f{
    pub fn new(data: [f32; 4]) -> Vec4f {
        Vec4f{e: data}
    }
    pub fn from(a: f32, b: f32, c: f32, d: f32) -> Vec4f {
        Vec4f{e: [a,b,c,d]}
    }
    pub fn normalize(&self) -> Vec4f {
        // returns another vector, doesnt mutate self
        let mag = self.magnitude();
        Vec4f{e: [self.e[0] / mag, self.e[1] / mag, self.e[2] / mag, self.e[3] / mag]}
    }
    pub fn magnitude(&self) -> f32 {
        let mag_squared: f32 = self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2] + self.e[3]*self.e[3];
        return mag_squared.sqrt();
    }
}

impl Sub for Vec4f{
    type Output = Vec4f;
    fn sub(self, v1: Vec4f) -> Vec4f{
        Vec4f{e : [self.e[0]-v1.e[0], self.e[1]-v1.e[1], self.e[2]-v1.e[2], self.e[3]-v1.e[3]]}
    }
}
impl Add for Vec4f{
    type Output = Vec4f;
    fn add(self, v1: Vec4f) -> Vec4f{
        Vec4f{e : [self.e[0]+v1.e[0], self.e[1]+v1.e[1], self.e[2]+v1.e[2], self.e[3]+v1.e[3]]}
    }
}
impl Mul<f32> for Vec4f{
    type Output = Vec4f;
    fn mul(self, scalar: f32) -> Vec4f{
        Vec4f{e : [self.e[0]*scalar, self.e[1]*scalar, self.e[2]*scalar, self.e[3]*scalar]}
    }
}
impl Div<f32> for Vec4f{
    type Output = Vec4f;
    fn div(self, scalar: f32) -> Vec4f{
        Vec4f{e : [self.e[0]/scalar, self.e[1]/scalar, self.e[2]/scalar, self.e[3]/scalar]}
    }
}

impl MulAssign<f32> for Vec4f {
    fn mul_assign(&mut self, scalar: f32){
        self.e[0] *= scalar;
        self.e[1] *= scalar;
        self.e[2] *= scalar;
        self.e[3] *= scalar;
    }
}
impl DivAssign<f32> for Vec4f {
    fn div_assign(&mut self, scalar: f32){
        self.e[0] /= scalar;
        self.e[1] /= scalar;
        self.e[2] /= scalar;
        self.e[3] /= scalar;
    }
}
impl AddAssign<f32> for Vec4f {
    fn add_assign(&mut self, scalar: f32){
        self.e[0] += scalar;
        self.e[1] += scalar;
        self.e[2] += scalar;
        self.e[3] += scalar;
    }
}
impl SubAssign<f32> for Vec4f {
    fn sub_assign(&mut self, scalar: f32){
        self.e[0] -= scalar;
        self.e[1] -= scalar;
        self.e[2] -= scalar;
        self.e[3] -= scalar;
    }
}
    
pub fn vec4f_dot(vec_a: Vec4f, vec_b: Vec4f) -> f32 {
    return vec_a.e[0] * vec_b.e[0] + vec_a.e[1] * vec_b.e[1] + vec_a.e[2] * vec_b.e[2] + vec_a.e[3] * vec_b.e[3];
}