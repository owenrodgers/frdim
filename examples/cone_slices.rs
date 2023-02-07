extern crate frdim;
use frdim::fourshapes::conics::{ConicSection, Plane, Cone};
use frdim::rendering::render::{render};

// high level implementation of frdim
// sld2 is under the hood
fn main() -> Result<(), String> {
    let cone_steepness: f32 = 7.0;
    let plane_coefs: [f32; 4] = [-10.0, 3.5, -0.5, 10.0];

    let plane: Plane = Plane::new(plane_coefs[0], plane_coefs[1], plane_coefs[2], plane_coefs[3]);
    let cone: Cone = Cone::new(cone_steepness);
    let mut conic_section: ConicSection = ConicSection::new(cone, plane);
    
    render(&mut conic_section)?;

    Ok(())
}
