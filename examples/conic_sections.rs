
extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use core::f32::consts::PI;

extern crate frdim;
use frdim::fourshapes::conics::{ConicSection, Plane, Cone};

/*
ls problem use:
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
*/
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

pub fn init() -> (WindowCanvas, EventPump) {
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

pub fn main() -> Result<(), String> {
    let (mut canvas, mut event_pump) = init();

    let c_stp = 7.0;
    let plane: [f32; 4] = [-10.0, 3.5, -0.5, 10.0];
    let mut x: f32 = 0.0;

    let c: Cone = Cone::new(c_stp);
    let p: Plane = Plane::new(plane[0], plane[1], plane[2], plane[3]);
    let mut csec: ConicSection = ConicSection::new(c, p);
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

/*
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
*/
