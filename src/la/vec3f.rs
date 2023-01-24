use std::ops::Sub;
use std::ops::Add;

// ---- Vec3f ----


#[derive(Copy, Clone)]
pub struct Vec3f{
    pub e : [f32; 3],
}
impl Vec3f {
    pub fn new(dat : &[f32]) -> Vec3f{
        let d : [f32; 3] = [dat[0], dat[1], dat[2]];
        Vec3f{ e : d}
    }
    pub fn dot(&mut self, v: &Vec3f) -> f32{
        return self.e[0]*v.e[0] + self.e[1]*v.e[1] + self.e[2]*v.e[2];
    }
    pub fn cross(&mut self, v1: &Vec3f) -> Vec3f{
        return Vec3f::new(&[ self.e[1]*v1.e[2] - self.e[2]*v1.e[1],
                                self.e[2]*v1.e[0] - self.e[0]*v1.e[2],
                                self.e[0]*v1.e[1] - self.e[1]*v1.e[0] ]);
    }
    pub fn normalize(&mut self){
        let len = Self::length(&self);
        self.e[0] /= len;
        self.e[1] /= len;
        self.e[2] /= len;
    }
    fn length(&self) -> f32{
        let l = self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
        return l.sqrt();
    }
    pub fn x(&self) -> f32{
        return self.e[0];
    }
    pub fn y(&self) -> f32{
        return self.e[1];
    }
    pub fn z(&self) -> f32{
        return self.e[2];
    }
    pub fn xy(&self) -> [f32; 2] {
        return [self.e[0], self.e[1]];
    }
}
// traits (operator overloads)
impl Sub for Vec3f{
    type Output = Vec3f;
    fn sub(self, v1: Vec3f) -> Vec3f{
        Vec3f{e : [self.e[0]-v1.e[0], self.e[1]-v1.e[1], self.e[2]-v1.e[2]]}
    }
}
impl Add for Vec3f{
    type Output = Vec3f;
    fn add(self, v1: Vec3f) -> Vec3f{
        Vec3f{e : [self.e[0]+v1.e[0], self.e[1]+v1.e[1], self.e[2]+v1.e[2]]}
    }
}
