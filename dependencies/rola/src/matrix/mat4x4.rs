use std::ops::{MulAssign, DivAssign, AddAssign, SubAssign};
use std::ops::{Sub, Add, Mul, Div};
use crate::vector::vec3f::Vec3f;

// ---- 4x4 Matrices ----
pub struct Mat4x4{
    pub e: [[f32; 4]; 4],
}
impl Mat4x4 {
    pub fn new() -> Mat4x4{
        let dat: [[f32; 4]; 4] = [[0.0; 4]; 4];
        Mat4x4{ e : dat }
    }
    pub fn identity() -> Mat4x4{
        let mut dat: [[f32; 4]; 4] = [[0.0; 4]; 4];
        dat[0][0] = 1.0; dat[1][1] = 1.0; dat[2][2] = 1.0; dat[3][3] = 1.0;
        Mat4x4{ e : dat }
    }
    // the following are for transformations encompassing rotations and translations
    // why is everything so hard
    // https://pages.mtu.edu/~shene/COURSES/cs3621/NOTES/geometry/geo-tran.html
    pub fn rotate_x(theta: f32) -> Mat4x4 {
        let mut dat = [[0.0; 4]; 4];
        dat[0][0] = 1.0;
        dat[1][1] = theta.cos();
        dat[1][2] = -theta.sin();
        dat[2][1] = theta.sin();
        dat[2][2] = theta.cos();
        dat[3][3] = 1.0;
        Mat4x4{ e : dat }
    }
    pub fn rotate_y(theta: f32) -> Mat4x4 {
        let mut dat = [[0.0; 4]; 4];
        dat[0][0] = theta.cos();
        dat[0][2] = theta.sin();
        dat[1][1] = 1.0;
        dat[2][0] = -theta.sin();
        dat[2][2] = theta.cos();
        dat[3][3] = 1.0;
        Mat4x4{ e : dat }
    }
    pub fn rotate_z(theta: f32) -> Mat4x4 {
        let mut dat = [[0.0; 4]; 4];
        dat[0][0] = theta.cos();
        dat[0][1] = -theta.sin();
        dat[1][0] = theta.sin();
        dat[1][1] = theta.cos();
        dat[2][2] = 1.0;
        dat[3][3] = 1.0;
        Mat4x4{ e : dat }
    }
    pub fn apply_translation(&mut self, t_vector: &Vec3f) {
        self.e[0][3] += t_vector.e[0];
        self.e[1][3] += t_vector.e[1];
        self.e[2][3] += t_vector.e[2];
    }
    
    pub fn projection(&mut self, h: &f32, w: &f32, fov: &f32, zfar: &f32, znear: &f32){
        let a: f32 = h / w;
        let f: f32 = 1.0 / (fov*0.5).tan();
        let q: f32 = zfar / (zfar - znear);
    
        self.e[0][0] = a * f;
        self.e[1][1] = f;
        self.e[2][2] = q;
        self.e[2][3] = 1.0;
        self.e[3][2] = (-1.0 * zfar * znear) / (zfar - znear);
    }
    
}

// overloads

impl Sub for Mat4x4{
    type Output = Mat4x4;
    fn sub(self, m1: Mat4x4) -> Mat4x4{
        let mut dat: [[f32; 4]; 4] = [[0.0; 4]; 4];
        dat[0][0] = self.e[0][0] - m1.e[0][0]; dat[0][1] = self.e[0][1] - m1.e[0][1]; dat[0][2] = self.e[0][2] - m1.e[0][2]; dat[0][3] = self.e[0][3] - m1.e[0][3];
        dat[1][0] = self.e[1][0] - m1.e[1][0]; dat[1][1] = self.e[1][1] - m1.e[1][1]; dat[1][2] = self.e[1][2] - m1.e[1][2]; dat[1][3] = self.e[1][3] - m1.e[1][3];
        dat[2][0] = self.e[2][0] - m1.e[2][0]; dat[2][1] = self.e[2][1] - m1.e[2][1]; dat[2][2] = self.e[2][2] - m1.e[2][2]; dat[2][3] = self.e[2][3] - m1.e[2][3];
        dat[3][0] = self.e[3][0] - m1.e[3][0]; dat[3][1] = self.e[3][1] - m1.e[3][1]; dat[3][2] = self.e[3][2] - m1.e[3][2]; dat[3][3] = self.e[3][3] - m1.e[3][3];
        Mat4x4{ e : dat }
    }
}
impl Add for Mat4x4{
    type Output = Mat4x4;
    fn add(self, m1: Mat4x4) -> Mat4x4{
        let mut dat: [[f32; 4]; 4] = [[0.0; 4]; 4];
        dat[0][0] = self.e[0][0] + m1.e[0][0]; dat[0][1] = self.e[0][1] + m1.e[0][1]; dat[0][2] = self.e[0][2] + m1.e[0][2]; dat[0][3] = self.e[0][3] + m1.e[0][3];
        dat[1][0] = self.e[1][0] + m1.e[1][0]; dat[1][1] = self.e[1][1] + m1.e[1][1]; dat[1][2] = self.e[1][2] + m1.e[1][2]; dat[1][3] = self.e[1][3] + m1.e[1][3];
        dat[2][0] = self.e[2][0] + m1.e[2][0]; dat[2][1] = self.e[2][1] + m1.e[2][1]; dat[2][2] = self.e[2][2] + m1.e[2][2]; dat[2][3] = self.e[2][3] + m1.e[2][3];
        dat[3][0] = self.e[3][0] + m1.e[3][0]; dat[3][1] = self.e[3][1] + m1.e[3][1]; dat[3][2] = self.e[3][2] + m1.e[3][2]; dat[3][3] = self.e[3][3] + m1.e[3][3];
        Mat4x4{ e : dat }
    }
}

impl Mul<f32> for Mat4x4{
    type Output = Mat4x4;
    fn mul(self, scalar: f32) -> Mat4x4{
        let mut dat: [[f32; 4]; 4] = [[0.0; 4]; 4];
        dat[0][0] = self.e[0][0] * scalar; dat[0][1] = self.e[0][1] * scalar; dat[0][2] = self.e[0][2] * scalar; dat[0][3] = self.e[0][3] * scalar;
        dat[1][0] = self.e[1][0] * scalar; dat[1][1] = self.e[1][1] * scalar; dat[1][2] = self.e[1][2] * scalar; dat[1][3] = self.e[1][3] * scalar;
        dat[2][0] = self.e[2][0] * scalar; dat[2][1] = self.e[2][1] * scalar; dat[2][2] = self.e[2][2] * scalar; dat[2][3] = self.e[2][3] * scalar;
        dat[3][0] = self.e[3][0] * scalar; dat[3][1] = self.e[3][1] * scalar; dat[3][2] = self.e[3][2] * scalar; dat[3][3] = self.e[3][3] * scalar;
        Mat4x4{ e : dat }
    }
}

impl Div<f32> for Mat4x4{
    type Output = Mat4x4;
    fn div(self, scalar: f32) -> Mat4x4{
        let mut dat: [[f32; 4]; 4] = [[0.0; 4]; 4];
        dat[0][0] = self.e[0][0] / scalar; dat[0][1] = self.e[0][1] / scalar; dat[0][2] = self.e[0][2] / scalar; dat[0][3] = self.e[0][3] / scalar;
        dat[1][0] = self.e[1][0] / scalar; dat[1][1] = self.e[1][1] / scalar; dat[1][2] = self.e[1][2] / scalar; dat[1][3] = self.e[1][3] / scalar;
        dat[2][0] = self.e[2][0] / scalar; dat[2][1] = self.e[2][1] / scalar; dat[2][2] = self.e[2][2] / scalar; dat[2][3] = self.e[2][3] / scalar;
        dat[3][0] = self.e[3][0] / scalar; dat[3][1] = self.e[3][1] / scalar; dat[3][2] = self.e[3][2] / scalar; dat[3][3] = self.e[3][3] / scalar;
        Mat4x4{ e : dat }
    }
}

impl DivAssign<f32> for Mat4x4 {
    fn div_assign(&mut self, scalar: f32) {
        self.e[0][0] *= scalar; self.e[0][1] *= scalar; self.e[0][2] *= scalar; self.e[0][3] *= scalar;
        self.e[1][0] *= scalar; self.e[1][1] *= scalar; self.e[1][2] *= scalar; self.e[1][3] *= scalar;
        self.e[2][0] *= scalar; self.e[2][1] *= scalar; self.e[2][2] *= scalar; self.e[2][3] *= scalar;
        self.e[3][0] *= scalar; self.e[3][1] *= scalar; self.e[3][2] *= scalar; self.e[3][3] *= scalar;
    }
}

impl MulAssign<f32> for Mat4x4 {
    fn mul_assign(&mut self, scalar: f32) {
        self.e[0][0] /= scalar; self.e[0][1] /= scalar; self.e[0][2] /= scalar; self.e[0][3] /= scalar;
        self.e[1][0] /= scalar; self.e[1][1] /= scalar; self.e[1][2] /= scalar; self.e[1][3] /= scalar;
        self.e[2][0] /= scalar; self.e[2][1] /= scalar; self.e[2][2] /= scalar; self.e[2][3] /= scalar;
        self.e[3][0] /= scalar; self.e[3][1] /= scalar; self.e[3][2] /= scalar; self.e[3][3] /= scalar;
    }
}

impl AddAssign<Mat4x4> for Mat4x4 {
    fn add_assign(&mut self, m1: Mat4x4) {
        self.e[0][0] += m1.e[0][0]; self.e[0][1] += m1.e[0][1]; self.e[0][2] += m1.e[0][2]; self.e[0][3] += m1.e[0][3];
        self.e[1][0] += m1.e[1][0]; self.e[1][1] += m1.e[1][1]; self.e[1][2] += m1.e[1][2]; self.e[1][3] += m1.e[1][3];
        self.e[2][0] += m1.e[2][0]; self.e[2][1] += m1.e[2][1]; self.e[2][2] += m1.e[2][2]; self.e[2][3] += m1.e[2][3];
        self.e[3][0] += m1.e[3][0]; self.e[3][1] += m1.e[3][1]; self.e[3][2] += m1.e[3][2]; self.e[3][3] += m1.e[3][3];
    }   
}

impl SubAssign<Mat4x4> for Mat4x4 {
    fn sub_assign(&mut self, m1: Mat4x4) {
        self.e[0][0] -= m1.e[0][0]; self.e[0][1] -= m1.e[0][1]; self.e[0][2] -= m1.e[0][2]; self.e[0][3] -= m1.e[0][3];
        self.e[1][0] -= m1.e[1][0]; self.e[1][1] -= m1.e[1][1]; self.e[1][2] -= m1.e[1][2]; self.e[1][3] -= m1.e[1][3];
        self.e[2][0] -= m1.e[2][0]; self.e[2][1] -= m1.e[2][1]; self.e[2][2] -= m1.e[2][2]; self.e[2][3] -= m1.e[2][3];
        self.e[3][0] -= m1.e[3][0]; self.e[3][1] -= m1.e[3][1]; self.e[3][2] -= m1.e[3][2]; self.e[3][3] -= m1.e[3][3];
    }
}

impl MulAssign<&Mat4x4> for Mat4x4 {
    fn mul_assign(&mut self, m1: &Mat4x4) {
        let mut d = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    d[i][j] += self.e[i][k] * m1.e[k][j];
                }
            }
        }
        self.e = d;
    }
}

