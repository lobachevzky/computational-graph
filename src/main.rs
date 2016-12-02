extern crate libc;

mod function; 
mod constant; 

#[allow(unused_imports)]
use function::{input, scalar, param, matrix};
use constant::Constant::Scalar;
use constant::new_matrix;
use std::collections::HashMap;

fn main() {
    let a = scalar(-2.); 
    let b = scalar(-3.); 
    //println!("a * b: {}", &a * &b);
    let x = param( "x", new_matrix(
            2, 3, vec![
            1., 1., 1., 
            1., 1., 1.])); 
    //println!("a * x: {}", &a * &x);
    println!("x: {}", x);
    let f = (&x + &b).abs() * (&x + &a);
    println!("f: {}", f);

    let mut args = HashMap::new();
    //args.insert("x", Scalar(-3.));

    //f.assign_outputs(&args);
    //println!("args: {:#?}", args);

    //if let Some(c) = f.eval(&args) {
        //println!("eval: {}", c);
    //}

    //println!("grad x: {}", f.grad("x"));

    //println!("x: {}", &x);
    f.minimize(&args, 0.4, 40);
    //println!("x: {}", &x);
}
