enum OptionalInt {
    Value(i32),
    Missing
}

enum OptionalFloat64 {
    Valuef64(f64),
    Missingf64
}

// tuples

fn typles() {
    let x = (1, "hello");
    let y: (i32, &str) = (1, "hello");
}


// using generics
enum Option<T> {
    Some(T),
    None
}

enum Result<A, Z> {
    Ok(A),
    Err(Z)
}

fn enummmms() {
    let x: Result<f64, String> = Ok(2.3f64);
    let y: Result<f64, String> = Err("There was an error.".to_string());
}
