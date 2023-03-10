extern crate sdl2;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;

extern crate rola;
use rola::vector::vec3f::{Vec3f, projection_multiply};

use rola::matrix::mat4x4::Mat4x4;

use crate::shaders::fun::{find_color};
use crate::screendata::ScreenData;

use crate::jives::jivemodel::JiveModel;

pub fn jive_render_init(screen_width: u32, screen_height: u32) -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust sdl2 window", screen_width as u32, screen_height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let canvas = window
        .into_canvas()
        .build()
        .unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    return (canvas, event_pump);
}


pub fn render_jmodel(canvas: &mut WindowCanvas, screen_data: &ScreenData, jmodel: &JiveModel) -> Result<(), String> {
    let vertices = jmodel.raw_vertices();
    
    let mut render_vertex: Vec3f;
    let mut position_vertex: Vec3f;
    let projection_matrix = screen_data.projection_matrix();

    for vertex in vertices.iter() {
        render_vertex = Vec3f{e : *vertex};
        position_vertex = Vec3f{e : *vertex};

        // multiply by projection mat
        // has specific projection_multiply function
        render_vertex *= &jmodel.transform_matrix;
        render_vertex *= &projection_matrix;

        // add 1 to x and y
        //render_vertex.e[0] += 1.0; render_vertex.e[1] += 1.0;
        let offset = Vec3f::from(1.0, 1.0, 0.0);
        render_vertex += offset;

        // scale x by half screen width
        render_vertex.e[0] *= screen_data.screen_width / 2.0;

        // scale y by half screen height
        render_vertex.e[1] *= screen_data.screen_height / 2.0;

        // render the point and find the color
        let (r,g,b) = find_color(&position_vertex);
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.fill_rect(Rect::new(render_vertex.e[0] as i32, render_vertex.e[1] as i32, 4, 4))?;
    }
    Ok(())
}
// standard rendering pipelineUpub fn render_vertex_buffer(canvas: &mut WindowCanvas, screen_data: &ScreenData, vertices: &Vec<[f32; 3]> ) -> Result<(), String> {

// done this wrong
/*
    need all the vertices and all of the indices
*/
pub fn render_vertex_buffer(canvas: &mut WindowCanvas, screen_data: &ScreenData, vertices: &Vec<[f32; 3]>, transform: &[[f32; 4]; 4] ) -> Result<(), String> {
    let mut render_vertex: Vec3f;
    let mut position_vertex: Vec3f;
    let projection_matrix = screen_data.projection_matrix();
    let tmat = Mat4x4{ e : *transform };

    for vertex in vertices.iter() {
        render_vertex = Vec3f{e : *vertex};
        position_vertex = Vec3f{e : *vertex};

        // multiply by projection mat
        // has specific projection_multiply function
        render_vertex *= &tmat;
        render_vertex = projection_multiply(&render_vertex, &projection_matrix);
        
        // add 1 to x and y
        //render_vertex.e[0] += 1.0; render_vertex.e[1] += 1.0;
        //let offset = Vec3f::from(1.0, 1.0, 0.0);
        render_vertex.e[0] += 1.0;
        render_vertex.e[1] += 1.0;

        // scale x by half screen width
        render_vertex.e[0] *= screen_data.screen_width / 2.0;

        // scale y by half screen height
        render_vertex.e[1] *= screen_data.screen_height / 2.0;

        // render the point and find the color
        let (r,g,b) = find_color(&position_vertex);
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.fill_rect(Rect::new(render_vertex.e[0] as i32, render_vertex.e[1] as i32, 2, 2))?;
    }
    Ok(())
}



pub fn draw_axis(c: &mut WindowCanvas, screen_data: &ScreenData, transform: &Mat4x4) -> Result<(), String> {
    let projection_matrix = screen_data.projection_matrix();
    let screen_width = screen_data.screen_width;
    let screen_height = screen_data.screen_height;
    
    let mut origin: Vec3f = Vec3f::from(0.0,0.0, 0.0);
    let mut x_axis: Vec3f = Vec3f::from(0.5, 0.0, 0.0);
    let mut y_axis: Vec3f = Vec3f::from(0.0, 0.5, 0.0);
    let mut z_axis: Vec3f = Vec3f::from(0.0, 0.0, 0.5);
    let mut n_zaxis: Vec3f = Vec3f::from(0.0, 0.0, -0.5);

    origin *= transform; x_axis *= transform; y_axis *= transform; z_axis *= transform; n_zaxis *= transform;
    origin *= &projection_matrix; x_axis *= &projection_matrix; y_axis *= &projection_matrix; z_axis *= &projection_matrix;
    n_zaxis *= &projection_matrix;

    x_axis.e[0] += 1.0; x_axis.e[1] += 1.0; y_axis.e[0] += 1.0; y_axis.e[1] += 1.0; z_axis.e[0] += 1.0; z_axis.e[1] += 1.0;
    n_zaxis.e[0] += 1.0; n_zaxis.e[1] += 1.0;

    origin.e[0] += 1.0; origin.e[1] += 1.0;
        
    x_axis.e[0] *= screen_width*0.5; x_axis.e[1] *= screen_height*0.5;
    y_axis.e[0] *= screen_width*0.5; y_axis.e[1] *= screen_height*0.5;
    z_axis.e[0] *= screen_width*0.5; z_axis.e[1] *= screen_height*0.5;
    n_zaxis.e[0] *= screen_width*0.5; n_zaxis.e[1] *= screen_height*0.5;

    origin.e[0] *= screen_width*0.5;
    origin.e[1] *= screen_height*0.5;

    c.line(origin.e[0] as i16, origin.e[1] as i16, x_axis.e[0] as i16, x_axis.e[1] as i16, Color::RGB(255, 0, 0))?;
    c.line(origin.e[0] as i16, origin.e[1] as i16, y_axis.e[0] as i16, y_axis.e[1] as i16, Color::RGB(0, 255, 0))?;
    c.line(origin.e[0] as i16, origin.e[1] as i16, z_axis.e[0] as i16, z_axis.e[1] as i16, Color::RGB(0, 0, 255))?;
    c.line(origin.e[0] as i16, origin.e[1] as i16, n_zaxis.e[0] as i16, n_zaxis.e[1] as i16, Color::RGB(0, 0, 255))?;
 

    Ok(())
}
