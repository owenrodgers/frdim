use std::ops::{MulAssign, DivAssign, AddAssign, SubAssign};
use std::ops::{Sub, Add, Mul, Div};

pub struct Mat3x3{
    pub e: [[f32; 3]; 3],
}
impl Mat3x3{
    pub fn new() -> Mat3x3{
        let dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        Mat3x3{ e: dat}
    }
    
    pub fn identity() -> Mat3x3 {
        let mut dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        dat[0][0] = 1.0; 
        dat[1][1] = 1.0; 
        dat[2][2] = 1.0;
        Mat3x3{ e : dat }
    }
    
    pub fn roll(theta: f32) -> Mat3x3 {
        let mut dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        dat[0][0] = 1.0;
        dat[1][1] = theta.cos();
        dat[1][2] = -1.0 * theta.sin();
        dat[2][1] = theta.sin();
        dat[2][2] = theta.cos();
        Mat3x3{ e : dat }
    }
    
    pub fn pitch(theta: f32) -> Mat3x3 {
        let mut dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        dat[0][0] = theta.cos();
        dat[0][2] = theta.sin();
        dat[1][1] = 1.0;
        dat[2][0] = -1.0 * theta.sin();
        dat[2][2] = theta.cos();
        Mat3x3{ e : dat }
    }
    
    pub fn yaw(theta: f32) -> Mat3x3 {
        let mut dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        dat[0][0] = theta.cos();
        dat[0][1] = -1.0 * theta.sin();
        dat[1][0] = theta.sin();
        dat[1][1] = theta.cos();
        dat[2][2] = 1.0;
        Mat3x3{ e : dat }
    }
    
}

// overloads

impl Sub for Mat3x3{
    type Output = Mat3x3;
    fn sub(self, m1: Mat3x3) -> Mat3x3{
        let mut dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        dat[0][0] = self.e[0][0] - m1.e[0][0]; dat[0][1] = self.e[0][1] - m1.e[0][1]; dat[0][2] = self.e[0][2] - m1.e[0][2];
        dat[1][0] = self.e[1][0] - m1.e[1][0]; dat[1][1] = self.e[1][1] - m1.e[1][1]; dat[1][2] = self.e[1][2] - m1.e[1][2];
        dat[2][0] = self.e[2][0] - m1.e[2][0]; dat[2][1] = self.e[2][1] - m1.e[2][1]; dat[2][2] = self.e[2][2] - m1.e[2][2];
        Mat3x3{ e : dat }
    }
}
impl Add for Mat3x3{
    type Output = Mat3x3;
    fn add(self, m1: Mat3x3) -> Mat3x3{
        let mut dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        dat[0][0] = self.e[0][0] + m1.e[0][0]; dat[0][1] = self.e[0][1] + m1.e[0][1]; dat[0][2] = self.e[0][2] + m1.e[0][2];
        dat[1][0] = self.e[1][0] + m1.e[1][0]; dat[1][1] = self.e[1][1] + m1.e[1][1]; dat[1][2] = self.e[1][2] + m1.e[1][2];
        dat[2][0] = self.e[2][0] + m1.e[2][0]; dat[2][1] = self.e[2][1] + m1.e[2][1]; dat[2][2] = self.e[2][2] + m1.e[2][2];
        Mat3x3{ e : dat }
    }
}

impl Mul<f32> for Mat3x3{
    type Output = Mat3x3;
    fn mul(self, scalar: f32) -> Mat3x3{
        let mut dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        dat[0][0] = self.e[0][0] * scalar; dat[0][1] = self.e[0][1] * scalar; dat[0][2] = self.e[0][2] * scalar;
        dat[1][0] = self.e[1][0] * scalar; dat[1][1] = self.e[1][1] * scalar; dat[1][2] = self.e[1][2] * scalar;
        dat[2][0] = self.e[2][0] * scalar; dat[2][1] = self.e[2][1] * scalar; dat[2][2] = self.e[2][2] * scalar;
        Mat3x3{ e : dat }
    }
}

impl Div<f32> for Mat3x3{
    type Output = Mat3x3;
    fn div(self, scalar: f32) -> Mat3x3{
        let mut dat: [[f32; 3]; 3] = [[0.0; 3]; 3];
        dat[0][0] = self.e[0][0] / scalar; dat[0][1] = self.e[0][1] / scalar; dat[0][2] = self.e[0][2] / scalar;
        dat[1][0] = self.e[1][0] / scalar; dat[1][1] = self.e[1][1] / scalar; dat[1][2] = self.e[1][2] / scalar;
        dat[2][0] = self.e[2][0] / scalar; dat[2][1] = self.e[2][1] / scalar; dat[2][2] = self.e[2][2] / scalar;
        Mat3x3{ e : dat }
    }
}

impl DivAssign<f32> for Mat3x3 {
    fn div_assign(&mut self, scalar: f32) {
        self.e[0][0] /= scalar; self.e[0][1] /= scalar; self.e[0][2] /= scalar;
        self.e[1][0] /= scalar; self.e[1][1] /= scalar; self.e[1][2] /= scalar;
        self.e[2][0] /= scalar; self.e[2][1] /= scalar; self.e[2][2] /= scalar;
    }
}

impl MulAssign<f32> for Mat3x3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.e[0][0] *= scalar; self.e[0][1] *= scalar; self.e[0][2] *= scalar;
        self.e[1][0] *= scalar; self.e[1][1] *= scalar; self.e[1][2] *= scalar;
        self.e[2][0] *= scalar; self.e[2][1] *= scalar; self.e[2][2] *= scalar;
    }
}

impl AddAssign<Mat3x3> for Mat3x3 {
    fn add_assign(&mut self, m1: Mat3x3) {
        self.e[0][0] += m1.e[0][0]; self.e[0][1] += m1.e[0][1]; self.e[0][2] += m1.e[0][2];
        self.e[1][0] += m1.e[1][0]; self.e[1][1] += m1.e[1][1]; self.e[1][2] += m1.e[1][2];
        self.e[2][0] += m1.e[2][0]; self.e[2][1] += m1.e[2][1]; self.e[2][2] += m1.e[2][2];
    }
}

impl SubAssign<Mat3x3> for Mat3x3 {
    fn sub_assign(&mut self, m1: Mat3x3) {
        self.e[0][0] -= m1.e[0][0]; self.e[0][1] -= m1.e[0][1]; self.e[0][2] -= m1.e[0][2];
        self.e[1][0] -= m1.e[1][0]; self.e[1][1] -= m1.e[1][1]; self.e[1][2] -= m1.e[1][2];
        self.e[2][0] -= m1.e[2][0]; self.e[2][1] -= m1.e[2][1]; self.e[2][2] -= m1.e[2][2];
    }
}

impl MulAssign<&Mat3x3> for Mat3x3 {
    fn mul_assign(&mut self, m1: &Mat3x3) {
        let mut d = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    d[i][j] += self.e[i][k] * m1.e[k][j];
                }
            }
        }
        self.e = d;
    }
}





