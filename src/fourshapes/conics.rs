
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

pub struct Cone{
    pub m: f32,
}
impl Cone{
    pub fn new(steepness: f32) -> Cone {
        Cone{m: steepness}
    }
}

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
