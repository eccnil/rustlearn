fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
   pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let result = 2+2;
        assert_eq!(result, 4, "todo el mundo sabe que el resultado es 4 y no {}", result);
        assert_ne!(result, 0); 
    }
    #[test]
    #[should_panic(expected= "should fail")] //el expected es opcional y nos permite saber que la execión es la que queremos y no otra
    fn it_fails(){
        panic!("this tests should fail");
    }
    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {width:8, height:7};
        let smaller = Rectangle {width:5, height:1};
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {width:8, height:7};
        let smaller = Rectangle {width:5, height:7};
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_works_v2() -> Result <(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("error malisimo"))
        }
    }
}
