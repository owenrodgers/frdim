extern crate sdl2;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::sdl2::gfx::primitives::DrawRenderer;

extern crate frdim;
use frdim::la::vec3f::Vec3f;
use frdim::la::matrix::{Mat4x4, Mat3x3};

use std::time::Duration;
use std::f32::consts::PI;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;
const FNEAR: f32 = 1.0;
const FFAR: f32 = 1000.0;
const FOV: f32 = 80.0; 

const X_ROTATION: f32 = 0.0;
const Y_ROTATION: f32 = 0.0;
const Z_ROTATION: f32 = 0.0;

const Z_OFFSET:f32 = 0.0;

/*
ls problem use:
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
*/

/*
    render a 3d cross section of  w-plane intersecting 4d cone
*/
// can we see a hyperconic section and a cone/plane intersection beside it?
// render the conic section
// render the cone
// render the plane

use frdim::fourshapes::hyperconics::{HyperCone, HyperPlane};
use frdim::fourshapes::conics::ConicSection;
use frdim::meshes::surfacemesh::Surface;



fn main() -> Result<(), String>{
    println!("Coordinate system on screen is -0.5 to +0.5");
    println!("x-axis: RED | y-axis: GREEN | z-axis: BLUE");
    //

    render()?;

    Ok(())
}

/*
renderable object struct
    center (x,y,z) offsets basically
    rotations
    surface_data Vec<Vec3f>

*/

pub struct RenderableObject{
    trans_x: f32, trans_y: f32, trans_z:f32,
    rotations: Vec<Mat3x3>,  // vec because order matters for passive/active rotations and to avoid gimball lock
    surface_data: Vec<Vec3f>,
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


pub fn render( ) -> Result<(), String> {
    let (mut canvas, mut event_pump) = render_init();

     // transformations for conic section:
    let mut rot_x_conic_section: Mat3x3 = Mat3x3::new();
    rot_x_conic_section.rotation_x(d2rad(0.0));

    let translations_conic_section: [f32; 3] = [0.25, 0.0, 0.0];
    let mut rotations_conic_section: Vec<Mat3x3> = Vec::new();
    rotations_conic_section.push(rot_x_conic_section);


    // transformations for cone
    // ----- renderable cone -----
    let mut rot_x_cone: Mat3x3 = Mat3x3::new();
    rot_x_cone.rotation_x(d2rad(0.0));

    let translations_cone: [f32; 3] = [0.0, 0.0, 0.0];
    let mut rotations_cone: Vec<Mat3x3> = Vec::new();
    rotations_cone.push(rot_x_cone);

    // ----- transformations for plane -----
    let mut rot_x_plane: Mat3x3 = Mat3x3::new();
    rot_x_plane.rotation_x(d2rad(0.0));

    let translations_plane: [f32; 3] = [-0.25, -0.25, 0.0];
    let mut rotations_plane: Vec<Mat3x3> = Vec::new();
    rotations_plane.push(rot_x_plane);


    // ----- some surfaces -----
    let plane_flag: u8 = 6;
    let plane_surface = Surface::new(plane_flag);

    let cone_flag: u8 = 5;
    let cone_surface = Surface::new(cone_flag);  


    // ----- hypercone intersection math ----- //
    let height: f32 = 4.0;
    let steepness: f32 = 2.0;
    let hypercone: HyperCone = HyperCone::new(height, steepness);

    //ax + by + cz = d
    let a = 1.0; let b = 0.0; let c = 1.0; let d = 3.0;
    let hyperplane: HyperPlane = HyperPlane::new(a, b, c, d);

    // conic_section appears
    let conic_section: ConicSection = hypercone.intersection(&hyperplane);
    println!("{:?}", conic_section.conic_coef);

    // cone, plane and conic section
    let plane_coefs = [a, b, c, d, 0.0, 0.0];
    let cone_coefs = [steepness, height, 0.0, 0.0, 0.0, 0.0];

    
    let chill_plane = RenderableObject::new(translations_plane, rotations_plane, plane_surface.vertices(plane_coefs));
    let chill_cone = RenderableObject::new(translations_cone, rotations_cone, cone_surface.vertices(cone_coefs));
    let hyperconic_section = RenderableObject::new(translations_conic_section, rotations_conic_section, conic_section.surface_data());
    
    // vec that contains the scene elements
    let mut scene_objects: Vec<RenderableObject> = Vec::new();

    scene_objects.push(chill_plane);
    scene_objects.push(chill_cone);
    scene_objects.push(hyperconic_section);

    // finally push everything to the buffer
    let vertex_buffer: Vec<Vec3f> = scene_to_vertices(&scene_objects);
    

    let mut rotation_y = Y_ROTATION;
    let mut rotation_x = X_ROTATION;
    let rot_inc = 5.0;
    
    'main: loop {
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => break 'main, 
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {rotation_x = X_ROTATION; rotation_y = Y_ROTATION},
                Event::KeyDown { keycode: Some(Keycode::Right ), .. } => {rotation_y -= rot_inc},
                Event::KeyDown { keycode: Some(Keycode::Left  ), .. } => {rotation_y += rot_inc;},
                Event::KeyDown { keycode: Some(Keycode::Down  ), .. } => {rotation_x -= rot_inc;},
                Event::KeyDown { keycode: Some(Keycode::Up    ), .. } => {rotation_x += rot_inc;},
                _ => { } } }
        
        let rotations: [f32; 2] = [rotation_x, rotation_y];
        canvas.set_draw_color(Color::RGB(25, 25, 25));
        canvas.clear();

        draw_axis(&mut canvas, &rotations)?;
        render_vertex_buffer(&mut canvas, &vertex_buffer, &rotations)?;
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
    }
    Ok(())
} 

pub fn render_vertex_buffer(c: &mut WindowCanvas, vb: &Vec<Vec3f>, rotations: &[f32; 2]) -> Result<(), String> {
    let mut rot_x: Mat3x3 = Mat3x3::new(); let mut rot_y: Mat3x3 = Mat3x3::new(); let mut rot_z: Mat3x3 = Mat3x3::new();
    rot_x.rotation_x(d2rad(rotations[0])); rot_y.rotation_y(d2rad(rotations[1])); rot_z.rotation_z(d2rad(Z_ROTATION));
    let rotations = [rot_x, rot_y, rot_z];
    let offsets = [1.0, 1.0, Z_OFFSET];


    for vertex in vb.iter() {
        let z = 10.0 * vertex.e[2];
        let render_point = apply_projective_transformations(vertex, rotations, offsets);

        if (render_point.e[0] < SCREEN_WIDTH && render_point.e[0] > 0.0 ) && (render_point.e[1] < SCREEN_WIDTH && render_point.e[1] > 0.0 ) {
            let (r,g,b) = lerpcolor((83.0, 85.0, 224.0), (76.0, 200.0, 237.0), z );
            c.set_draw_color(Color::RGB(r,g,b));
            c.fill_rect(Rect::new(render_point.e[0] as i32, render_point.e[1] as i32, 4, 4))?;
        } 
        
    }
    Ok(())
}

pub fn apply_projective_transformations(vertex: &Vec3f, rotations: [Mat3x3; 3], offsets: [f32; 3]) -> Vec3f {
    let mut projection_matrix: Mat4x4 = Mat4x4::new();
    projection_matrix.projection(&SCREEN_HEIGHT, &SCREEN_WIDTH, &d2rad(FOV), &FFAR, &FNEAR);

    let mut render_point = Vec3f::from(vertex.e);
    render_point.e[2] += offsets[2];

    render_point = render_point * rotations[1]; render_point = render_point * rotations[0];
    render_point *= &projection_matrix; 
    render_point.e[0] += offsets[0];
    render_point.e[1] += offsets[1];
        
    render_point.e[0] *= SCREEN_WIDTH*0.5;
    render_point.e[1] *= SCREEN_HEIGHT*0.5;

    return render_point;
}

pub fn draw_axis(c: &mut WindowCanvas, rotations: &[f32; 2]) -> Result<(), String> {
    let mut projection_matrix: Mat4x4 = Mat4x4::new();
    projection_matrix.projection(&SCREEN_HEIGHT, &SCREEN_WIDTH, &d2rad(FOV), &FFAR, &FNEAR);
    
    let mut rot_y: Mat3x3 = Mat3x3::new();
    rot_y.rotation_y(d2rad(rotations[1]));

    let mut rot_x: Mat3x3 = Mat3x3::new();
    rot_x.rotation_x(d2rad(rotations[0]));
    
    let mut origin: Vec3f = Vec3f::from([0.0,0.0, 0.0 + Z_OFFSET]);
    let mut x_axis: Vec3f = Vec3f::from([0.5, 0.0, 0.0 + Z_OFFSET]);
    let mut y_axis: Vec3f = Vec3f::from([0.0, 0.5, 0.0 + Z_OFFSET]);
    let mut z_axis: Vec3f = Vec3f::from([0.0, 0.0, 0.5 + Z_OFFSET]);

    origin = origin * rot_y; origin = origin * rot_x;

    x_axis = x_axis * rot_y; x_axis = x_axis * rot_x;
    y_axis = y_axis * rot_y; y_axis = y_axis * rot_x;
    z_axis = z_axis * rot_y; z_axis = z_axis * rot_x;

    origin *= &projection_matrix; x_axis *= &projection_matrix; y_axis *= &projection_matrix; z_axis *= &projection_matrix;

    x_axis.e[0] += 1.0; x_axis.e[1] += 1.0; y_axis.e[0] += 1.0; y_axis.e[1] += 1.0; z_axis.e[0] += 1.0; z_axis.e[1] += 1.0;

    origin.e[0] += 1.0; origin.e[1] += 1.0;
        
    x_axis.e[0] *= SCREEN_WIDTH*0.5; x_axis.e[1] *= SCREEN_HEIGHT*0.5;
    y_axis.e[0] *= SCREEN_WIDTH*0.5; y_axis.e[1] *= SCREEN_HEIGHT*0.5;
    z_axis.e[0] *= SCREEN_WIDTH*0.5; z_axis.e[1] *= SCREEN_HEIGHT*0.5;

    origin.e[0] *= SCREEN_WIDTH*0.5;
    origin.e[1] *= SCREEN_HEIGHT*0.5;

    c.line(origin.e[0] as i16, origin.e[1] as i16, x_axis.e[0] as i16, x_axis.e[1] as i16, Color::RGB(255, 0, 0))?;
    c.line(origin.e[0] as i16, origin.e[1] as i16, y_axis.e[0] as i16, y_axis.e[1] as i16, Color::RGB(0, 255, 0))?;
    c.line(origin.e[0] as i16, origin.e[1] as i16, z_axis.e[0] as i16, z_axis.e[1] as i16, Color::RGB(0, 0, 255))?;
    

    Ok(())
}

pub fn render_init() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();//?;
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust sdl2 window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(5, 52, 99));
    canvas.clear();
    canvas.present();

    (canvas, event_pump)
}

pub fn d2rad(degrees: f32) -> f32 {
    return degrees * (PI / 180.0); 
}

pub fn lerpcolor(color_1: (f32, f32, f32), color_2: (f32, f32, f32), z: f32) -> (u8, u8, u8) {
    fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
        (1.0 - t) * v0 + t * v1
    }
    let (r1,g1,b1) = color_1;
    let (r2,g2,b2) = color_2;
    let normz = z;

    let r = lerp(r1, r2, normz);
    let g = lerp(g1, g2, normz);
    let b = lerp(b1, b2, normz);
    (r as u8,g as u8,b as u8)
}


