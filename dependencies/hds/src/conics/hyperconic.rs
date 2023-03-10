use crate::HyperCone;
use crate::HyperPlane;
use crate::Surface;

// major surfaces we are concerned with rendering
const SPHERE: u8 = 1;
const ELLIPSOID: u8 = 2;
const HYPERBOLOID: u8 = 3;
const PARABOLOID: u8 = 4;

/*
const CONE: u8 = 5;
const PLANE: u8 = 6;
*/

pub struct HyperConic {
    pub hypercone: HyperCone,
    pub hyperplane: HyperPlane,
    pub conic_coefficients: [f32; 6],
}
impl HyperConic {
    pub fn new(hcone: HyperCone, hplane: HyperPlane) -> HyperConic {
        let c_coefs = hcone_hplane_intersection(&hcone, &hplane);
        HyperConic{hypercone: hcone, hyperplane: hplane, conic_coefficients: c_coefs}
    }
    pub fn update_plane_b(&mut self, value: f32) {
        self.hyperplane.b = value;
        self.conic_coefficients = hcone_hplane_intersection(&self.hypercone, &self.hyperplane);
    }

    pub fn conic_type(&self) -> u8 {
        let discriminant = self.conic_coefficients[1] * self.conic_coefficients[1] - 4.0 * self.conic_coefficients[0] * self.conic_coefficients[2];
        let conic_type;

        if discriminant < 0.0 {  
            if self.conic_coefficients[0] == self.conic_coefficients[2] {
                conic_type = SPHERE;
            } else {
                conic_type = ELLIPSOID;
            }
        } else if discriminant > 0.0 { 
            conic_type = HYPERBOLOID;
        } else {  
            conic_type = PARABOLOID;
        }
        return conic_type;
    }
}

pub fn hcone_hplane_intersection(hcone: &HyperCone, hplane: &HyperPlane) -> [f32; 6] {
    let mut cfs: [f32; 6] = [0.0; 6];
    // Ax^2 + Bxy + Cy^2 + Dx + Ey = F
    cfs[0] = hplane.c * hplane.c * hcone.steepness * hcone.steepness - hplane.a * hplane.a;
    // to avoid -0.0
    if hplane.a == 0.0 || hplane.b == 0.0 { cfs[1] = 0.0;} 
    else { cfs[1] = -2.0 * hplane.a * hplane.b; }
    
    cfs[2] = hplane.c * hplane.c * hcone.steepness * hcone.steepness - hplane.b * hplane.b;
    cfs[3] = 2.0 * hplane.a * hplane.d;                                              
    cfs[4] = 2.0 * hplane.b * hplane.d;                            
    cfs[5] = hplane.d * hplane.d;                                       
    cfs
}

pub fn vertices_from_hyperconic(hyperconic: &HyperConic) -> Vec<[f32; 3]> {
    let conic_type = hyperconic.conic_type();
    let conic_surface = Surface::new(conic_type);
    let vertices: Vec<[f32; 3]> = conic_surface.vertices(hyperconic.conic_coefficients);
    return vertices;
}



