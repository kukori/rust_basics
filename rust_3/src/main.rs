#[derive(Debug)]

struct SomeStruct {
    num: i32,
}

// read only reference
fn print_some_struct(the_struct: &SomeStruct) {
    println!("{:?}", the_struct)
}

// mutable reference
fn mutate_struct(the_struct: &mut SomeStruct) {
    the_struct.num = 5;
}

// lifetimes: we specify that a and b needs to be in scope at least as long as the return value for this function
fn biggest<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct {
    if (a.num > b.num) {
        a
    } else {
        b
    }
}

fn main() {
    let mut some_struct = SomeStruct { num: 3 };
    print_some_struct(&some_struct);
    mutate_struct(&mut some_struct);
    print_some_struct(&some_struct);
}
