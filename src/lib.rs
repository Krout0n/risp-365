#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Num(usize),
    Add(Box<AST>, Box<AST>),
}

pub fn eval(ast: AST) -> usize {
    match ast {
        AST::Num(v) => v,
        AST::Add(left, right) => {
            let left_obj = eval(*left);
            let right_obj = eval(*right);
            left_obj + right_obj
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let ast = AST::Num(1);
        assert_eq!(eval(ast), 1);

        // (1+2)
        let simple_add = AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)));
        assert_eq!(eval(simple_add), 3);

        // ((((1+2)+3)+4)+5)
        let complicated_add = AST::Add(
            Box::new(AST::Add(
                Box::new(AST::Add(
                    Box::new(AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)))),
                    Box::new(AST::Num(3)),
                )),
                Box::new(AST::Num(4)),
            )),
            Box::new(AST::Num(5)),
        );

        assert_eq!(eval(complicated_add), 15);
    }
}
