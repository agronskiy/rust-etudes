use std::string::String;

struct MyStruct<'a, 'b> {
    val: &'a str,
    val2: &'b str,
}

// From https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
// Changing it here so that 'a and 'b are used separately.
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        // ERROR: lifetime may not live long enough  consider adding the following bound: `'b: 'a`
        // y  <-- uncomment this
        x // makes borrow checked happy
    }
}

fn wrong_lifetime() {}

fn struct_lifetime() {
    let s = "some";
    let mut st = MyStruct {
        val: &s,
        val2: &"another",
    };
    {
        let s2 = String::from("some2");
        // ERROR: `s2` does not live long enough  borrowed value does not live long enough
        // st.val2 = &s2;
    }
    println!("Struct: {}", st.val2);
}

fn main() {
    struct_lifetime();
    wrong_lifetime();
}
