use crate::Triangle;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
/*
---- MESH STRUCT ----
Vec<Triangle>

Mesh only houses sets of 3 points [x,y,z]
heavy lifting to generate is done by auxillary function

Mesh::new() -> Blank mesh

*/

pub struct Mesh{
    pub triangles: Vec<Triangle>,
}
impl Mesh{
    pub fn new() -> Mesh{
        let mut v = Vec::new();
        Mesh{ triangles: v}
    }
    pub fn build_triangles(&mut self, filename: &str) {
        let (vertices, indices) = Self::load_vdata(filename);
        println!("{} lines of vertex data", vertices.len());
        println!("{} lines of index data", indices.len());

        for index_set in indices{
            // convert to zero based index
            //println!("{:?}", index_set);
            let a1 = (index_set[0] - 1) as usize;
            let a2 = (index_set[1] - 1) as usize;
            let a3 = (index_set[2] - 1) as usize;
            //println!("{}, {}, {}", a1, a2, a3);
            let v1 = vertices[a1];
            let v2 = vertices[a2];
            let v3 = vertices[a3];

            //println!("{:?}, {:?}, {:?}", v1, v2, v3);

            let tri: Triangle = Triangle::new(&v1, &v2, &v3);
            self.triangles.push(tri);
        }
    }



    pub fn load_vdata(filename: &str) -> (Vec<[f32; 3]>, Vec<[i32; 3]>){
        // read raw from file
        let mut raw_v: Vec<[f32; 3]> = Vec::new();
        let mut raw_i: Vec<[i32; 3]> = Vec::new();
        if let Ok(lines) = Self::read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    let raw: Vec<&str> = ip.split(" ").collect();

                    if raw[0] == "v" {
                        let vdata: [f32; 3] = [raw[1].parse().unwrap(), 
                                 raw[2].parse().unwrap(), 
                                 raw[3].parse().unwrap()];
                        raw_v.push(vdata);

                    } else if raw[0] == "f"{
                        let idata: [i32; 3] = [raw[1].parse().unwrap(), 
                                 raw[2].parse().unwrap(), 
                                 raw[3].parse().unwrap()];
                        raw_i.push(idata);
                    }
                }
            }
        }
        return (raw_v, raw_i);
        
    }
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename).expect("not able to open");
        Ok(io::BufReader::new(file).lines())
    }

}

