#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Num(usize),
    Add(Box<AST>, Box<AST>),
    Minus(Box<AST>, Box<AST>),
}

pub fn eval(ast: AST) -> usize {
    match ast {
        AST::Num(v) => v,
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
    ($num:expr) => {
        $crate::AST::Num($num)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let ast = AST::Num(1);
        assert_eq!(eval(ast), 1);

        // (1 + 2)
        // (+ 1 2)
        let simple_add = AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)));
        assert_eq!(eval(simple_add), 3);

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

        assert_eq!(eval(complicated_add), 15);

        assert_eq!(
            eval(
                // ((1 + 2) - 2)
                // (- (+ 1 2) 2)
                ast!((- (+ 1 2) 2)),
            ),
            1
        )
    }

    #[test]
    fn test_ast_macro() {
        assert_eq!(
            ast!((+ 1 2)),
            AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)))
        );

        assert_eq!(
            ast!((+ (+ (+ (+ 1 2) 3) 4 ) 5)),
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
        )
    }
}
