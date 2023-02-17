

// populate the vertex buffer
// generate a surface mesh of a sphere
// using parametric equations
/*
const SPHERE: u8 = 1;

pub struct SurfaceMesh {
    pub vertex_buffer: Vec<Vec3f>,
}

impl SurfaceMesh {
    pub fn new(level_of_detail: f32, mesh_type: u8 ) -> SurfaceMesh {
        SurfaceMesh{vertex_buffer}
    }
}
*/
use std::f32::consts::PI;
use crate::Vec3f;


const CONE: u8 = 1;
const SPHERE: u8 = 2;
const HYPERBOLIC_PARABOLOID: u8 = 3;

pub struct Surface {
    flag: u8,
    vmin: usize, vmax: usize, vstep: usize,
    umin: usize, umax: usize, ustep: usize,
}

impl Surface {
    pub fn new(flag: u8) -> Surface {
        match flag {
            CONE => { 
                Surface{flag: CONE, vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 360, ustep: 1} }
            SPHERE => {
                Surface{flag: SPHERE, vmin: 0, vmax: 180, vstep: 30, umin: 0, umax: 360, ustep: 4} }
            HYPERBOLIC_PARABOLOID => {
                Surface{flag: HYPERBOLIC_PARABOLOID, vmin: 0, vmax: 150, vstep: 5, umin: 0, umax: 100, ustep: 5} 
            }
            _ => {
                Surface{flag: SPHERE, vmin: 0, vmax: 180, vstep: 30, umin: 0, umax: 360, ustep: 4} }
            }
        }


    
    pub fn fill_vertexbuffer(&self, vertex_buffer: &mut Vec<Vec3f>){
        for v in (self.vmin..self.vmax).step_by(self.vstep) {
            let v_param = v as f32;
            
            for u in (self.umin..self.umax).step_by(self.ustep) {
                let u_param = u as f32;
                
                let (x,y,z) = Self::solve_push(self.flag, v_param, u_param);
                if self.flag == HYPERBOLIC_PARABOLOID {
                    vertex_buffer.push(Vec3f::from([x,y,z]));
                    vertex_buffer.push(Vec3f::from([-1.0 * x,y,z]));
                    vertex_buffer.push(Vec3f::from([-1.0 * x, -1.0 * y,z]));
                    vertex_buffer.push(Vec3f::from([x, -1.0 * y,z]));
                    
                } else {
                    vertex_buffer.push(Vec3f::from([x,y,z]));
                }
                
            }
        }

        /*
        for v in (self.vmin..self.vmax).step_by(self.vstep) {
            let v_param = v as f32;
            
            for u in (self.umin..self.umax).step_by(self.ustep) {
                let u_param = u as f32;
                
                let (x,y,z) = Self::solve_push(self.flag, u_param, v_param);
                vertex_buffer.push(Vec3f::from([x,y,z]));
            }
        }
        */
    }

    pub fn solve_push(surface_type: u8, v_parameter: f32, u_parameter: f32) -> (f32, f32, f32) {
        

        match surface_type {
            CONE => {
                // in this case v = t (y coordinate) and u = theta
                // so theta (v) needs to be converted to radians
                let theta = Self::d2rad(u_parameter);
                let x = (v_parameter / 50.0) * theta.cos();
                let y = (v_parameter / 50.0) * theta.sin();
                let z = v_parameter / 50.0;
                (x,y,z)
            }
            SPHERE => {
                // u = psi and v = theta
                // both angles so need to go to radians
                let theta = Self::d2rad(v_parameter);
                let psi = Self::d2rad(u_parameter);
                let scale: f32 = 1.0/10.0;

                let x = scale * psi.cos() * theta.sin();
                let y = scale * psi.sin() * theta.sin();
                let z = scale * theta.cos();
                (x,y,z)
    
            }
            HYPERBOLIC_PARABOLOID => {
                let scale: f32 = 1.0 / 500.0;
                let x = scale * v_parameter;
                let y = scale * u_parameter;
                let z = 2.0 * (x * x) -  2.0 * (y * y);
                (x,y,z)
            }
            _ => {
                // have a nice cone
                let theta = Self::d2rad(v_parameter);
                let x = u_parameter * theta.cos();
                let y = u_parameter * theta.sin();
                let z = u_parameter;
                (x,y,z)
            }
        }
    }
    fn d2rad(degrees: f32) -> f32 {
        return degrees * (PI / 180.0);
    }
}



