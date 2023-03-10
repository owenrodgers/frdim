use rola::vector::vec3f::{projection_multiply, Vec3f};
use rola::matrix::mat4x4::Mat4x4;
use core::f32::consts::PI;
use rola::matrix::mat3x3::Mat3x3;

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn pmult() {
        let v = Vec3f::new([1.0, 1.0, 1.0]);
        let mut m = Mat4x4::new();
        m.projection(&800.0, &800.0, &90.0, &1000.0, &1.0);
        let v2 = projection_multiply(&v, &m);
        println!("{:?}", v2.e);
        println!("testing");
    }
    #[test]
    fn rmat() {
        let rm = Mat3x3::pitch(PI / 2.0);
        let mut id = Mat3x3::identity();

        id *= &rm;
        println!("{:?}", id.e);

    }
}
