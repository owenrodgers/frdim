
use sdl2::pixels::Color;

use sdl2::render::WindowCanvas;

pub struct ScreenData2d {
    pub width: i32,
    pub height: i32,
}

pub fn render_2vec(canvas: &mut WindowCanvas, points: &Vec<[f32; 2]>, screen_data: &ScreenData2d) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut rp;
    let screen_height = screen_data.height as f32;
    let screen_width = screen_data.width as f32;

    for p in points.iter() {
        rp = [p[0], p[1]];
        let screen_x = map_to_screen(rp[0], -1.0, 1.0, 0.0, screen_width);
        let screen_y = map_to_screen(rp[1], -1.0, 1.0, 0.0, screen_height);
        canvas.draw_point((screen_x as i32,screen_y as i32))?;
    }
    Ok(())
}

pub fn map_to_screen(val: f32, min_1: f32, max_1: f32, min_2: f32, max_2: f32) -> f32 {
    return (val - min_1) * (max_2 - min_2) / (max_1 - min_1) + min_2;
}