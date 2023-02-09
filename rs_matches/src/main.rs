enum TestEnum<T, E> {
    One,
    Two(T),
    Three { one: E, two: i32, three: char },
}

// Nice abstraction for printing the type
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let test: TestEnum<(), i32> = TestEnum::Three {
        one: 10,
        two: 42,
        three: 'b',
    };
    let test_two: TestEnum<i32, ()> = TestEnum::Two(100);
    let test_one: TestEnum<(), ()> = TestEnum::One;
    match test {
        TestEnum::One => {}
        TestEnum::Two(i) => print_type_of(&i),
        // example in which the order of unpacked variables is different -- name matters.
        TestEnum::Three { two, .. } => println!("three {two}"),
    }
    match test_two {
        TestEnum::One => {}
        TestEnum::Two(i) => print_type_of(&i),
        // example in which the order of unpacked variables is different -- name matters.
        TestEnum::Three { two, .. } => println!("three {two}"),
    }
    match test_one {
        TestEnum::One => {}
        TestEnum::Two(i) => print_type_of(&i),
        // example in which the order of unpacked variables is different -- name matters.
        TestEnum::Three { two, .. } => println!("three {two}"),
    }
}
