
/*
    cone with steepness m modeled by
    (mx)^2 + (my)^2 = z^2
    intersected by plane modeled by
    ax + by + cz = d

    projection onto the xy plane is of the form:

    Ax^2 + Bxy + Cy^2 + Dx + Ey + F = 0
    Where:
        A = (c^2 * m^2 - a^2)
        B = -2.0 * a * b
        C = c^2 * m^2 - b^2
        D = 2.0 * a * d
        E = 2.0 * b * d
        F = -1.0 * d^2

    https://www.desmos.com/calculator/inzny6ittx 
    
*/

// render the conic section
use crate::Mat2x2;

#[derive(Default, Copy, Clone)]
pub struct Cone{
    pub m: f32,
}
impl Cone{
    pub fn new(steepness: f32) -> Cone {
        Cone{m: steepness}
    }
}


#[derive(Default, Copy, Clone)]
pub struct Plane{
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}
impl Plane{
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Plane{
        Plane{ a:a, b:b, c:c, d:d}
    }
}
pub struct ConicSection{
    pub cone: Cone,
    pub plane: Plane,
    pub conic_coef: [f32; 6],
}

impl ConicSection{
    pub fn new(cone: Cone, plane: Plane) -> ConicSection {
        let conic_coef: [f32; 6] = Self::compute_conic_coefficients(&cone, &plane);
        ConicSection{cone: cone, plane: plane, conic_coef: conic_coef}
    }

    pub fn get_points(&self, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Vec<(f32, f32)> {
        fn round(x: f32, decimals: u32) -> f32 {
            let y = 10i32.pow(decimals) as f32;
            (x * y).round() / y
        }

        let x_step: f32 = 0.01;
        let y_step: f32 = 0.01;
        let x_samples = (max_x - min_x) as f32 / x_step;
        let y_samples = (max_y - min_y) as f32 / y_step;

        let mut current_x = min_x as f32;
        let mut current_y = min_y as f32;

        let mut points: Vec<(f32, f32)> = Vec::new();

        for _x in 0..x_samples as usize {
            for _y in 0..y_samples as usize {
                current_y += y_step; 
                // push current_x, current_y onto vector
                let evaluation = Self::evaluate([self.conic_coef[0], self.conic_coef[1], self.conic_coef[2]], round(current_x, 2), round(current_y,2));
                let rounded_evaluation = round(evaluation, 1);

                if rounded_evaluation == 1.0 {
                    points.push((round(current_x, 2), round(current_y,2)));
                }

            }
            current_y = min_y as f32;
            current_x += x_step;
        }

        return points;
    }

    fn evaluate(coefs: [f32; 3], x: f32, y: f32) -> f32 {
        // ax^2 + bxy + cy^2
        (coefs[0] * x * x) + (coefs[1] * x * y) + (coefs[2] * y * y)
    }

    pub fn align(&self) -> ConicSection {
        // returns a conic section with major and minor axis aligned with x,y axis
        // implementation is in mat2x2 along with papers
        let charmat = Mat2x2::new([self.conic_coef[0], self.conic_coef[1]/2.0, self.conic_coef[1]/2.0, self.conic_coef[2]]);

        // eigenvalues -> eigenvectors
        let (lambda1, lambda2) = charmat.eigenvalues();
        let (eigenvector_1,eigenvector_2) = charmat.eigenvectors(lambda1, lambda2);

        // P is a rotation matrix and scaled by magnitude of the eigenvectors (1/ sqrt(2))
        let mut p = Mat2x2::new([eigenvector_1[0], eigenvector_2[0], eigenvector_1[1], eigenvector_2[1]]);
        let mag: f32 = (eigenvector_1[0] * eigenvector_1[0] + eigenvector_1[1] * eigenvector_1[1]).sqrt();
        p *= 1.0 / mag;

        // P^T * A * P   
        // https://math.emory.edu/~lchen41/teaching/2020_Fall/Section_8-2.pdf 
        let aligned_coefficients = p.transpose() * charmat * p; 

        let matrix_entries = aligned_coefficients.to_array(); // [[a,b], [b,c]]
        ConicSection{cone: self.cone, plane: self.plane, conic_coef: [matrix_entries[0], matrix_entries[1], matrix_entries[3], 0.0, 0.0, 0.0]}
    }

    fn compute_conic_coefficients(cone: &Cone, plane: &Plane) -> [f32; 6] {
        let mut cfs: [f32; 6] = [0.0; 6];
        cfs[0] = plane.c * plane.c * cone.m * cone.m - plane.a * plane.a;                       // A
        cfs[1] = -2.0 * plane.a * plane.b;                                                      // B
        cfs[2] = plane.c * plane.c * cone.m * cone.m - plane.b * plane.b;                       // C
        cfs[3] = 2.0 * plane.a * plane.d;                                                       // D
        cfs[4] = 2.0 * plane.b * plane.d;                                                       // E
        cfs[5] = -1.0 * plane.d * plane.d;                                                      // F
        cfs
    }
    pub fn is_valid(&self, x: f32, y: f32, precision: u32) -> f32 {
        fn round(x: f32, decimals: u32) -> f32 {
            let y = 10i32.pow(decimals) as f32;
            (x * y).round() / y
        }

        let c: [f32; 6] = self.conic_coef;
        let evaluated: f32 = round((c[0] * x * x) + (c[1] * x * y) + (c[2] * y * y) + (c[3] * x) + (c[4] * y) + c[5], precision);

        evaluated
    }
    fn valid(x: f32, y: f32, c: [f32; 6], precision: u32) -> bool {
        fn round(x: f32, decimals: u32) -> f32 {
            let y = 10i32.pow(decimals) as f32;
            (x * y).round() / y
        }
        let evaluated: f32 = round((c[0] * x * x) + (c[1] * x * y) + (c[2] * y * y) + (c[3] * x) + (c[4] * y) + c[5], precision);
        if evaluated < 400.0 && evaluated > -400.0 {
            return true;
        } else {
            return false;
        }
    }

    fn cart_to_screen(cartesian_x: f32, cartesian_y: f32, sc_width: f32, sc_height: f32) -> (i32, i32) {
        let scx = cartesian_x + sc_width / 2.0;
        let scy = sc_height / 2.0 - cartesian_y;
        (scx as i32, scy as i32)
    }

    pub fn compute_conic(&mut self, xmin: i32, xmax: i32, ymin: i32, ymax:i32) -> Vec<(i32, i32)> {
        // return set of points that represent a valid conic section in  **** SCREEN SPACE ****
        // specify precision with samples
        self.conic_coef = Self::compute_conic_coefficients(&self.cone, &self.plane);
        let xrange: i32 = xmax + xmin.abs();
        let yrange: i32 = ymax + ymin.abs();
        let xstep: usize = 2; //(xrange as f32 / samples as f32) as usize;
        let mut vertices: Vec<(i32, i32)> = Vec::new();

        for c_x in (xmin..xmax).step_by(xstep) {
            for c_y in ymin..ymax {
                if Self::valid(c_x as f32, c_y as f32, self.conic_coef, 0) {
                    let (tx, ty) = Self::cart_to_screen(c_x as f32, c_y as f32, xrange as f32, yrange as f32);
                    vertices.push((tx, ty));
                }
            }
        }
        vertices
    }


}
