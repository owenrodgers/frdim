extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use std::time::Duration;

mod la;
use la::{Vec3f, Mat3x3, Mat4x4, Triangle};

mod render;
use render::{fill_tri};

mod mesh;
use mesh::WireCube;

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

    let mut rmx = Mat3x3::new();
    let mut rmy = Mat3x3::new();
    let mut rmz = Mat3x3::new();

    canvas.set_draw_color(Color::RGB(5, 52, 99));
    canvas.clear();
    canvas.present();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse button down at ({},{})", x, y);
                    theta += theta_increment;
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
    let mut light: Vec3f = Vec3f::new(&[0.0, 0.0, -1.0]);

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
            normal = triangle.compute_normal();
            normal.normalize();
            light.normalize();

            let dp = normal.dot(&light);
            let cval = (255.0 * dp) as u8;

            triangle.project(projection_matrix);
            
            triangle.translate_x(&1.0);
            triangle.translate_y(&1.0);

            triangle.scale_x(&(SCREEN_WIDTH*0.5));
            triangle.scale_y(&(SCREEN_HEIGHT*0.5));

            fill_tri(c, &mut triangle.vertices[0].xy(), 
                        &mut triangle.vertices[1].xy(), 
                        &mut triangle.vertices[2].xy(), 
                        &[cval,cval,cval]);
        }

    }
    Ok(())

}



