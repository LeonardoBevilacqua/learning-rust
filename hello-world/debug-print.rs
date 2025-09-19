// This structure cannot be printed either with `fmt::Display` or with `fmt::Debug`
// struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation required to make this `struct`
// printable with `dmt::Debug`.
// #[derive(Debug)]
// struct DebugPrintable(i32);

// exemple
#[derive(Debug)]
struct Structure(i32);

// Make deep structure printable
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // pretty printing with `{:#?}`
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("normal: {:?}", peter);
    println!("pretty: {:#?}", peter);
}
