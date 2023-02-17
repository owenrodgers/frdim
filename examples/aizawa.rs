extern crate sdl2;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::sdl2::gfx::primitives::DrawRenderer;

extern crate frdim;
use frdim::la::vec3f::Vec3f;
use frdim::la::matrix::{Mat4x4, Mat3x3};

use std::time::Duration;

extern crate rand;
use rand::Rng;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;
const FNEAR: f32 = 1.0;
const FFAR: f32 = 1000.0;
const FOV: f32 = 90.0; 
const BACKSPACE: char = 8u8 as char;


// the aizawa attractor
// https://analogparadigm.com/downloads/alpaca_17.pdf


fn main() -> Result<(), String>{
    println!("The Aizawa Attractor");
    render()?;
    Ok(())
}

pub fn render() -> Result<(), String> {
    let (mut canvas, mut event_pump) = render_init();
    let mut rng = rand::thread_rng();

    let mut points: Vec<Vec3f> = Vec::new();
    let coefficients = [0.095, 0.7, 0.65, 3.5, 0.1];

    let n = 500;
    
    for _x in 0..n {
        let cx = rng.gen_range(-1.0..1.0) / 1000.0;
        let cy = rng.gen_range(-1.0..1.0) / 1000.0;
        let cz = rng.gen_range(-1.0..1.0) / 1000.0;
        points.push(Vec3f::from([cx, cy, cz]));
    }
    

    let mut projection_matrix: Mat4x4 = Mat4x4::new();
    projection_matrix.projection(&SCREEN_HEIGHT, &SCREEN_WIDTH, &FOV, &FFAR, &FNEAR);


    'main: loop {
        for event in event_pump.poll_iter() {
            match event { Event::Quit { .. } => break 'main,  _ => { } } }
        
        canvas.set_draw_color(Color::RGB(10, 10, 10));
        canvas.clear();

            render_points(&mut canvas, &points, &projection_matrix)?;
            update_points(&mut points, coefficients);
        

        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        
    }
    Ok(())
} 

fn update_points(points: &mut Vec<Vec3f>, coefficients: [f32; 5]) -> &mut Vec<Vec3f> {
    let alpha = coefficients[0];
    let beta = coefficients[1];
    let gamma = coefficients[2];
    let delta = coefficients[3];
    let epsilon = coefficients[4];

    let mut x: f32; let mut y: f32; let mut z: f32;
    let mut z_minus_beta: f32;

    let mut dx: f32; let mut dy: f32; let mut dz: f32;
    
    for point in points.iter_mut(){
        x = (1.0 / 3.0) * point.e[0];
        y = (1.0 / 4.0) * point.e[1];
        z = (1.0 / 2.0) * point.e[2];

        z_minus_beta = (2.0 / 5.0) * (z - beta);

        dx = x * z_minus_beta - delta * y;
        dy = delta * x + y * z_minus_beta;
        dz = gamma + alpha * z - (z.powf(3.0) / 3.0) + epsilon * z * x.powf(3.0);
        point.e[0] = x + dx;
        point.e[1] = y + dy;
        point.e[2] = z + dz;
        
    }

    points
}


fn render_points(c: &mut WindowCanvas, points: &Vec<Vec3f>, projection_matrix: &Mat4x4) -> Result<(), String> {
    
    let mut render_point: Vec3f;
    let mut rot_y: Mat3x3 = Mat3x3::new();
    rot_y.rotation_y(15.0);

    let mut rot_x: Mat3x3 = Mat3x3::new();
    rot_x.rotation_x(15.0);


    let z_offset: f32 = 10.0;
    //let center: Vec3f = Vec3f::from([0.0, 0.0, z_offset]);

    for point in points.iter() {
        
        render_point = Vec3f::from(point.e);
        render_point.e[2] += z_offset;

        //let (r,g,b) = find_color(1.0);
        
        render_point = render_point * rot_y; render_point = render_point * rot_x;
        render_point *= projection_matrix; // operator overload mulassign for vec3f mat4x4
        render_point.e[0] += 1.0;
        render_point.e[1] += 1.0;
            

        render_point.e[0] *= SCREEN_WIDTH*0.5;
        render_point.e[1] *= SCREEN_HEIGHT*0.5;

        if (render_point.e[0] < SCREEN_WIDTH && render_point.e[0] > 0.0 ) && (render_point.e[1] < SCREEN_WIDTH && render_point.e[1] > 0.0 ) {
            c.set_draw_color(Color::RGB(255, 255, 255));
            c.fill_rect(Rect::new(render_point.e[0] as i32, render_point.e[1] as i32, 4, 4))?;
            print!("{}\r{:?}", BACKSPACE, render_point.e);
        }
    }
    // x axis

    let mut origin: Vec3f = Vec3f::from([0.0,0.0,0.0]);
    let mut x_axis: Vec3f = Vec3f::from([2.0, 0.0, 0.0]);
    let mut y_axis: Vec3f = Vec3f::from([0.0, 2.0, 0.0]);
    let mut z_axis: Vec3f = Vec3f::from([0.0, 0.0, 2.0]);

    origin = origin * rot_y; origin = origin * rot_x;

    x_axis = x_axis * rot_y; x_axis = x_axis * rot_x;
    y_axis = y_axis * rot_y; y_axis = y_axis * rot_x;
    z_axis = z_axis * rot_y; z_axis = z_axis * rot_x;

    origin *= projection_matrix;
    x_axis *= projection_matrix;
    y_axis *= projection_matrix;
    z_axis *= projection_matrix;

    x_axis.e[0] += 1.0; x_axis.e[1] += 1.0;
    y_axis.e[0] += 1.0; y_axis.e[1] += 1.0;
    z_axis.e[0] += 1.0; z_axis.e[1] += 1.0;

    origin.e[0] += 1.0;
    origin.e[1] += 1.0;
        
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




