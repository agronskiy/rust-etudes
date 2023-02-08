fn main() {
    let y = my_func();
    println!("Hello, world, number is {y}!");
}

fn my_func() -> i64 {
    let x: i64 = 10;
    x
}

#[test]
fn some_test() {
    assert_eq!(my_func(), 10);
}
