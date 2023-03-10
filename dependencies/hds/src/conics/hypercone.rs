// hypercone

pub struct HyperCone {
    pub steepness: f32,
    pub height: f32,
}
impl HyperCone {
    pub fn new(steep: f32, height: f32) -> HyperCone {
        HyperCone{steepness: steep, height: height}
    }
}
