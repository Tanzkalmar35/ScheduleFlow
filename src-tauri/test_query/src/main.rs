use simple_query::log_fn;

#[log_fn]
fn hello() {
    println!("Hello, world!");
}

fn main() {
    hello();
}
