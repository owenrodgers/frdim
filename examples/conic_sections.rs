// standard library
use std::time::Duration;
use core::f32::consts::PI;

extern crate sdl2;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

extern crate frdim;
use frdim::fourshapes::conics::{Cone, Plane, ConicSection};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

/*
ls problem use:
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
*/

fn main() -> Result<(), String>{
    let cone_steepness: f32 = 7.0;
    let plane_coefs: [f32; 4] = [-10.0, 3.5, -0.5, 10.0];

    let plane: Plane = Plane::new(plane_coefs[0], plane_coefs[1], plane_coefs[2], plane_coefs[3]);
    let cone: Cone = Cone::new(cone_steepness);

    let mut conic: ConicSection = ConicSection::new(cone, plane);
    render(&mut conic)?;

    Ok(())
}

pub fn render( csec: &mut ConicSection ) -> Result<(), String> {
    let (mut canvas, mut event_pump) = render_init();

    let mut x: f32 = 0.0;
    let mut points: Vec<(i32, i32)> = csec.compute_conic(-400, 400, -400, 400);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event { Event::Quit { .. } => break 'main,  _ => { } } }

        
        canvas.set_draw_color(Color::RGB(5, 52, 99));
        canvas.clear();

        render_conic_section(&mut canvas, &points)?;

        csec.plane.a = 2.0 * x.cos();
        csec.plane.d = 15.0 * (x / 2.0 * PI).sin() + 10.0;
        points = csec.compute_conic(-400, 400, -400, 400);

        x += 0.05;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
    }
    Ok(())
} 

fn render_conic_section(c: &mut WindowCanvas, points: &Vec<(i32, i32)>) -> Result<(), String> {
    c.set_draw_color(Color::RGB(255, 255, 255));
    for p in points.iter() { 
        let (x,y) = p;
        c.fill_rect(Rect::new(*x, *y, 4, 4))?;
    }
    Ok(())
}

pub fn render_init() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();//?;
    let video_subsystem = sdl_context.video().unwrap();//?;
    let window = video_subsystem
        .window("rust sdl2 window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();//?;
    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();//?;
    let event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(5, 52, 99));
    canvas.clear();
    canvas.present();

    (canvas, event_pump)
}