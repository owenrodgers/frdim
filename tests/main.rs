#[allow(dead_code)]
#[allow(unused_imports)]


//cargo build && cargo test -- --nocapture
use frdim::fourshapes::hypersphere::HsSlice;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hs_new() {
        let mut hs: HsSlice = HsSlice::new(4.0);
        println!("Triangles: {:?}", hs.mesh.triangles.len());


        println!("old radius: {}", hs.radius);
        let radius_new: f32 = 2.0;
        hs.mesh_update(radius_new);
        println!("new radius: {}", hs.radius);


    }
}

/*
162 lines of vertex data
320 lines of index data
Triangles: 320
*/


