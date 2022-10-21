#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Num(usize),
    Add(Box<AST>, Box<AST>),
    Minus(Box<AST>, Box<AST>),
    Bool(bool),
    If {
        cond: Box<AST>,
        then: Box<AST>,
        els: Box<AST>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(usize),
    Bool(bool),
}

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

pub fn eval(ast: AST) -> Object {
    match ast {
        AST::Num(v) => Object::Num(v),
        AST::Add(left, right) => {
            let left_obj = eval(*left);
            let right_obj = eval(*right);
            left_obj + right_obj
        }
        AST::Minus(left, right) => {
            let left_obj = eval(*left);
            let right_obj = eval(*right);
            left_obj - right_obj
        }
        AST::Bool(b) => Object::Bool(b),
        AST::If { cond, then, els } => match eval(*cond) {
            Object::Bool(true) => eval(*then),
            Object::Bool(false) => eval(*els),
            Object::Num(v) if v != 0 => eval(*then),
            Object::Num(_) => eval(*els),
            _ => unimplemented!(),
        },
    }
}

// 関数呼び出しは型や引数が一致していないと呼び出せないが
// マクロは型も引数の個数も一致してなくても呼び出せる
#[macro_export]
macro_rules! ast {
    // tt には `(+ 1 2)` とか `1` などがマッチする
    ((+ $left:tt $right:tt)) => {
        // このマクロの中でASTやpubにしてるやつを使いたいときは
        // `$crate::`っておまじないをつけてください:pray:
        $crate::AST::Add(Box::new(ast!($left)), Box::new(ast!($right)))
    };
    ((- $left:tt $right:tt)) => {
        $crate::AST::Minus(Box::new(ast!($left)), Box::new(ast!($right)))
    };
    ((If $cond:tt $then:tt $els:tt)) => {
        $crate::AST::If {
            cond: Box::new(ast!($cond)),
            then: Box::new(ast!($then)),
            els: Box::new(ast!($els)),
        }
    };
    // 1 や true がマッチする
    ($exp:expr) => {
        $crate::AST::from($exp)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let ast = AST::Num(1);
        assert_eq!(eval(ast), Object::Num(1));

        // (1 + 2)
        // (+ 1 2)
        let simple_add = AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)));
        assert_eq!(eval(simple_add), Object::Num(3));

        // ((((1 + 2) + 3) + 4) + 5)
        // (+ (+ (+ (+ 1 2) 3) 4 ) 5)
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

        assert_eq!(eval(complicated_add), Object::Num(15));

        assert_eq!(
            eval(
                // ((1 + 2) - 2)
                // (- (+ 1 2) 2)
                ast!((- (+ 1 2) 2)),
            ),
            Object::Num(1)
        );

        assert_eq!(eval(ast!(true)), Object::Bool(true));
        assert_eq!(eval(ast!(false)), Object::Bool(false));

        assert_eq!(eval(ast!((If true 1 2))), Object::Num(1));
        assert_eq!(eval(ast!((If false 1 2))), Object::Num(2));

        assert_eq!(eval(ast!((If 1 1 2))), Object::Num(1));
        assert_eq!(eval(ast!((If 0 1 2))), Object::Num(2));
    }

    #[test]
    fn test_ast_macro() {
        assert_eq!(
            ast!((+ 1 2)),
            AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)))
        );

        assert_eq!(
            ast!((+ (+ (+ (+ 1 2) 3) 4) 5)),
            AST::Add(
                Box::new(AST::Add(
                    Box::new(AST::Add(
                        Box::new(AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)))),
                        Box::new(AST::Num(3)),
                    )),
                    Box::new(AST::Num(4)),
                )),
                Box::new(AST::Num(5)),
            )
        );

        assert_eq!(
            ast!((- 10 5)),
            AST::Minus(Box::new(AST::Num(10)), Box::new(AST::Num(5)))
        );

        assert_eq!(ast!(true), AST::Bool(true));
        assert_eq!(ast!(false), AST::Bool(false));
        assert_eq!(
            ast!((If 1 2 3)),
            AST::If {
                cond: Box::new(AST::Num(1)),
                then: Box::new(AST::Num(2)),
                els: Box::new(AST::Num(3))
            }
        );
    }
}
