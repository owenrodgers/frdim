// hyperplane

pub struct HyperPlane {
    pub a: f32, pub b: f32,
    pub c: f32, pub d: f32,
}
impl HyperPlane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> HyperPlane {
        HyperPlane{a: a, b: b, c: c, d: d}
    }
    pub fn from(coefficients: [f32; 4]) -> HyperPlane {
        HyperPlane{a : coefficients[0], b : coefficients[1], 
                   c : coefficients[2], d : coefficients[3]}
    }
}