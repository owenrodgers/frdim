
extern crate rola;
use rola::vector::vec3f::Vec3f;

use rola::matrix::mat4x4::Mat4x4;

pub struct JiveModel {
    pub vertices: Vec<Vec3f>,
    pub transform_matrix: Mat4x4,
}

impl JiveModel {
    pub fn new(vertex_data: Vec<[f32; 3]>) -> JiveModel {
        let vertices = vertex_data.iter()
                                  .map(|x| Vec3f::new(*x))
                                  .collect::<Vec<Vec3f>>();
        let transform = Mat4x4::identity();
        JiveModel{vertices: vertices, transform_matrix: transform}   
    }
    pub fn update(&mut self, vdata: Vec<[f32; 3]>) {
        self.vertices = vdata.iter()
                        .map(|x| Vec3f::new(*x))
                        .collect::<Vec<Vec3f>>();
    }
    pub fn raw_vertices(&self) -> Vec<[f32; 3]> {
        let rv = self.vertices.iter()
                              .map(|x| x.e)
                              .collect::<Vec<[f32; 3]>>();
        return rv;
    }
    pub fn rotate_x(&mut self, theta: f32) {
        let rollmat = Mat4x4::rotate_x(theta);
        self.transform_matrix *= &rollmat;
    }
    pub fn rotate_y(&mut self, theta: f32) {
        let pitchmat = Mat4x4::rotate_y(theta);
        self.transform_matrix *= &pitchmat
    }
    pub fn rotate_z(&mut self, theta: f32) {
        let yawmat = Mat4x4::rotate_z(theta);
        self.transform_matrix *= &yawmat; 
    }
    pub fn apply_translation(&mut self, t: [f32; 3]) {
        self.transform_matrix.e[0][3] += t[0];
        self.transform_matrix.e[1][3] += t[1];
        self.transform_matrix.e[2][3] += t[2];
    }
}