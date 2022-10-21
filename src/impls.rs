use crate::{Object, AST};

impl std::ops::Add for Object {
    type Output = Object;
    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left + right),
            _ => panic!(
                "left and right are expected to be Num, but got left: {:?}, right: {:?}",
                self, rhs
            ),
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Object;
    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left - right),
            _ => panic!(
                "left and right are expected to be Num, but got left: {:?}, right: {:?}",
                self, rhs
            ),
        }
    }
}

impl From<usize> for AST {
    fn from(v: usize) -> Self {
        AST::Num(v)
    }
}

impl From<bool> for AST {
    fn from(v: bool) -> Self {
        AST::Bool(v)
    }
}
