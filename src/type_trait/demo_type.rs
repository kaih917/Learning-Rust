use std::string::String;
use crate::type_trait::comparable::Comparable;


pub struct DemoType{
    name: String,
    size: i32,
}

impl DemoType {
    pub fn print_name(&self){
        let name = self.name.to_string();
        println!("{}", name);
    }
    
    pub fn new(name: String, size: i32) -> Self{
        return Self{
            name,
            size,
        }
    }
}

impl Comparable<Self> for DemoType {
    fn compare(&self, var: &Self) ->u32{
        if self.size > var.size {
            return 0
        }
        return 1
    }
}
