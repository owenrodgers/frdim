extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

use std::time::Duration;

mod threedim;
use threedim::{Mat4x4, Mat3x3, WireCube, Floor, Triangle};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

const FNEAR: f32 = 1.0;
const FFAR: f32 = 1000.0;
const FOV: f32 = 90.0;


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust sdl2 window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut projection_matrix = Mat4x4::new();
    projection_matrix.projection(&SCREEN_HEIGHT, &SCREEN_WIDTH, &FOV, &FFAR, &FNEAR);

    let mut theta: f32 = 0.0;
    let mut theta_increment: f32 = 0.15;

    let mut rmz = Mat3x3::new();
    let mut rmx = Mat3x3::new();

    canvas.set_draw_color(Color::RGB(5, 52, 99));
    canvas.clear();
    canvas.present();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse button down at ({},{})", x, y);
                    match theta_increment{
                        0.15 => theta_increment = 0.0,
                        0.00 => theta_increment = 0.15,
                        _ => theta_increment = 0.15,
                    }
                }
                _ => { }
                
            }

        }

        canvas.set_draw_color(Color::RGB(5, 52, 99));
        canvas.clear();
        
        rmz.rotation_z(&theta);
        rmx.rotation_x(&theta);
        render(&mut canvas, &projection_matrix, &rmz, &rmx).ok();
        theta += theta_increment;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
    }
    Ok(())
}

fn render(c: &mut WindowCanvas, pmat: &Mat4x4, rmz: &Mat3x3, rmx: &Mat3x3) -> Result<(),String>{
    // render the wire cube
    //let meshcube: WireCube = WireCube::new();
    let floor: WireCube = WireCube::new();
    let projection_matrix: &Mat4x4 = pmat;

    for vertice_set in floor.vertices.iter(){
        let mut triangle: Triangle = Triangle::new(vertice_set);
        
        triangle.rotate(&rmz); 
        triangle.rotate(&rmx); 

        triangle.translate_z(&3.0);

        triangle.project(projection_matrix);

        triangle.translate_x(&1.0);
        triangle.translate_y(&1.0);

        triangle.scale_x(&(SCREEN_WIDTH*0.5));
        triangle.scale_y(&(SCREEN_HEIGHT*0.5));
        
        tri(c, triangle.vertices[0].e[0], triangle.vertices[0].e[1], 
               triangle.vertices[1].e[0], triangle.vertices[1].e[1], 
               triangle.vertices[2].e[0], triangle.vertices[2].e[1])

    }
    Ok(())

}

fn tri(c: &mut WindowCanvas, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32){
    //c.set_draw_color(Color::RGB(255, 255, 255));
    c.line(x1 as i16, y1 as i16, x2 as i16, y2 as i16, Color::RGB(255, 255, 255)).ok();
    c.line(x2 as i16, y2 as i16, x3 as i16, y3 as i16, Color::RGB(255, 255, 255)).ok();
    c.line(x3 as i16, y3 as i16, x1 as i16, y1 as i16, Color::RGB(255, 255, 255)).ok();
}


