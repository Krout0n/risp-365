use std::collections::HashMap;

use risp::{ast, eval};

fn main() {
    let mut env = HashMap::new();
    // let plus_two = ast!((Define plus_two (Func (x) (+ x 2))));
    // eval(plus_two, &mut env);

    // let app = ast!((Apply plus_two 3));
    // let obj = eval(app, &mut env);
    // dbg!(&obj);

    // let y = ast!((Define y 10));
    // eval(y, &mut env);
    // let app = ast!((Apply plus_two y));
    // let obj = eval(app, &mut env);
    // dbg!(obj);
    let sum = ast!(
    (Define sum
        (Func (n)
            (If (== n 1)
                1
                (+ n (Apply sum (- n 1)))
            ))));
    eval(sum, &mut env);
    let sum_app = ast!((Apply sum 100));
    let res = eval(sum_app, &mut env);
    dbg!(&res);
}
