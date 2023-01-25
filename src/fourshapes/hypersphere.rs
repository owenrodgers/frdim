// slice of hypersphere struct

// contains a spherical mesh, radius to scale mesh

use crate::Mesh;

pub struct HsSlice{
    pub radius: f32,
    pub mesh: Mesh,
}

// on radius update, scale the mesh

impl HsSlice{
    pub fn new() -> HsSlice {
        let mut m = Mesh::new();
        m.build_triangles("models/poser_sphere.obj");

        HsSlice{ radius : 1.0, mesh : m }
    }
    pub fn mesh_update(&mut self, r: f32){
        let k = r / self.radius;
        self.radius = r;

        for tri in self.mesh.triangles.iter_mut() {
            tri.scale(k);
        }
    }

}




