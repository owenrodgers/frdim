

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

// big 4 surfaces
const SPHERE: u8 = 1;
const ELLIPSOID: u8 = 2;
const HYPERBOLOID: u8 = 3;
const PARABOLOID: u8 = 4;

const CONE: u8 = 5;
const PLANE: u8 = 6;

const HYPERBOLIC_PARABOLOID: u8 = 10;

pub struct Surface {
    flag: u8,
    vmin: usize, vmax: usize, vstep: usize,
    umin: usize, umax: usize, ustep: usize,
}

impl Surface {
    pub fn new(flag: u8) -> Surface {
        match flag {
            SPHERE => {
                Surface{flag: SPHERE, vmin: 0, vmax: 180, vstep: 30, umin: 0, umax: 360, ustep: 4} 
            }
            ELLIPSOID => {
                Surface{flag: ELLIPSOID, vmin: 0, vmax: 180, vstep: 10, umin: 0, umax: 360, ustep: 4}
            }
            HYPERBOLOID => {
                Surface{flag: HYPERBOLOID, vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 360, ustep: 4}
            }
            PARABOLOID => {
                Surface{flag: PARABOLOID, vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 360, ustep: 4}
            }
            PLANE => {
                Surface{flag: PLANE, vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 10, ustep: 1}
            }
            CONE => { 
                Surface{flag: CONE, vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 360, ustep: 1} 
            }
            HYPERBOLIC_PARABOLOID => {
                Surface{flag: HYPERBOLIC_PARABOLOID, vmin: 0, vmax: 150, vstep: 5, umin: 0, umax: 100, ustep: 5} 
            }
            _ => {
                Surface{flag: SPHERE, vmin: 0, vmax: 180, vstep: 30, umin: 0, umax: 360, ustep: 4} }
            }
        }

    pub fn vertices(&self, conic_coefficients: [f32; 6]) -> Vec<Vec3f> {
        let mut vertices = Vec::new();

        for v in (self.vmin..self.vmax).step_by(self.vstep) {
            let v_param = v as f32;
                
            for u in (self.umin..self.umax).step_by(self.ustep) {
                let u_param = u as f32;
                    
                let (x,y,z) = Self::solve(self.flag, v_param, u_param, &conic_coefficients);
                    vertices.push(Vec3f::from([x,y,z]));
            }
        }
        return vertices;

    }
    pub fn solve(surface_type: u8, v_parameter: f32, u_parameter: f32, optional_parameters: &[f32; 6]) -> (f32, f32, f32) {
        let scale: f32 = 1.0 / 20.0;

        match surface_type {
            CONE => {
                // in this case v = t (y coordinate) and u = theta
                // so theta (v) needs to be converted to radians
                // the optional parameters come in as the steepness and height

                let height = optional_parameters[1];
                let steepness = optional_parameters[0];
                let scale = height / (200.0 * steepness);

                let theta = Self::d2rad(u_parameter);
                let x = scale * v_parameter * theta.cos();
                let y = scale * v_parameter * theta.sin();
                let z = scale * optional_parameters[0] * v_parameter;
                (x,y,z)
            }
            PLANE => {
                let k: f32 = 1.0 / 20.0;
                let a = optional_parameters[0];
                let b = optional_parameters[1];
                let c = optional_parameters[2];
                let d = optional_parameters[3];
                
                let x = k * u_parameter as f32;
                let y = k * v_parameter as f32;
                let z = k * (1.0 / c) * (d - a * u_parameter as f32 - b * v_parameter as f32);
                (x,y,z)
            }
            ELLIPSOID => {
                // u = psi and v = theta
                //
                // both angles so need to go to radians
                let theta = Self::d2rad(v_parameter);
                let psi = Self::d2rad(u_parameter);
                let a: f32 = optional_parameters[0] / 2.0;
                let b: f32 = optional_parameters[2] / 2.0;
                let c: f32;

                if a > b { 
                    c = b 
                } else {
                    c = a
                }

                let x = a * scale * psi.cos() * theta.sin();
                let y = b * scale * psi.sin() * theta.sin();
                let z = c * scale * theta.cos();
                (x,y,z)

            }
            HYPERBOLOID => {
                let theta = Self::d2rad(u_parameter);
                let v: f32 = v_parameter as f32 / 10.0;
                //let scale: f32 = 0.1;
                let a: f32 = optional_parameters[0];
                let b: f32 = optional_parameters[2];

                let x = a * scale * v.cosh() * theta.cos();
                let y = b * scale * v.cosh() * theta.sin();
                let z = scale * v.sinh();
                (x,z,y)
            }
            PARABOLOID => {
                let theta = Self::d2rad(u_parameter);
                let v: f32 = v_parameter;
                //let scale: f32 = 0.2;
                let a: f32 = optional_parameters[0];
                let b: f32 = optional_parameters[2];

                let x = a * scale * v * theta.cos();
                let y = b * scale * v * theta.sin();
                let z = scale * v * v;
                (x,y,z)
            }
            SPHERE => {
                // u = psi and v = theta
                // both angles so need to go to radians
                let theta = Self::d2rad(v_parameter);
                let psi = Self::d2rad(u_parameter);
                let a = optional_parameters[0] / 2.0;

                let x = a * scale * psi.cos() * theta.sin();
                let y = a * scale * psi.sin() * theta.sin();
                let z = a * scale * theta.cos();
                (x,y,z)
            }
            HYPERBOLIC_PARABOLOID => {
                //let scale: f32 = 1.0 / 500.0;
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



