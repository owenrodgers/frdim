
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
            [0.0, 1.0, 0.0,  0.0, 1.0, 0.0,  0.0, 0.0, 0.0],

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
/*
pub struct Object{
    // object class for housing vertices
    // should have its own rotation, scaling, translating
    // vertices composed of triangles, vector?
    pub vertices : Vec<Triangle>,
    pub rotation_x : &Mat3x3, pub rotation_y : &Mat3x3, pub rotation_z : &Mat3x3,
    pub scale_x : &f32, pub scale_y : &f32, pub scale_z : &f32,
    pub translation_x : &f32, pub translation_y : &f32, pub translation_z : &f32,
    pub projection : &Mat4x4,

}
impl Object{
    pub fn new(verts: Vec<Triangle>, rmx: &Mat3x3, rmy: &Mat3x3, rmz: &Mat3x3,
               sclx: &f32, scly: &f32, sclz: &f32,
               tx: &f32, ty: &f32, tx: &f32
               projection : &Mat4x4 ) -> Object{

        Object{vertices : verts,
               rotation_x : rmx, rotation_y : rmy, rotation_z : rmz,
               scale_x : sclx,   scale_y : scly,   scale_z : sclz,
               translation_x : tx, translation_y : ty, translation_z : ty,
               projection : projection_matrix}
    }
    pub fn process(&mut self){
        // returns f32 pointer of triangles making up mesh
        for triangle in vertices.iter_mut(){
            // general rotations
            triangle.rotate(&self.rotation_x);
            triangle.rotate(&self.rotation_y);
            triangle.rotate(&self.rotation_z); 
    
            // for projection on screen
            triangle.translate_z(&3.0);
            triangle.project(&self.projection);
            triangle.translate_x(&1.0);
            triangle.translate_y(&1.0);
    
            triangle.scale_x(&self.scale_x);
            triangle.scale_y(&self.scale_y); 
        }
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
    pub x : f32, pub y : f32, pub z : f32,
}
impl Vec3f {
    pub fn new(dat : &[f32]) -> Vec3f{
        let d : [f32; 3] = [dat[0], dat[1], dat[2]];
        Vec3f{ e : d, x : d[0], y : d[1], z : d[2] }
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

}
