
use jive::jives::jivemodel::JiveModel;
use core::f32::consts::PI;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pmult() {
        println!("rotations are broken");
        let mut hyperconic_model = JiveModel::new(Vec::new());
        println!("{:?}", hyperconic_model.transform_matrix.e);
        hyperconic_model.rotate_pitch(PI / 4.0);
        println!("{:?}", hyperconic_model.transform_matrix.e);
        hyperconic_model.rotate_roll(PI / 3.0);
        println!("{:?}", hyperconic_model.transform_matrix.e);
    }
}