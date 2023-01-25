
pub struct HyperSphere{
    pub slice_radius: f32,
}

impl HyperSphere{
    pub fn new(x: f32) -> HyperSphere{
        HyperSphere{ slice_radius: x}
    }
    pub fn foo(&mut self, w_0: f32){
        println!("slice radius: {}", self.slice_radius);
    }
}



