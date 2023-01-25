// for test directory

use crate::la::vec3f::Vec3f;
use crate::la::mat3x3::Mat3x3;
use crate::la::mat4x4::Mat4x4;
pub mod la;

//
use crate::meshrender::triangle::Triangle;
use crate::meshrender::mesh::Mesh;
use crate::meshrender::render::{fill_tri};
pub mod meshrender;


use crate::fourshapes::hypersphere::HsSlice;
pub mod fourshapes;

