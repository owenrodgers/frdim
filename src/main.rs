extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

use std::time::Duration;

mod threedim;
use threedim::{Mat4x4, Mat3x3, Vec3f, WireCube, Triangle};

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
    let theta_increment: f32 = 0.05;

    let mut rmz = Mat3x3::new();
    let mut rmx = Mat3x3::new();
    let mut rmy = Mat3x3::new();

    canvas.set_draw_color(Color::RGB(5, 52, 99));
    canvas.clear();
    canvas.present();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse button down at ({},{})", x, y);
                    theta = 0.0;
                }
                _ => { }
                
            }

        }

        canvas.set_draw_color(Color::RGB(5, 52, 99));
        canvas.clear();
        
        rmx.rotation_x(&theta);
        rmy.rotation_y(&theta);
        rmz.rotation_z(&theta);
        render(&mut canvas, &projection_matrix, &rmx, &rmy, &rmz).ok();
        theta += theta_increment;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
    }
    Ok(())
}

fn render(c: &mut WindowCanvas, pmat: &Mat4x4, rmx: &Mat3x3, rmy: &Mat3x3, rmz: &Mat3x3) -> Result<(),String>{
    // render the wire cube
    let wc: WireCube = WireCube::new();
    let projection_matrix: &Mat4x4 = pmat;

    let camera: Vec3f = Vec3f::new(&[0.0;3]);
    let mut normal: Vec3f;
    let mut dot: f32;

    for vertice_set in wc.vertices.iter(){
        let mut triangle: Triangle = Triangle::new(vertice_set);

        triangle.rotate(&rmx);
        triangle.rotate(&rmy);
        triangle.rotate(&rmz); 

        triangle.translate_z(&3.0);

        // calculate surface normals for rendering
        normal = triangle.compute_normal();
        normal.normalize();
        dot = normal.dot(&(triangle.vertices[0] - camera));

        if dot < 0.0 {
            triangle.project(projection_matrix);

            triangle.translate_x(&1.0);
            triangle.translate_y(&1.0);

            triangle.scale_x(&(SCREEN_WIDTH*0.5));
            triangle.scale_y(&(SCREEN_HEIGHT*0.5));
            tri(c, triangle.vertices[0].e[0], triangle.vertices[0].e[1], 
                triangle.vertices[1].e[0], triangle.vertices[1].e[1], 
                triangle.vertices[2].e[0], triangle.vertices[2].e[1])
        }

    }
    Ok(())

}

fn tri(c: &mut WindowCanvas, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32){
    //c.set_draw_color(Color::RGB(255, 255, 255));
    c.line(x1 as i16, y1 as i16, x2 as i16, y2 as i16, Color::RGB(255, 255, 255)).ok();
    c.line(x2 as i16, y2 as i16, x3 as i16, y3 as i16, Color::RGB(255, 255, 255)).ok();
    c.line(x3 as i16, y3 as i16, x1 as i16, y1 as i16, Color::RGB(255, 255, 255)).ok();
}


