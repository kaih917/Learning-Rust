use crate::shape::run::Run;

#[derive(Debug)]
pub struct Square {
    buttom: i32,
    height: i32,
}

impl Square {
    pub fn new(buttom: i32, height: i32) -> Square{
        return Square{
            buttom,
            height,
        }
    }
}

impl Run for Square {
    fn calculate(&self) -> i32{
        return self.buttom * self.height
    }
}