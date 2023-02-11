#[allow(dead_code)]
#[allow(unused_imports)]


//cargo build && cargo test -- --nocapture

//use frdim::fourshapes::hyperconics::{polynomial_roots, eigenvalues, eigenvectors, align_conic};
use frdim::la::matrix::Mat2x2;
use frdim::fourshapes::conics::{Cone, Plane, ConicSection};
#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_eigenvalues_1() {
        // test eigenvalues
        // eigenvectors
        // multiply operator
        let l1 = 3.0 + 5.0_f32.sqrt();
        let l2 = 3.0 - 5.0_f32.sqrt();

        let mat: Mat2x2 = Mat2x2::new([4.0, -2.0, -2.0, 2.0]);
        let (el1, el2) = mat.eigenvalues();
        assert_eq!((el1, el2), (l1, l2)); // eigenvalues
    }

    //#[test]
    fn test_eigenvectors_1() {
        let mat: Mat2x2 = Mat2x2::new([10.0, 6.0, 6.0, 10.0]);
        let (lambda1, lambda2) = mat.eigenvalues();
        let ev1 = [1.0, 1.0]; // eigenvector for lambda1
        let ev2 = [-1.0, 1.0]; // eigenvector for lambda2

        let (v1, v2) = mat.eigenvectors(lambda1, lambda2);
        assert_eq!((ev1, ev2), (v1, v2));
    }

    //#[test]
    fn test_multiply_1() {
        let m1 = Mat2x2::new([2.0, 3.0, 8.0, 2.0]);
        let m2 = Mat2x2::new([1.0, 4.0, 7.0, 0.0]);
        let m3 = m1 * m2;
        assert_eq!(m3.e, [23.0, 8.0, 22.0, 32.0]);

    }

    //#[test]
    fn test_multiply_scalar_1() {
        let m1 = Mat2x2::new([2.0, 3.0, 8.0, 2.0]);
        let scalar = 2.0;
        let m3 = m1 * scalar;

        assert_eq!(m3.e, [4.0, 6.0, 16.0, 4.0]);
    }

    #[test]
    fn test_align_1() {
        let cone_steepness: f32 = 7.0;
        let plane_coefs: [f32; 4] = [-10.0, 3.5, -0.5, 10.0];
        let plane: Plane = Plane::new(plane_coefs[0], plane_coefs[1], plane_coefs[2], plane_coefs[3]);
        let cone: Cone = Cone::new(cone_steepness);
    
        let mut conic: ConicSection = ConicSection::new(cone, plane);
        conic.conic_coef = [1.25, -2.0, 1.25, 0.0, 0.0, 1.0];
        let aligned_conic = conic.align();
        println!("{:?}", aligned_conic.conic_coef);
    }

}