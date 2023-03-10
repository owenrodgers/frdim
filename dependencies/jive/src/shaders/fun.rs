extern crate rola;
use rola::vector::vec3f::Vec3f;

pub fn find_color(position: &Vec3f) -> (u8, u8, u8) {
    // 0 < x,y,z < 10
    let norm_position = position.normalize();
    let nr = (norm_position.e[0] * 255.0) as u8;
    let ng = (norm_position.e[1] * 255.0) as u8;
    let nb = (norm_position.e[2] * 255.0) as u8;
    // for high absolute value of z, 
    
    return (nr, ng, nb);
}

pub fn shade_on_normal(position: &Vec3f) -> (u8, u8, u8) {
    // function of distance from origin
    let sick_color = Vec3f::from(232.0, 70.0, 99.0);
    /*
    let p = Vec3f{e : position.e};
    let light = Vec3f::from(0.0, 0.0, -1.0);

    light.normalize();
    p.normalize();
*/
    //let lum = vec3f_dot(&p, &light);
    let d_o = position.magnitude();
    let max = 0.5;

    let r = (color_map(d_o, 0.0, max, 0.0,sick_color.e[0])) as u8;
    let g = (color_map(d_o, 0.0, max, 0.0,sick_color.e[1])) as u8;
    let b = (color_map(d_o, 0.0, max, 0.0,sick_color.e[2])) as u8;

    /*
    let r = (255.0) as u8;
    let g = (255.0) as u8;
    let b = (255.0) as u8;
    */
    return (r,g,b);

}

pub fn color_map(val: f32, r1_min: f32, r1_max: f32, r2_min: f32, r2_max:f32) -> f32 {
    let t1 = r1_max - r1_min;
    let d = val / t1; // 0.5
    let t2 = r2_max - r2_min; // 2
    let inc = t2 * d;
    return r2_min + inc;
}
