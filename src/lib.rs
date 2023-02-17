

use crate::la::vec3f::Vec3f;
use crate::la::matrix::Mat4x4;
use crate::la::matrix::Mat3x3;
use crate::la::matrix::Mat2x2;

pub mod la;

use crate::meshes::triangle::Triangle;
use crate::meshes::mesh::Mesh;
//use crate::meshes::surfacemesh::Surface;
pub mod meshes;

//use crate::rendering::render::fill_tri;
pub mod rendering;

//use crate::fourshapes::hypersphere::HyperSphere;
//use crate::fourshapes::conics::Cone;
//use crate::fourshapes::conics::Plane;
use crate::fourshapes::conics::ConicSection;
pub mod fourshapes;


