// tesseract
// bottom line the math is not there yet
// just going to write in some 4d vertices
use crate::util::helpers::projection_4d3d;
pub struct HyperCube {
    pub vertices: [[f32; 4]; 16],
}
impl HyperCube {
    pub fn new() -> HyperCube {
        let vertices = [            
        [-1.0, -1.0, -1.0,  1.0],
        [ 1.0, -1.0, -1.0,  1.0],
        [ 1.0,  1.0, -1.0,  1.0],
        [-1.0,  1.0, -1.0,  1.0],
        [-1.0, -1.0,  1.0,  1.0],
        [ 1.0, -1.0,  1.0,  1.0],
        [ 1.0,  1.0,  1.0,  1.0],
        [-1.0,  1.0,  1.0,  1.0],
        [-1.0, -1.0, -1.0, -1.0],
        [ 1.0, -1.0, -1.0, -1.0],
        [ 1.0,  1.0, -1.0, -1.0],
        [-1.0,  1.0, -1.0, -1.0],
        [-1.0, -1.0,  1.0, -1.0],
        [ 1.0, -1.0,  1.0, -1.0],
        [ 1.0,  1.0,  1.0, -1.0],
        [-1.0,  1.0,  1.0, -1.0]];

        HyperCube{vertices: vertices}
    }
    pub fn vertices_in_vec4(vertices: &[[f32; 4]; 16]) -> Vec<[f32; 4]> {
        let mut v4s = Vec::new();
        for v in vertices.iter() {
            v4s.push(*v);
        }
        return v4s;
    }
    pub fn vertices_in_vec3(&self) -> Vec<[f32; 3]> {
        let v4s = Self::vertices_in_vec4(&self.vertices);
        let v3s = projection_4d3d(v4s);
        return v3s;
    }
}

