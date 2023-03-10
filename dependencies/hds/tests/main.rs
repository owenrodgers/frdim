

use hds::conics::hypercone::HyperCone;
use hds::conics::hyperplane::HyperPlane;
use hds::conics::hyperconic::HyperConic;

use hds::tesseracts::tesseract::HyperCube;
use hds::util::helpers::projection_4d3d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hchpintersect_1() {
        let csteep = 2.0;
        let cheight = 5.0;
        let hcone = HyperCone::new(csteep, cheight);
        let hplane = HyperPlane::new(1.0, 0.0, 1.0, 3.0);
        let hconic = HyperConic::new(hcone, hplane);
        println!("{:?}", hconic.conic_coefficients);
    }

    #[test]
    fn test_tess423_1() {
        // bring the 4 vectors to 3 with projection matrix
        let hc = HyperCube::new();
        let vertices_four = hc.vertices_in_vec();
        let vertices_three = projection_4d3d(&vertices_four);
        for v in vertices_three.iter() {
            println!("{:?}", v);
        }

        
    }

}
