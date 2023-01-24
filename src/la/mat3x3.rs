// ---- 3x3 Matrices ----
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
    pub fn rotation_y(&mut self, theta: &f32){
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
