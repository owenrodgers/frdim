extern crate sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

extern crate frdim;
use frdim::la::vec3f::Vec3f;
use frdim::la::matrix::Mat4x4;
use frdim::meshes::triangle::Triangle;
use frdim::rendering::render::fill_tri;
use frdim::fourshapes::hypersphere::HyperSphere;

use std::time::Duration;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

const FNEAR: f32 = 1.0;
const FFAR: f32 = 1000.0;
const FOV: f32 = 90.0; 


fn main() -> Result<(), String>{
    let hs_radius: f32 = 4.0;
    let init_w0: f32 = 0.0;
    let mut hsp = HyperSphere::new(hs_radius, init_w0);

    render_hypersphere(&mut hsp)?;

    Ok(())
}

pub fn render_hypersphere(hsp: &mut HyperSphere) -> Result<(), String> {
    let (mut canvas, mut event_pump) = render_init();
    let mut projection_matrix: Mat4x4 = Mat4x4::new();
    projection_matrix.projection(&SCREEN_HEIGHT, &SCREEN_WIDTH, &FOV, &FFAR, &FNEAR);

    let mut cw0 = 0.0;
    let mut x: f32 = 0.0;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event { Event::Quit { .. } => break 'main,  _ => { } } }

        
        canvas.set_draw_color(Color::RGB(5, 52, 99));
        canvas.clear();

        if hsp.valid_slice() { render_slice_tris(&mut canvas, &projection_matrix, &hsp.slice_mesh.triangles)? }
        hsp.update_mesh(cw0);
        cw0 = x.cos() + 1.0;
        x += 0.05;
        

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
    }
    Ok(())
}

fn find_color(luminence: f32) -> (u8, u8, u8) {
    ((255.0 * luminence) as u8,
     (255.0 * luminence) as u8,
     (255.0 * luminence) as u8)
}

pub fn render_init() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
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

pub fn render_slice_tris(c: &mut WindowCanvas, projection_matrix: &Mat4x4, tris: &Vec<Triangle>) -> Result<(), String> {
    let camera: Vec3f = Vec3f::new(&[0.0;3]);
    let mut light: Vec3f = Vec3f::new(&[0.0, 0.0, -1.0]);

    let mut normal: Vec3f;
    let mut dot: f32;

    let mut render_tri: Triangle;

    for tri in tris.iter() {
        render_tri = Triangle{ vertices: tri.vertices };
        render_tri.translate_z(&5.0);

        normal = render_tri.compute_normal();
        normal.normalize();
        dot = normal.dot(&(render_tri.vertices[0] - camera));

        if dot < 0.0 {
            normal = render_tri.compute_normal();
            normal.normalize();
            light.normalize();

            let (r,g,b) = find_color(normal.dot(&light));

            render_tri.project(projection_matrix);
            render_tri.translate_x(&1.0);
            render_tri.translate_y(&1.0);
            render_tri.scale_x(&(SCREEN_WIDTH*0.5));
            render_tri.scale_y(&(SCREEN_HEIGHT*0.5));

            fill_tri(c, &mut render_tri.vertices[0].xy(), 
                        &mut render_tri.vertices[1].xy(), 
                        &mut render_tri.vertices[2].xy(), 
                        &[r,g,b]);
        }

    }
    Ok(())

}
