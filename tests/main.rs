#[allow(dead_code)]
#[allow(unused_imports)]
//use frdim::la::vec3f::Vec3f;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        //let v: Vec3f = Vec3f::new(&[0.0, 1.0, 2.0]);
        assert_eq!(add(2,2), 4);
        for i in 0..10 {
            println!("cool number {}", i);
        }
        println!("what goin on");
    }

    #[test]
    fn test_add() {
        assert_eq!(add(1,2), 3);
    }
}


