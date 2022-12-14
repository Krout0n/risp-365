use std::collections::HashMap;

mod impls;

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
        params: Vec<String>,
        body: Box<AST>,
    },
    Apply {
        fn_lit: Box<AST>,
        args: Vec<AST>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(usize),
    Bool(bool),
    Function { params: Vec<String>, body: Box<AST> },
}

pub fn eval(ast: AST, env: &mut HashMap<String, Object>) -> Object {
    let obj = match ast {
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
        AST::Function { params, body } => Object::Function { params, body },
        AST::Apply { fn_lit, args } => {
            let args_val = args.into_iter().map(|arg| eval(arg, &mut env.clone()));
            let fn_lit_obj = eval(*fn_lit, &mut env.clone());
            match fn_lit_obj {
                Object::Function { params, body } => {
                    let mut deep_env: HashMap<String, Object> =
                        params.into_iter().zip(args_val).collect();
                    deep_env.extend(env.clone().into_iter());
                    eval(*body, &mut deep_env)
                }
                _ => unimplemented!(),
            }
        }
    };
    // dbg!(obj)
    obj
}

// ?????????????????????????????????????????????????????????????????????????????????
// ???????????????????????????????????????????????????????????????????????????
#[macro_export]
macro_rules! ast {
    // tt ?????? `(+ 1 2)` ?????? `1` ????????????????????????
    ((+ $left:tt $right:tt)) => {
        // ????????????????????????AST???pub??????????????????????????????????????????
        // `$crate::`?????????????????????????????????????????????:pray:
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
    ((Func ($( $param:ident )*) $body:tt)) => {
        $crate::AST::Function {
            params: vec![$( stringify!($param).to_string() ), *],
            body: Box::new(ast!($body)),
        }
    };
    ((Apply $fn_lit:tt $( $arg:tt )*)) => {
        $crate::AST::Apply {
            fn_lit: Box::new(ast!($fn_lit)),
            args: vec![$( ast!($arg) ), *],
        }
    };
    // $name:ident ??????????????????????????????????????????????????????
    (true) => {
        $crate::AST::Bool(true)
    };
    (false) => {
        $crate::AST::Bool(false)
    };
    ($name:ident) => {
        $crate::AST::Ident(std::stringify!($name).to_string())
    };
    // 1 ??? true ??????????????????
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

        let mut env = HashMap::new();
        let plus_two = ast!((Define plus_two (Func (x) (+ x 2))));
        eval(plus_two, &mut env);

        let app = ast!((Apply plus_two 3));
        let obj = eval(app, &mut env);
        assert_eq!(obj, Object::Num(5));

        let f = ast!((Define f (Func (a b) (+ a (+ b 1)))));
        eval(f, &mut env);
        let f_app = ast!((Apply f 10 20));
        assert_eq!(eval(f_app, &mut env), Object::Num(31));

        let f_app = ast!((Apply (Func (a b) (+ a (+ b 1))) 100 200));
        assert_eq!(eval(f_app, &mut env), Object::Num(301));

        let g = ast!((Define g (Func (y) (If (== y 0) 1000 (Apply f 10 y)))));
        eval(g, &mut env);

        let g_app = ast!((Apply g 500));
        assert_eq!(eval(g_app, &mut env), Object::Num(511));
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
            ast!((Func () 2)),
            AST::Function {
                params: vec![],
                body: Box::new(AST::Num(2)),
            }
        );

        assert_eq!(
            ast!((Func (x) (+ x 2))),
            AST::Function {
                params: vec!["x".to_string()],
                body: Box::new(AST::Add(
                    Box::new(AST::Ident("x".to_string())),
                    Box::new(AST::Num(2)),
                ))
            }
        );

        assert_eq!(
            ast!((Define x (Func (x y) (+ y 2)))),
            AST::Define {
                name: "x".to_string(),
                value: Box::new(AST::Function {
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(AST::Add(
                        Box::new(AST::Ident("y".to_string())),
                        Box::new(AST::Num(2)),
                    ))
                })
            }
        );
    }
}
