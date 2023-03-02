extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;

extern crate frdim;
use frdim::fourshapes::conics::{ConicSection, Cone, Plane, conic_intersection};

extern crate jive;
use jive::util::bustin::ScreenData;
use jive::jives::jivesurface::JiveSurface;

use jive::util::bustin::jive_render_init;
use jive::util::bustin::degrees_to_radians;
use jive::util::bustin::render_jive_surface;

use std::time::Duration;

fn main() -> Result<(), String> {
    // render a conic section with jive
    println!("jive");
    let screen_data = ScreenData{screen_width: 800.0, screen_height: 800.0, 
                                 field_of_view: 80.0, f_near: 1.0, f_far: 1000.0 };
    
    // generate mathematical objects
    let a = 0.0; let b = 0.0; let c = 1.0; let d = 10.0;
    let hyperplane: Plane = Plane::new(a,b,c,d);

    let cone_steepness = 2.0; let cone_height = 5.0;
    let hypercone: Cone = Cone::new(cone_height, cone_steepness);

    let conic_section: ConicSection = conic_intersection(&hyperplane, &hypercone);
    let mut conic: JiveSurface = JiveSurface::new( conic_section.conic_type(), conic_section.conic_coef );

    
    let (mut canvas, mut event_pump) = jive_render_init(800, 800);
    let theta: f32 = degrees_to_radians(&2.0);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => break 'main, 
                _ => { } } }

        canvas.set_draw_color(Color::RGB(25, 25, 25));
        canvas.clear();

        
        conic.rotate_roll(theta);
        render_jive_surface(&mut canvas, &conic, &screen_data)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
    }
    
    Ok(())
}
