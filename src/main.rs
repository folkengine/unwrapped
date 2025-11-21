fn call_something() -> usize {
    do_something().unwrap()
}

fn do_something() -> Result<usize, &'static str> {
    Err("Nope")
}

fn main() {
    let _ = call_something();
}
