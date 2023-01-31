
use crate::Mesh;

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
        cfs[0] = plane.c * plane.c * cone.m * cone.m - plane.a * plane.a; // A
        cfs[1] = -2.0 * plane.a * plane.b;                                                      // B
        cfs[2] = plane.c * plane.c * cone.m * cone.m - plane.b * plane.b; // C
        cfs[3] = 2.0 * plane.a * plane.b;                                                       // D
        cfs[4] = 2.0 * plane.b * plane.d;                                                       // E
        cfs[5] = -1.0 * plane.d * plane.d;                                                      // F
        cfs
    }

}
