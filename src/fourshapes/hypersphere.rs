// slice of hypersphere struct

// contains a spherical mesh, radius to scale mesh

use crate::Mesh;

pub struct HyperSphere{
    pub hypersphere_radius: f32,
    pub w_naught: f32,

    pub slice_radius: f32,
    pub slice_mesh: Mesh,
    pub is_valid: bool,
}

impl HyperSphere{
    pub fn new(hypersphere_radius: f32, w_naught: f32) -> HyperSphere {
        let slice_radius = 1.0; //Self::compute_slice_radius(hypersphere_radius, w_naught).unwrap_or(0.0);
        let mut slice_mesh = Mesh::new();

        slice_mesh.build_triangles("models/poser_sphere.obj");
        //slice_mesh.scale_mesh(slice_radius);

        HyperSphere{ 
            hypersphere_radius: hypersphere_radius,
            w_naught: w_naught,
            slice_radius: slice_radius,
            slice_mesh: slice_mesh,
            is_valid: true }
    }
    pub fn update_mesh(&mut self, w0: f32) {
        // is the w0 in a valid range ?
        // yes -> scale mesh accordingly
        // no -> do nothing
        if w0 > 0.0 && w0 < self.hypersphere_radius {
            // find slice radius
            // scale mesh
            // k can never be 0
            let new_slice_radius = Self::compute_slice_radius(self.hypersphere_radius, w0).unwrap_or(1.0);
            let k = new_slice_radius / self.slice_radius;
            self.slice_mesh.scale_mesh(k);
            self.slice_radius = new_slice_radius;
            self.w_naught = w0;
        }
    }
    pub fn valid_slice(&mut self) -> bool {
        if self.w_naught > 0.0 && self.w_naught < self.hypersphere_radius {
            self.is_valid = true;
            return true;
        } else {
            self.is_valid = false;
            return false
        }
    } 

    fn compute_slice_radius(hypersphere_radius: f32, w0: f32) -> Option<f32> {
        // https://www.desmos.com/calculator/viiz8ekenw
        if w0 < hypersphere_radius {
            let slice_radius: f32 = (w0 * (2.0 * hypersphere_radius - w0 ) ).sqrt();
            if slice_radius <= 0.0 {
                Some(1.0)
            } else {
                Some(slice_radius.abs())
            }
        } else {
            None
        }
    }

}




