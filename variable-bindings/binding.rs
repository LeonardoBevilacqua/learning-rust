// Variable bindings have a scope, and are constrained to live in a block. A nlock is a collection
// of statemetns enclosed by braces {}
fn main() {
    // This binding live in the main function
    let long_lived_binding = 1;

    // this is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}
