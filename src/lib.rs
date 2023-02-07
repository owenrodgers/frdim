

use crate::la::vec3f::Vec3f;
use crate::la::mat3x3::Mat3x3;
use crate::la::mat4x4::Mat4x4;
pub mod la;

use crate::meshes::triangle::Triangle;
use crate::meshes::mesh::Mesh;
pub mod meshes;

//use crate::rendering::render::fill_tri;
pub mod rendering;

//use crate::fourshapes::hypersphere::HyperSphere;
//use crate::fourshapes::conics::Cone;
//use crate::fourshapes::conics::Plane;
//use crate::fourshapes::conics::ConicSection;
pub mod fourshapes;


