fn main() {
    // format {} with value
    println!("{} days", 31);

    /* format with position */
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");

    // format with named arguments
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // different formatting
    println!("Base 10:               {}", 580);
    println!("Base 2 (binary):       {:b}", 580);
    println!("Base 8 (octal):        {:o}", 580);
    println!("Base 16 (hexadecimal): {:x}", 580);

    // right-justify
    println!("{number:>5}", number=1);
    // pad numbers
    println!("{number:0>5}", number=1);
    // pad numbers, left-adjust
    println!("{number:0<5}", number=1);
    // named arguments in format with `$`
    println!("{number:0>width$}", number=1, width=5);
    /*
     * For Rust 1.58 and above, you can directly capture the argument from a 
     * surrounding variable. Just like the above, this will output 
     * "    1", 4 white spaces and a "1".
     */
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // Rust checks the correct number of arguments
    /*
     * println!("My name is {0}, {1} {0}", "Bond");
     *
     * error: invalid reference to positional argument 1 (there is 1 argument) 
     * --> formatted-print.rs:29:32 
     * | 
     * 29 |     println!("My name is {0}, {1} {0}", "Bond"); 
     * |                                ^ 
     * | 
     * = note: positional arguments are zero-based
     * 
     * error: aborting due to 1 previous error
     */
    println!("My name is {0}, {1} {0}", "Bond", "James"); // correct

    /*
     * Only types that implement `fmt::Display` can be formatted with `{}`. User-defined
     * types do not implement it by default.
     */
    // struct Structure(i32);
    // println!("This struct `{}` won't print because doesn't implement `fmt::Display`", Structure(3));

    // print PI as 3.142
    let pi = 3.141592;
    println!("Pi is roughtly {:.3}", pi);
}
