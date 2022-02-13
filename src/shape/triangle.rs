use crate::shape::run::Run;

pub struct Triangle {
    buttom: i32,
    height: i32,
}

impl Triangle {
    pub fn new(buttom: i32, height:i32) -> Triangle{
        return Triangle{
            buttom,
            height,
        }
    }
}

impl Run for Triangle{
    fn calculate(&self) -> i32{
        return self.buttom * self.height / 2
    }
}