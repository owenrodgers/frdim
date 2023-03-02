use::std::mem;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

use crate::Vec3f;
use crate::Mat3x3;


pub struct RenderableObject{
    pub trans_x: f32, pub trans_y: f32, pub trans_z:f32,
    pub rotations: Vec<Mat3x3>,  // order matters
    pub surface_data: Vec<Vec3f>,
}

impl RenderableObject {
    pub fn new(tr: [f32; 3], rotations: Vec<Mat3x3>, surface_data: Vec<Vec3f> ) -> RenderableObject {
        RenderableObject{ trans_x: tr[0], trans_y: tr[1], trans_z: tr[2],
                          rotations: rotations, surface_data: surface_data}
    }
    pub fn push_to_vertex_buffer(&self, vertex_buffer: &mut Vec<Vec3f>) {
        // apply transform to each vertex in surface data
        let mut transformed: Vec3f;

        for vertex in self.surface_data.iter() {
            transformed = Vec3f::from([vertex.e[0], vertex.e[1], vertex.e[2]]);

            transformed = Self::apply_rotations(&transformed, &self.rotations);
            transformed = Self::apply_translations(&transformed, self.trans_x, self.trans_y, self.trans_z);
            
            vertex_buffer.push(transformed);
        }
        
    }
    fn apply_rotations(vertex: &Vec3f, rotations: &Vec<Mat3x3> ) -> Vec3f {
        let mut rotated: Vec3f = Vec3f::from([vertex.e[0], vertex.e[1], vertex.e[2]]);
        for rotation in rotations.iter() {
            rotated = rotated * *rotation;
        }
        return rotated;
    }
    fn apply_translations(vertex: &Vec3f, trans_x: f32, trans_y: f32, trans_z:f32) -> Vec3f {
        let x = vertex.e[0] + trans_x;
        let y = vertex.e[1] + trans_y;
        let z = vertex.e[2] + trans_z;
        return Vec3f::from([x,y,z]);
    }
}

pub fn scene_to_vertices(scene_objects: &Vec<RenderableObject>) -> Vec<Vec3f> {
    let mut vertex_buffer: Vec<Vec3f> = Vec::new();
    for r_object in scene_objects.iter() {
        r_object.push_to_vertex_buffer(&mut vertex_buffer);
    }
    return vertex_buffer;
}







// functions for rendering triangles

pub fn tri(c: &mut WindowCanvas, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32){
    c.set_draw_color(Color::RGB(255, 255, 255));
    c.line(x1 as i16, y1 as i16, x2 as i16, y2 as i16, Color::RGB(255, 255, 255)).ok();
    c.line(x2 as i16, y2 as i16, x3 as i16, y3 as i16, Color::RGB(255, 255, 255)).ok();
    c.line(x3 as i16, y3 as i16, x1 as i16, y1 as i16, Color::RGB(255, 255, 255)).ok();
}
pub fn fill_tri(c: &mut WindowCanvas, v1: &mut [f32; 2], v2: &mut [f32; 2], v3: &mut [f32; 2], fc: &[u8;3]){
    // assumes v1.y <= v2.y <= v3.y
    //         a     b     c
    // order vertices based on y
    if v1[1] > v2[1]{ mem::swap(v1, v2); }
    if v2[1] > v3[1]{ mem::swap(v2, v3); }  
    if v1[1] > v2[1]{ mem::swap(v1, v2); }

    if v2[1] == v3[1]{
        fill_bottom_flat(c, &v1, &v2, &v3, fc);
    } else if v1[1] == v2[1] {
        fill_top_flat(c, &v1, &v2, &v3, fc);
    } else {
        let v4: [f32; 2] = [(v1[0] + ((v2[1] - v1[1]) / (v3[1] - v1[1]) * (v3[0] - v1[0]))), v2[1]];
        fill_bottom_flat(c, &v1, &v2, &v4, fc);
        fill_top_flat(c, &v2, &v4, &v3, fc);
    }  
}
pub fn fill_bottom_flat(c: &mut WindowCanvas, v1: &[f32; 2], v2: &[f32; 2], v3: &[f32; 2], fc: &[u8;3]){
    
    let invslope1: f32 = (v2[0] - v1[0]) / (v2[1] - v1[1]);
    let invslope2: f32 = (v3[0] - v1[0]) / (v3[1] - v1[1]);
    let mut curx1 = v1[0];
    let mut curx2 = v1[0];
    
    for scanline_y in v1[1] as i32..v2[1] as i32{
        c.line(curx1 as i16, scanline_y as i16, curx2 as i16, scanline_y as i16, Color::RGB(fc[0], fc[1], fc[2])).ok();
        curx1 += invslope1;
        curx2 += invslope2;
    }
}
pub fn fill_top_flat(c: &mut WindowCanvas, v1: &[f32; 2], v2: &[f32; 2], v3: &[f32; 2], fc: &[u8;3]){
    
    let invslope1: f32 = (v3[0] - v1[0]) / (v3[1] - v1[1]);
    let invslope2: f32 = (v3[0] - v2[0]) / (v3[1] - v2[1]);
    let mut curx1 = v3[0];
    let mut curx2 = v3[0];

    for scanline_y in (v1[1] as i32 .. v3[1] as i32 ).rev(){
        c.line(curx1 as i16, scanline_y as i16, curx2 as i16, scanline_y as i16, Color::RGB(fc[0], fc[1], fc[2])).ok();
        curx1 -= invslope1;
        curx2 -= invslope2; 
    }
    
}