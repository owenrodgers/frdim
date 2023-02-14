extern crate sdl2;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

extern crate frdim;


use std::time::Duration;

/*
ls problem use:
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
*/


const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

/*
const FNEAR: f32 = 1.0;
const FFAR: f32 = 1000.0;
const FOV: f32 = 90.0; 
*/

/*
    for a conic in form Ax^2 + Bxy + Cy^2 = 1.0
    3x^2 - 2xy + 3y^2 = 1
    align
    find points
    rasterize
    rotate



*/


use frdim::fourshapes::conics::{Cone, Plane, ConicSection};

fn scale_pointvector(points: Vec<(f32, f32)>, screen_width: f32, screen_height: f32) -> Vec<(i32, i32)> {
    // scale everything out to fit in screen
    // cartesian to screen conversion
    let mut processed: Vec<(i32, i32)> = Vec::new();
    
    for p in points.iter() {
        let (mut gx, mut gy) = p;
        gx *= screen_width/2.0;
        gy *= screen_height/2.0;
        processed.push(cart_to_screen(gx, gy, screen_width, screen_height));
    }
    return processed;
}

fn cart_to_screen(cartesian_x: f32, cartesian_y: f32, sc_width: f32, sc_height: f32) -> (i32, i32) {
    let scx = cartesian_x + sc_width / 2.0;
    let scy = sc_height / 2.0 - cartesian_y;
    (scx as i32, scy as i32)
}


fn main() -> Result<(), String>{
    let c = Cone::new(1.0);
    let p = Plane::new(0.0, 0.0, 0.0, 0.0);

    //[10x^2 + 12xy + 10y^2 = 1] -> [16x^2 + 4y^2 = 1]
    let simple_coefficients = [10.0, 12.0, 10.0, 0.0, 0.0, 0.0];
    let simple_conic = ConicSection{cone: c, plane: p, conic_coef: simple_coefficients};
    let aligned_simple = simple_conic.align();
    println!("{:?}", aligned_simple.conic_coef);
    // good

    // points
    // for this case everything is confined in x: [-1, 1], y: [-1, 1]
    let min_x = -1; let max_x = 1;
    let min_y = -1; let max_y = 1;

    let points_aligned = aligned_simple.get_points(min_x, max_x, min_y, max_y);
    let points_aligned = scale_pointvector(points_aligned, SCREEN_WIDTH, SCREEN_HEIGHT);

    let points_original = simple_conic.get_points(min_x, max_x, min_y, max_y);
    let points_original = scale_pointvector(points_original, SCREEN_WIDTH, SCREEN_HEIGHT);
    
    render(points_original, points_aligned)?;



    Ok(())
}

pub fn render(pset_1: Vec<(i32, i32)>, pset_2: Vec<(i32, i32)> ) -> Result<(), String> {
    let (mut canvas, mut event_pump) = render_init();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event { Event::Quit { .. } => break 'main,  _ => { } } }

        
        canvas.set_draw_color(Color::RGB(5, 52, 99));
        canvas.clear();

        render_conic_section(&mut canvas, &pset_1, (255, 0, 0))?;
        render_conic_section(&mut canvas, &pset_2, (0,0,0))?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
    }
    Ok(())
} 

fn render_conic_section(c: &mut WindowCanvas, points: &Vec<(i32, i32)>, color: (u8, u8, u8)) -> Result<(), String> {
    let (r,g,b) = color;
    c.set_draw_color(Color::RGB(r, g, b));
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
