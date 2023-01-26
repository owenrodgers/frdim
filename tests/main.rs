#[allow(dead_code)]
#[allow(unused_imports)]


//cargo build && cargo test -- --nocapture
use frdim::fourshapes::hypersphere::HyperSphere;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sr1() {
        // hypersphere_radius: f32, w_naught
        let hsr: f32 = 5.0; // = hypersphere_radius
        let w0: f32 = 1.0; // = w_naught
        let mut hs: HyperSphere = HyperSphere::new(hsr, w0);
        println!("radius: {}, slice radius: {}, w0: {}", hs.hypersphere_radius, hs.slice_radius, hs.w_naught);

    }

}