struct Foo {
    str_val: String,
    int_val: i32,
}

fn main() {
    let mut foos = Vec::new();
    foos.push(Foo {
        str_val: "ten".to_string(),
        int_val: 10,
    });
    foos.push(Foo {
        str_val: "twenty".to_string(),
        int_val: 20,
    });

    // ERROR"cannot move out of index of `Vec<Foo>`"
    // let moved = foos[0].str_val;

    let mut standalone = Foo {
        str_val: "thirty".to_string(),
        int_val: 30,
    };
    // Allowed to move
    let moved_thirty = standalone.str_val;

    // Allowed to move
    // let int_moved = standalone.int_val;
    // ERROR: "use of moved value: `standalone.str_val`"
    // let error = standalone.str_val;
    // ERROR: partial move
    // let another = standalone;
}
