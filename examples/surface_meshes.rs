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
use frdim::meshes::surfacemesh::Surface;

use std::time::Duration;
use std::f32::consts::PI;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;
const FNEAR: f32 = 1.0;
const FFAR: f32 = 1000.0;
const FOV: f32 = 80.0; 

const X_ROTATION: f32 = 15.0;
const Y_ROTATION: f32 = 15.0;
const Z_ROTATION: f32 = 0.0;

const Z_OFFSET:f32 = 0.0;

fn main() -> Result<(), String>{
    println!("Surface meshes");
    println!("x-axis: RED | y-axis: GREEN | z-axis: BLUE");
    render()?;
    Ok(())
}





/*
pub fn populate_vb(vertex_buffer: &mut Vec<Vec3f>, surface: Surface) {
    /*
    // https://www.geogebra.org/m/BjV7cNwb
    for v in (0..10).step_by(1) {
        let t_param = v as f32 / 50.0;
        for u in 0..360 {
            let theta: f32 = u as f32;
            let (x,y,z) = solve_surface(surface_type, t_param, theta);


            vertex_buffer.push(Vec3f::from([x,y,z]));
        }
    }
    */

    

    for v in (0..180).step_by(30) {
        for u in (0..360).step_by(4) {
            let psi = d2rad(u as f32); //   u
            let theta = d2rad(v as f32); // v
            let scale: f32 = 1.0 / 10.0;

            let x = scale * psi.cos() * theta.sin();
            let y = scale * psi.sin() * theta.sin();
            let z = scale * theta.cos();
            vertex_buffer.push(Vec3f::from([x,y,z]));

        }
    }

    for v in (0..180).step_by(30) {
        for u in (0..360).step_by(4) {
            let psi = d2rad(u as f32); //   u
            let theta = d2rad(v as f32); // v
            let scale: f32 = 1.0 / 10.0;

            let x = scale * psi.cos() * theta.sin();
            let y = scale * psi.sin() * theta.sin();
            let z = scale * theta.cos();
            vertex_buffer.push(Vec3f::from([x,z,y]));

        }
    }
}
*/

pub fn render() -> Result<(), String> {
    let (mut canvas, mut event_pump) = render_init();
    let cool_surface: Surface = Surface::new(3);

    let mut vertex_buffer: Vec<Vec3f> = Vec::new();
    cool_surface.fill_vertexbuffer(&mut vertex_buffer);

    //populate_vb(&mut vertex_buffer, cone);

    let mut rotation_y = Y_ROTATION;
    let mut rotation_x = X_ROTATION;
    let rot_inc = 5.0;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => break 'main, 
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {rotation_x = X_ROTATION; rotation_y = Y_ROTATION},
                Event::KeyDown { keycode: Some(Keycode::Right ), .. } => rotation_y -= rot_inc,
                Event::KeyDown { keycode: Some(Keycode::Left  ), .. } => rotation_y += rot_inc,
                Event::KeyDown { keycode: Some(Keycode::Down  ), .. } => rotation_x -= rot_inc,
                Event::KeyDown { keycode: Some(Keycode::Up    ), .. } => rotation_x += rot_inc,
                _ => { } } }
        
        let rotations: [f32; 2] = [rotation_x, rotation_y];
        canvas.set_draw_color(Color::RGB(25, 25, 25));
        canvas.clear();

        draw_axis(&mut canvas, &rotations)?;
        render_vertex_buffer(&mut canvas, &vertex_buffer, &rotations)?;
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
        
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
        //println!("{}", z);
        let render_point = apply_projective_transformations(vertex, rotations, offsets);

        if (render_point.e[0] < SCREEN_WIDTH && render_point.e[0] > 0.0 ) && (render_point.e[1] < SCREEN_WIDTH && render_point.e[1] > 0.0 ) {
            let (r,g,b) = lerpcolor((222.0, 31.0, 56.0), (76.0, 200.0, 237.0), z );
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
