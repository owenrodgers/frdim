#[allow(dead_code)]
#[allow(unused_imports)]


//cargo build && cargo test -- --nocapture
use frdim::fourshapes::hypersphere::HyperSphere;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hs_new() {
        let r: f32 = 6.0;
        let mut hs: HyperSphere = HyperSphere::new(r);
        hs.foo(6.0);
        
    }
}


