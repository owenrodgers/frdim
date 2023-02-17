
// governing ConicSection
// from the points 

use crate::Vec3f;
use crate::ConicSection;
use crate::Mat3x3;
pub struct HyperConic {
    pub conic: ConicSection,
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

