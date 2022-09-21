

fn main() {

}

fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + f(x) 
}

fn rust_function() -> i32 {

    let add_two = |x| { 1 + x };
    let square = |x| { x * x };
    twice(5, add_two);
    twice(6, square);

    compose(5, |n| { n + 42 }, |n| { n * 2 })
}

fn compose<F, G>(x: i32, f: F, g: G) -> i32 
    where F: Fn(i32) -> i32, G: Fn(i32) -> i32 {
    g(f(x)) 
}

fn rust_add(x: i32, y:i32) -> i32 { x + y }

// closure 
/// Rust documentation thing





