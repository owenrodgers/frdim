extern crate sdl2;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

extern crate frdim;
use frdim::la::vec3f::Vec3f;
use frdim::la::matrix::Mat4x4;


use std::time::Duration;

/*
ls problem use:
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
*/

// BOOM

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;


const FNEAR: f32 = 1.0;
const FFAR: f32 = 1000.0;
const FOV: f32 = 90.0; 


// ConicSection refactor
// implement color map
// make some 3d shapes

use frdim::fourshapes::conics::{Cone, Plane, ConicSection};
use frdim::fourshapes::hyperconics::HyperConic;
// god this is a mess
// vec3f rewrite
//


fn main() -> Result<(), String>{
    let c = Cone::new(1.0);
    let p = Plane::new(0.0, 0.0, 0.0, 0.0);

    //[10x^2 + 12xy + 10y^2 = 1] -> [16x^2 + 4y^2 = 1]
    let simple_coefficients = [1.0, -4.0, 1.0, 0.0, 0.0, 0.0];
    let simple_conic = ConicSection{cone: c, plane: p, conic_coef: simple_coefficients};
    let aligned_simple = simple_conic.align();

    println!("Original: {:?}", simple_coefficients);
    println!("Aligned:  {:?}", aligned_simple.conic_coef);

    // 3d rendering functions, here comes the projection matrix
    // color map
    // vec3f

    // pointcloud for hyperconic
    let mut hyperconic: HyperConic = HyperConic::new(aligned_simple);
    //println!("{}", pointcloud.len());
    render(&mut hyperconic)?;
    Ok(())
}




pub fn render(hyperconic: &mut HyperConic) -> Result<(), String> {
    let (mut canvas, mut event_pump) = render_init();
    let mut pset_1 = hyperconic.revolve(15.0);

    let mut projection_matrix: Mat4x4 = Mat4x4::new();
    projection_matrix.projection(&SCREEN_HEIGHT, &SCREEN_WIDTH, &FOV, &FFAR, &FNEAR);

    let mut x: f32 = 0.0;
    let x_increment = 0.1;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event { Event::Quit { .. } => break 'main,  _ => { } } }

        
        canvas.set_draw_color(Color::RGB(5, 52, 99));
        canvas.clear();

        render_hyperconic(&mut canvas, &pset_1, &projection_matrix)?;
        // shift ConicSection coefficients
        hyperconic.conic.conic_coef = [3.0, 0.0, 10.0 * x.cos(), 0.0, 0.0, 0.0];
        x += x_increment;
        pset_1 = hyperconic.revolve(15.0);
        // revolve again
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
    }
    Ok(())
} 


fn render_hyperconic(c: &mut WindowCanvas, points: &Vec<Vec3f>, projection_matrix: &Mat4x4) -> Result<(), String> {
    let mut light: Vec3f = Vec3f::new(&[0.0, 0.0, -1.0]);
    let mut dot: f32;
    let mut render_point: Vec3f;
    let mut render_point_normal: Vec3f;

    let z_offset: f32 = 2.0;
    let center: Vec3f = Vec3f::from([0.0, 0.0, z_offset]);

    for point in points.iter() {
        render_point = Vec3f::from(point.e);
        render_point.e[2] += z_offset;

        render_point_normal = render_point - center;
        render_point_normal.normalize();

        dot = render_point_normal.dot(&light);

        if dot > 0.0 {
            render_point_normal = render_point - center;
            render_point_normal.normalize();
            light.normalize();

            let (r,g,b) = find_color(render_point_normal.dot(&light));

            render_point *= projection_matrix; // operator overload mulassign for vec3f mat4x4
            render_point.e[0] += 1.0;
            render_point.e[1] += 1.0;
            
            render_point.e[0] *= SCREEN_WIDTH*0.5;
            render_point.e[1]*= SCREEN_HEIGHT*0.5;

            c.set_draw_color(Color::RGB(r, g, b));
            c.fill_rect(Rect::new(render_point.e[0] as i32, render_point.e[1] as i32, 4, 4))?;
        }

        
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

fn find_color(luminence: f32) -> (u8, u8, u8) {
    ((255.0 * luminence) as u8,
     (255.0 * luminence) as u8,
     (255.0 * luminence) as u8)
}
