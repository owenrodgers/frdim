use crate::Vec3f;
use crate::Mat3x3;
use crate::Mat4x4;

// ---- Triangle ----

pub struct Triangle{
    pub vertices : [Vec3f ; 3],
}
impl Triangle {
    pub fn new(v1r: &[f32; 3], v2r: &[f32; 3], v3r: &[f32; 3]) -> Triangle {
        let va = Vec3f::new(v1r);
        let vb = Vec3f::new(v2r);
        let vc = Vec3f::new(v3r);
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
    pub fn scale(&mut self, val: f32){
        for vn in self.vertices.iter_mut(){
            vn.e[0] *= val;
            vn.e[1] *= val;
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
        let mut v1 = self.vertices[1] - self.vertices[0];
        let v2 = self.vertices[2] - self.vertices[0];

        return v1.cross(&v2);
    }

}