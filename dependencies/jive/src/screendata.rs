use rola::matrix::mat4x4::Mat4x4;
use core::f32::consts::PI;

pub struct ScreenData{
    //contains useful info
    pub screen_width: f32,
    pub screen_height: f32,
    pub field_of_view: f32,
    pub f_near: f32,
    pub f_far: f32,
    pub display_axis: bool,
}

impl ScreenData{
    pub fn projection_matrix(&self) -> Mat4x4 {
        let mut mat: Mat4x4 = Mat4x4::new();
        let theta = degrees_to_radians(self.field_of_view);
        
        let a: f32 = self.screen_height / self.screen_width;
        let f: f32 = 1.0 / (theta*0.5).tan();
        let q: f32 = self.f_far / (self.f_far - self.f_near);
    
        mat.e[0][0] = a * f;
        mat.e[1][1] = f;
        mat.e[2][2] = q;
        mat.e[2][3] = 1.0;
        mat.e[3][2] = (-1.0 * self.f_far * self.f_near) / (self.f_far - self.f_near);
        return mat;
    }
}

fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
    //return degrees;
}