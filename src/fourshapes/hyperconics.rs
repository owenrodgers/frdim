
// governing ConicSection
// from the points 

//use crate::Vec3f;
use crate::ConicSection;
//use crate::Mat3x3;
use crate::Cone;
use crate::Plane;

pub struct HyperCone {
    pub height: f32,
    pub steepness: f32,
}
impl HyperCone {
    pub fn new(cone_height: f32, cone_steepness: f32) -> HyperCone {
        HyperCone{height: cone_height, steepness: cone_steepness}
    }
    pub fn intersection(&self, plane: &HyperPlane) -> ConicSection {
        let mut cc: [f32; 6] = [0.0; 6];
        cc[0] = (plane.c * plane.c) * (self.steepness * self.steepness) - (plane.a * plane.a);
        cc[1] = -2.0 * plane.a * plane.b;
        cc[2] = (plane.c * plane.c) * (self.steepness * self.steepness) - (plane.b * plane.b);
        cc[3] = 2.0 * plane.a * plane.d;
        cc[4] = 2.0 * plane.b * plane.d;
        cc[5] = -1.0 * plane.d * plane.d;
        let normal_plane = Plane{a: plane.a, b: plane.b, c: plane.c, d: plane.d};
        let normal_cone = Cone{m: self.steepness, h: 2.0};

        return ConicSection{cone: normal_cone, plane: normal_plane, conic_coef: cc}
    }

}

pub struct HyperPlane {
    // a,b,c,d
    // ax + by + cz = d
    pub a: f32, pub b: f32,
    pub c: f32, pub d: f32,
}
impl HyperPlane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> HyperPlane {
        HyperPlane{a: a, b: b, c: c, d: d}
    }
}
/*
pub struct HyperConic {
    pub conic_coefficients: [f32; 6],
    pub cone: HyperCone,
    pub plane: HyperPlane,
    //pointcloud: Vec<Vec3f>,
}

impl HyperConic {
    pub fn new(conic: ConicSection) -> HyperConic {
        // find projection onto xy plane
        // rotate around y axis
        // push onto pointcloud
        HyperConic{conic: conic}
    }

    pub fn revolve(&mut self, degree_increment: f32) -> Vec<Vec3f> {
        let mut pointcloud: Vec<Vec3f> = Vec::new();
        let original_pointset = self.conic.get_points(-1, 1, -1, 1);
        
        let steps = (180.0 / degree_increment) as usize;
        // if degree increment is 15 then our total steps will be 12
        
        let mut current_theta = 0.0;
        let mut rotation_matrix = Mat3x3::new();
        rotation_matrix.rotation_y(current_theta);
        

        for _s in 0..steps {
            // rotate original points clockwise about y axis by current theta
            // increment theta
            // let rotation_matrix = counter clockwise about y axis (current_theta)
            
            rotation_matrix.rotation_y(current_theta);

            for point in original_pointset.iter() {
                let (cx, cy) = point;
                let original_point = Vec3f::from([*cx, *cy, 0.0]);
                let rotated_point = original_point * rotation_matrix;
                pointcloud.push(rotated_point);
            }

            current_theta += degree_increment;
            
        }

        pointcloud
    }
    /*
    fn build_pointcloud(conic_section: ConicSection, rotation_increment: f32) {
        // compute conic section in xy space
        // revolve around y axis by the rotation increment
        
        let xyspace_points: Vec<Vec3f> = conic_section.get_points();
    }
    */
}

*/