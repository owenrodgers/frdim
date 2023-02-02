#[allow(dead_code)]
#[allow(unused_imports)]


//cargo build && cargo test -- --nocapture
use frdim::fourshapes::hypersphere::HyperSphere;
use frdim::fourshapes::conics::{Cone, Plane, ConicSection};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cc_1() {
        // cone steepness
        let c_stp = 1.0;
        // plane = [a,b,c,d] where: ax + bx + cz = d
        let plane: [f32; 4] = [0.0, 0.0, 1.0, 2.0];

        let mut c: Cone = Cone::new(c_stp);
        let mut p: Plane = Plane::new(0.0, 0.0, 1.0, 2.0);
        let mut cs: ConicSection = ConicSection::new(c, p);
        let ans = [1.0, 0.0, 1.0, 0.0, 0.0, -4.0];
        assert_eq!(cs.conic_coef, ans);
    }

    #[test]
    fn test_cc_2() {
        let c_stp = 1.0;
        let plane: [f32; 4] = [1.0, 2.0, 1.0, 5.0];

        let c: Cone = Cone::new(c_stp);
        let p: Plane = Plane::new(plane[0], plane[1], plane[2], plane[3]);
        let cs: ConicSection = ConicSection::new(c, p);

        let ans = [0.0, -4.0, -3.0, 10.0, 20.0, -25.0];
        assert_eq!(cs.conic_coef, ans);
    }

    #[test]
    fn test_cc_3() {
        let c_stp = 1.0;
        let plane: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let c: Cone = Cone::new(c_stp);
        let p: Plane = Plane::new(plane[0], plane[1], plane[2], plane[3]);
        let cs: ConicSection = ConicSection::new(c, p);

        let ans = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(cs.conic_coef, ans);
    }

    #[test]
    fn test_is_valid_1() {
        let c_stp = 3.0;
        let plane: [f32; 4] = [-2.0, 7.0, 1.0, 4.0];

        let c: Cone = Cone::new(c_stp);
        let p: Plane = Plane::new(plane[0], plane[1], plane[2], plane[3]);
        let cs: ConicSection = ConicSection::new(c, p);

        let x: f32 = 0.0;
        let y: f32 = 1.0;
        let eval = cs.is_valid(x,y, 2);

        assert_eq!(eval, 0.0);
    }

    #[test]
    fn test_is_valid_2() {
        let c_stp = 6.0;
        let plane: [f32; 4] = [-2.0, 4.0, 1.0, 4.0];

        let c: Cone = Cone::new(c_stp);
        let p: Plane = Plane::new(plane[0], plane[1], plane[2], plane[3]);
        let cs: ConicSection = ConicSection::new(c, p);

        let x: f32 = 0.0;
        let y: f32 = -2.0;
        let eval = cs.is_valid(x,y, 2);

        assert_eq!(eval, 0.0);
    }

    #[test]
    fn test_is_valid_3() {

        let c_stp = 6.0;
        let plane: [f32; 4] = [-2.0, 3.5, -0.5, 4.0];

        let c: Cone = Cone::new(c_stp);
        let p: Plane = Plane::new(plane[0], plane[1], plane[2], plane[3]);
        let cs: ConicSection = ConicSection::new(c, p);

        let x: f32 = -0.86;
        let y: f32 = 5.0;
        let eval = cs.is_valid(x,y, 1);
        println!("{:?}", eval);
        assert_eq!(eval, 0.0);
        
    }



}