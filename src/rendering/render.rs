use::std::mem;
use std::time::Duration;
use core::f32::consts::PI;

extern crate sdl2;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;


use crate::{ConicSection, Plane, Cone};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

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
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(5, 52, 99));
    canvas.clear();
    canvas.present();

    (canvas, event_pump)
}

pub fn render( csec: &mut ConicSection ) -> Result<(), String> {
    let (mut canvas, mut event_pump) = render_init();

    let mut x: f32 = 0.0;
    let mut points: Vec<(i32, i32)> = csec.compute_conic(-400, 400, -400, 400);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse button down at ({},{})", x, y);
                }
                _ => { }
                
            }

        }

        canvas.set_draw_color(Color::RGB(5, 52, 99));
        canvas.clear();
        

        canvas.set_draw_color(Color::RGB(255, 255, 255));
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
    for p in points.iter() { 
        let (x,y) = p;
        c.fill_rect(Rect::new(*x, *y, 4, 4));
    }
    Ok(())
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

