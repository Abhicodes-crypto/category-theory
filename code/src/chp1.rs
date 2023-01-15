// Function which takes two function as arguments
// and returns their composition
// f : A -> B , g : B -> C
// create g(f(a)) : A -> C

fn f<A, B>(a: A) -> B {
    todo!()
}
fn g<B, C>(b: B) -> C {
    todo!()
}

fn c<A, B, C>(f: fn(A) -> B, g: fn(B) -> C) -> impl Fn(A) -> C {
    move |a| g(f(a))
}

fn add_1(i: i32) -> i32 {
    i + 1
}
fn mul_2(i: i32) -> i32 {
    i * 2
}

pub fn call() {
    let composition = c(add_1, mul_2);
    println!("{}", composition(3));
}
