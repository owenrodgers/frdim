

// to_vertex_buffer()
// projection_4d3d()
pub fn helper_function() {
    println!("This jawn aint doing shit");
}


pub fn projection_4d3d(vecs_4: Vec<[f32; 4]>) -> Vec<[f32; 3]> {
    // since cones, planes and conics keep the 4th dimension under the hood 
    // this is essentially only for the tesseract
    // convert 4vectors vertex data to 3vectors
    let mut vecs_3: Vec<[f32; 3]> = Vec::new();
    let mut projection_matrix; 
    let mut projected;
    let distance = 2.0;

    for vec_4 in vecs_4.iter() {
        projection_matrix = projection_4d_3d(distance, vec_4[3]);
        projected = mul(&vec_4, &projection_matrix);
        vecs_3.push(projected);
    }
    return vecs_3;
}
fn mul(vecs_4: &[f32; 4], pmat: &[[f32; 4]; 3]) -> [f32; 3] {
    let mut data = [0.0; 3];
    data[0] = vecs_4[0]*pmat[0][0] + vecs_4[1]*pmat[0][1] + vecs_4[2]*pmat[0][2] + vecs_4[3]*pmat[0][3];
    data[1] = vecs_4[0]*pmat[1][0] + vecs_4[1]*pmat[1][1] + vecs_4[2]*pmat[1][2] + vecs_4[3]*pmat[1][3];
    data[2] = vecs_4[0]*pmat[2][0] + vecs_4[1]*pmat[2][1] + vecs_4[2]*pmat[2][2] + vecs_4[3]*pmat[2][3];
    return data;
}
fn projection_4d_3d(distance: f32, w_coordinate: f32 ) -> [[f32; 4]; 3] {
    let w: f32 = 1.0 / distance - w_coordinate;
    return [
        [w, 0.0, 0.0, 0.0],
        [0.0, w, 0.0, 0.0],
        [0.0, 0.0, w, 0.0]];
}

