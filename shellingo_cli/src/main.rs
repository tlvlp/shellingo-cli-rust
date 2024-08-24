use shellingo_core::moka_test;

fn main() {
    let test: String = moka_test::moka();
    println!("Hello, world! {}", test);
}
