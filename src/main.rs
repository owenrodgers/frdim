extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use std::time::Duration;

use crate::la::vec3f::Vec3f;
use crate::la::mat3x3::Mat3x3;
use crate::la::mat4x4::Mat4x4;
pub mod la;

use crate::meshrender::triangle::Triangle;
use crate::meshrender::mesh::Mesh;
use crate::meshrender::render::{fill_tri};
pub mod meshrender;

use crate::fourshapes::hypersphere::HyperSphere;
pub mod fourshapes;


const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

const FNEAR: f32 = 1.0;
const FFAR: f32 = 1000.0;
const FOV: f32 = 90.0;

/*
System for propogating errors up call stack needs to be in place with Option<> and Result<>
occlusion needs to be fixed, rendering faces that should not be seen
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
*/


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

    let hs_radius: f32 = 4.0;
    let init_w0: f32 = 0.0;
    let mut cw0 = 0.0;
    let mut x: f32 = 0.0;
    let mut hsp = HyperSphere::new(hs_radius, init_w0);
    //hsp.update_mesh(1.0);

    let mut projection_matrix = Mat4x4::new();
    projection_matrix.projection(&SCREEN_HEIGHT, &SCREEN_WIDTH, &FOV, &FFAR, &FNEAR);

    canvas.set_draw_color(Color::RGB(5, 52, 99));
    canvas.clear();
    canvas.present();

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
        
        if hsp.valid_slice() { render_slice(&mut canvas, &hsp, &projection_matrix).ok(); }
        
        hsp.update_mesh(cw0);
        cw0 = x.cos() + 1.0;
        x += 0.05;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
    }
    Ok(())
}

// render a slice of 4d object
fn render_slice(c: &mut WindowCanvas, hypersphere: &HyperSphere, pmat: &Mat4x4) -> Result<(), String> {
    let camera: Vec3f = Vec3f::new(&[0.0;3]);
    let mut light: Vec3f = Vec3f::new(&[0.0, 0.0, -1.0]);

    let mut normal: Vec3f;
    let mut dot: f32;

    let mut render_tri: Triangle;

    for tri in hypersphere.slice_mesh.triangles.iter() {
        render_tri = Triangle{ vertices: tri.vertices };
        render_tri.translate_z(&5.0);

        normal = render_tri.compute_normal();
        normal.normalize();
        dot = normal.dot(&(render_tri.vertices[0] - camera));

        if dot < 0.0 {
            normal = render_tri.compute_normal();
            normal.normalize();
            light.normalize();

            let (r,g,b) = find_color(normal.dot(&light), hypersphere.slice_radius);

            render_tri.project(pmat);
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

fn find_color(product: f32, radius: f32) -> (u8, u8, u8) {
    let cvr = (74.0 * product ) as u8;
    let cvg = (168.0 * product ) as u8;
    let cvb = (82.0 * product ) as u8;
    (cvr, cvg, cvb)
}




// render and obj file
fn render(c: &mut WindowCanvas, mesh: &Mesh, pmat: &Mat4x4, rmx: &Mat3x3, rmy: &Mat3x3, rmz: &Mat3x3) -> Result<(),String>{
    // render the mesh
    let projection_matrix: &Mat4x4 = pmat;

    let camera: Vec3f = Vec3f::new(&[0.0;3]);
    let mut light: Vec3f = Vec3f::new(&[0.0, 0.0, -1.0]);

    let mut normal: Vec3f;
    let mut dot: f32;

    let mut render_tri: Triangle;


    for tri in mesh.triangles.iter(){
        render_tri = Triangle{ vertices: tri.vertices}; // this is bad

        render_tri.rotate(&rmx);
        render_tri.rotate(&rmy);
        render_tri.rotate(&rmz);

        render_tri.translate_z(&3.0);

        // calculate surface normals for rendering
        normal = render_tri.compute_normal();
        normal.normalize();
        dot = normal.dot(&(render_tri.vertices[0] - camera));

        if dot < 0.0 {
            normal = render_tri.compute_normal();
            normal.normalize();
            light.normalize();

            let cval = (255.0 * normal.dot(&light) ) as u8;

            render_tri.project(projection_matrix);
            render_tri.translate_x(&1.0);
            render_tri.translate_y(&1.0);
            render_tri.scale_x(&(SCREEN_WIDTH*0.5));
            render_tri.scale_y(&(SCREEN_HEIGHT*0.5));

            fill_tri(c, &mut render_tri.vertices[0].xy(), 
                        &mut render_tri.vertices[1].xy(), 
                        &mut render_tri.vertices[2].xy(), 
                        &[cval,cval,cval]);
        }

    }
    Ok(())

}
