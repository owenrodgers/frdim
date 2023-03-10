extern crate sdl2; 
use sdl2::event::Event; 
use sdl2::pixels::Color; 

extern crate hds;
use hds::conics::hypercone::HyperCone;
use hds::conics::hyperplane::HyperPlane;
use hds::conics::hyperconic::{HyperConic, vertices_from_hyperconic};

extern crate jive; 
use jive::screendata::ScreenData; 
use jive::jives::jivemodel::JiveModel;
use jive::pipeline::{render_jmodel, jive_render_init, draw_axis};

use core::f32::consts::PI;
use std::time::Duration; 
 

/* ----- Dynamic Hyperconic Section ----- */

fn main() -> Result<(), String> { 
    let screen_data = ScreenData{screen_width: 800.0, screen_height: 800.0,  
                                 field_of_view: 120.0, f_near: 1.0, f_far: 1000.0,
                                 display_axis: true }; 

    let hc = HyperCone::new(2.0, 5.0);
    let hp = HyperPlane::new(0.0, 1.5, 1.0, 2.0);

    let mut hyp_c = HyperConic::new(hc, hp);
    let mut hyperconic_model = JiveModel::new(vertices_from_hyperconic(&hyp_c));

    let mut x: f32 = 0.0;
    let (mut canvas, mut event_pump) = jive_render_init(800, 800);
     
    'main: loop { 
        for event in event_pump.poll_iter() { 
            match event {  
                Event::Quit { .. } => break 'main,  
                _ => { } } } 
 
        canvas.set_draw_color(Color::RGB(25, 25, 25)); 
        canvas.clear(); 
         
        hyperconic_model.rotate_x(PI / 96.0);
        hyp_c.update_plane_b(0.75 * (x/2.0).cos() + 0.76); x += 0.1;

        render_jmodel(&mut canvas, &screen_data, &hyperconic_model)?;
        draw_axis(&mut canvas, &screen_data, &hyperconic_model.transform_matrix)?;
        
        hyperconic_model.update(vertices_from_hyperconic(&hyp_c));
        canvas.present(); 
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); 

    } 
     
    Ok(()) 
}
