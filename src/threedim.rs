use std::ops::{Sub, Add};

// housing structs for 3 dimensional objects
pub struct WireCube{
    pub vertices: [[f32; 9]; 12],
}
impl WireCube {
    pub fn new() -> WireCube{
        let verts:[[f32; 9]; 12] = [
            // south
            [0.0, 0.0, 0.0,  0.0, 1.0, 0.0,  1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0,  1.0, 1.0, 0.0,  1.0, 0.0, 0.0],
            
            // east
            [1.0, 0.0, 0.0,  1.0, 1.0, 0.0,  1.0, 1.0, 1.0],
            [1.0, 0.0, 0.0,  1.0, 1.0, 1.0,  1.0, 0.0, 1.0],
            
            // north
            [1.0, 0.0, 1.0,  1.0, 1.0, 1.0,  0.0, 1.0, 1.0],
            [1.0, 0.0, 1.0,  0.0, 1.0, 1.0,  0.0, 0.0, 1.0],
            
            // west
            [0.0, 0.0, 1.0,  0.0, 1.0, 1.0,  0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0,  0.0, 1.0, 0.0,  0.0, 0.0, 0.0],

            // top
            [0.0, 1.0, 0.0,  0.0, 1.0, 1.0,  1.0, 1.0, 1.0],
            [0.0, 1.0, 0.0,  1.0, 1.0, 1.0,  1.0, 1.0, 0.0],

            // bottom
            [1.0, 0.0, 1.0,  0.0, 0.0, 1.0,  0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0,  0.0, 0.0, 0.0,  1.0, 0.0, 0.0],
        ];
        WireCube{vertices: verts}
    }
}

// floor
/*
pub struct Floor{
    pub vertices: [[f32; 9]; 2],
}
impl Floor{
    pub fn new() -> Floor{
        let verts:[[f32; 9]; 2] = [
            [1.0, 0.0, 1.0,  0.0, 0.0, 1.0,  0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0,  0.0, 0.0, 0.0,  1.0, 0.0, 0.0],
        ];
        Floor{vertices: verts}
    }
}
*/

// ---- 4x4 Matrices ----
pub struct Mat4x4{
    e: [f32; 16],
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

// ---- 3x3 Matrices ----
pub struct Mat3x3{
    e: [f32; 9],
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



// ---- Vec3f ----

#[derive(Copy, Clone)]
pub struct Vec3f{
    pub e : [f32; 3],
}
impl Vec3f {
    pub fn new(dat : &[f32]) -> Vec3f{
        let d : [f32; 3] = [dat[0], dat[1], dat[2]];
        Vec3f{ e : d}
    }
    pub fn dot(&mut self, v: &Vec3f) -> f32{
        return self.e[0]*v.e[0] + self.e[1]*v.e[1] + self.e[2]*v.e[2];
    }
    pub fn cross(&mut self, v1: &Vec3f) -> Vec3f{
        return Vec3f::new(&[ self.e[1]*v1.e[2] - self.e[2]*v1.e[1],
                                self.e[2]*v1.e[0] - self.e[0]*v1.e[2],
                                self.e[0]*v1.e[1] - self.e[1]*v1.e[0] ]);
    }
    pub fn normalize(&mut self){
        let len = Self::length(&self);
        self.e[0] /= len;
        self.e[1] /= len;
        self.e[2] /= len;
    }
    fn length(&self) -> f32{
        let l = self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
        return l.sqrt();
    }
    pub fn x(&self) -> f32{
        return self.e[0];
    }
    pub fn y(&self) -> f32{
        return self.e[1];
    }
    pub fn z(&self) -> f32{
        return self.e[2];
    }
}
// traits (operator overloads)
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

// ---- Triangle ----
pub struct Triangle{
    pub vertices : [Vec3f ; 3],
}
impl Triangle {
    pub fn new(raw_vertices: &[f32; 9]) -> Triangle {
        let va = Vec3f::new(&raw_vertices[0..3]);
        let vb = Vec3f::new(&raw_vertices[3..6]);
        let vc = Vec3f::new(&raw_vertices[6..9]);
        Triangle{ vertices: [va, vb, vc] }
    }
    pub fn rotate(&mut self, rotation_matrix: &Mat3x3){
        // apply rotation matrix
        self.vertices = [Self::dot_v3m3(self.vertices[0], &rotation_matrix),
                         Self::dot_v3m3(self.vertices[1], &rotation_matrix),
                         Self::dot_v3m3(self.vertices[2], &rotation_matrix)]
    }
    pub fn project(&mut self, projection_matrix: &Mat4x4){
        // apply projection matrix
        self.vertices = [Self::dot_v4m4(self.vertices[0], &projection_matrix),
                         Self::dot_v4m4(self.vertices[1], &projection_matrix),
                         Self::dot_v4m4(self.vertices[2], &projection_matrix)];
    }

    // translate individual components
    pub fn translate_x(&mut self, val: &f32){
        for vn in self.vertices.iter_mut(){
            vn.e[0] += val;
        }
    }
    pub fn translate_y(&mut self, val: &f32){
        for vn in self.vertices.iter_mut(){
            vn.e[1] += val;
        }
    }
    pub fn translate_z(&mut self, val: &f32){
        for vn in self.vertices.iter_mut(){
            vn.e[2] += val;
        }
    }

    pub fn scale_x(&mut self, val: &f32){
        for vn in self.vertices.iter_mut(){
            vn.e[0] *= val;
        }
    }
    pub fn scale_y(&mut self, val: &f32){
        for vn in self.vertices.iter_mut(){
            vn.e[1] *= val;
        }
    }
    pub fn scale_z(&mut self, val: &f32){
        for vn in self.vertices.iter_mut(){
            vn.e[2] *= val;
        }
    }

    fn dot_v4m4(vector: Vec3f, matrix: &Mat4x4) -> Vec3f{
        let mut d: [f32; 3] = [0.0; 3];
        let w: f32;
        d[0] = vector.e[0]*matrix.e[0] + vector.e[1]*matrix.e[4] + vector.e[2]*matrix.e[8] + matrix.e[12];
        d[1] = vector.e[0]*matrix.e[1] + vector.e[1]*matrix.e[5] + vector.e[2]*matrix.e[9] + matrix.e[13];
        d[2] = vector.e[0]*matrix.e[2] + vector.e[1]*matrix.e[6] + vector.e[2]*matrix.e[10] + matrix.e[14];
        w = vector.e[0]*matrix.e[3] + vector.e[1]*matrix.e[7] + vector.e[2]*matrix.e[11] + matrix.e[15];
        if w != 0.0{
            d[0] /= w; d[1] /= w; d[2] /= w;
        }
        return Vec3f::new(&d); 
    }
    fn dot_v3m3(vector: Vec3f, matrix: &Mat3x3) -> Vec3f{
        let mut d: [f32; 3] = [0.0; 3];
        d[0] = vector.e[0]*matrix.e[0] + vector.e[1]*matrix.e[1] + vector.e[2]*matrix.e[2];
        d[1] = vector.e[0]*matrix.e[3] + vector.e[1]*matrix.e[4] + vector.e[2]*matrix.e[5];
        d[2] = vector.e[0]*matrix.e[6] + vector.e[1]*matrix.e[7] + vector.e[2]*matrix.e[8];
        return Vec3f::new(&d);
    }
    pub fn compute_normal(&mut self) -> Vec3f{
        let mut norm: Vec3f = Vec3f::new(&[0.0; 3]);
        let mut v1: Vec3f = Vec3f::new(&[0.0; 3]);
        let mut v2: Vec3f = Vec3f::new(&[0.0; 3]);
        v1.e[0] = self.vertices[1].e[0] - self.vertices[0].e[0];
        v1.e[1] = self.vertices[1].e[1] - self.vertices[0].e[1];
        v1.e[2] = self.vertices[1].e[2] - self.vertices[0].e[2];

        v2.e[0] = self.vertices[2].e[0] - self.vertices[0].e[0];
        v2.e[1] = self.vertices[2].e[1] - self.vertices[0].e[1];
        v2.e[2] = self.vertices[2].e[2] - self.vertices[0].e[2];

        norm.e[0] = v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1];
        norm.e[1] = v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2];
        norm.e[2] = v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0];

        return norm;

    }

}
