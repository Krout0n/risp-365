#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Num(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(usize),
}

pub fn eval(ast: AST) -> Object {
    match ast {
        AST::Num(v) => Object::Num(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let ast = AST::Num(1);
        assert_eq!(eval(ast), Object::Num(1));
    }
}
