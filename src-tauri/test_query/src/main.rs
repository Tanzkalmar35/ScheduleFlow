use simple_query::log_fn;

#[log_fn]
fn hello(pg_driver: &PgDriver) {
    println!("Hello, world!");
}

fn main() {
    hello();
}
