use std::collections::HashMap;

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
    Equal(Box<AST>, Box<AST>),
    Define {
        name: String,
        value: Box<AST>,
    },
    Ident(String),
    Function {
        arg: String,
        body: Box<AST>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(usize),
    Bool(bool),
    Function { arg: String, body: Box<AST> },
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

pub fn eval(ast: AST, env: &mut HashMap<String, Object>) -> Object {
    match ast {
        AST::Num(v) => Object::Num(v),
        AST::Add(left, right) => {
            let left_obj = eval(*left, env);
            let right_obj = eval(*right, env);
            left_obj + right_obj
        }
        AST::Minus(left, right) => {
            let left_obj = eval(*left, env);
            let right_obj = eval(*right, env);
            left_obj - right_obj
        }
        AST::Bool(b) => Object::Bool(b),
        AST::If { cond, then, els } => match eval(*cond, env) {
            Object::Bool(true) => eval(*then, env),
            Object::Bool(false) => eval(*els, env),
            Object::Num(v) if v != 0 => eval(*then, env),
            Object::Num(_) => eval(*els, env),
            _ => unimplemented!(),
        },
        AST::Equal(left, right) => Object::Bool(eval(*left, env) == eval(*right, env)),
        AST::Define { name, value } => {
            let value = eval(*value, env);
            env.insert(name, value.clone());
            value
        }
        AST::Ident(id) => {
            if let Some(obj) = env.get(&id) {
                obj.clone()
            } else {
                panic!("given ident {} is not defined", id)
            }
        }
        AST::Function { arg, body } => Object::Function { arg, body },
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
    ((== $left:tt $right:tt)) => {
        $crate::AST::Equal(Box::new(ast!($left)), Box::new(ast!($right)))
    };
    ((If $cond:tt $then:tt $els:tt)) => {
        $crate::AST::If {
            cond: Box::new(ast!($cond)),
            then: Box::new(ast!($then)),
            els: Box::new(ast!($els)),
        }
    };
    ((Define $name:ident $value:tt)) => {
        $crate::AST::Define {
            name: std::stringify!($name).to_string(),
            value: Box::new(ast!($value)),
        }
    };
    ((Func $arg:ident $body:tt)) => {
        $crate::AST::Function {
            arg: std::stringify!($arg).to_string(),
            body: Box::new(ast!($body)),
        }
    };
    // $name:ident にマッチしてしまうので先に書いておく
    (true) => {
        $crate::AST::Bool(true)
    };
    (false) => {
        $crate::AST::Bool(false)
    };
    ($name:ident) => {
        $crate::AST::Ident(std::stringify!($name).to_string())
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
        let mut empty_env = HashMap::new();
        let ast = AST::Num(1);
        assert_eq!(eval(ast, &mut empty_env), Object::Num(1));

        // (1 + 2)
        // (+ 1 2)
        let simple_add = AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)));
        assert_eq!(eval(simple_add, &mut empty_env), Object::Num(3));

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

        assert_eq!(eval(complicated_add, &mut empty_env), Object::Num(15));

        assert_eq!(
            eval(
                // ((1 + 2) - 2)
                // (- (+ 1 2) 2)
                ast!((- (+ 1 2) 2)),
                &mut empty_env
            ),
            Object::Num(1)
        );

        assert_eq!(eval(ast!(true), &mut empty_env), Object::Bool(true));
        assert_eq!(eval(ast!(false), &mut empty_env), Object::Bool(false));

        assert_eq!(eval(ast!((If true 1 2)), &mut empty_env), Object::Num(1));
        assert_eq!(eval(ast!((If false 1 2)), &mut empty_env), Object::Num(2));

        assert_eq!(eval(ast!((If 1 1 2)), &mut empty_env), Object::Num(1));
        assert_eq!(eval(ast!((If 0 1 2)), &mut empty_env), Object::Num(2));

        assert_eq!(
            eval(ast!((== 3 (+ 1 2))), &mut empty_env),
            Object::Bool(true)
        );
        assert_eq!(
            eval(ast!((== 0 (+ 1 2))), &mut empty_env),
            Object::Bool(false)
        );
    }

    #[test]
    fn test_eval_with_env() {
        let mut env = HashMap::new();
        let value = eval(ast!((Define x 1)), &mut env);

        assert_eq!(value, Object::Num(1));
        assert_eq!(env.get("x"), Some(&Object::Num(1)));

        assert_eq!(eval(ast!(x), &mut env), Object::Num(1));
        assert_eq!(eval(ast!((+ 3 x)), &mut env), Object::Num(4));
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

        assert_eq!(
            ast!((== 1 2)),
            AST::Equal(Box::new(AST::Num(1)), Box::new(AST::Num(2)))
        );

        assert_eq!(
            ast!((Define x 1)),
            AST::Define {
                name: "x".to_string(),
                value: Box::new(AST::Num(1))
            }
        );

        assert_eq!(ast!(x), AST::Ident("x".to_string()));
        assert_eq!(
            ast!((+ 1 x)),
            AST::Add(Box::new(AST::Num(1)), Box::new(AST::Ident("x".to_string())))
        );

        assert_eq!(
            ast!((Func x (+ x 2))),
            AST::Function {
                arg: "x".to_string(),
                body: Box::new(AST::Add(
                    Box::new(AST::Ident("x".to_string())),
                    Box::new(AST::Num(2)),
                ))
            }
        );

        assert_eq!(
            ast!((Define x (Func y (+ y 2)))),
            AST::Define {
                name: "x".to_string(),
                value: Box::new(AST::Function {
                    arg: "y".to_string(),
                    body: Box::new(AST::Add(
                        Box::new(AST::Ident("y".to_string())),
                        Box::new(AST::Num(2)),
                    ))
                })
            }
        );
    }
}
