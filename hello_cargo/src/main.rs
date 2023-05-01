fn main() {
    let y = my_func() + 10;
    println!("Hello, world, number is {y}!");
}

fn my_func() -> i64 {
    let x: i64 = 10;
    let y = x + 10;
    let z = y + 10;
    return z;
}

#[test]
fn some_test() {
    assert_eq!(my_func(), 10);
}
