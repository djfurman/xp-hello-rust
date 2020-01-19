fn main() {
    // `{}` will automatically stringify any argument value
    println!("{} days", 31);

    // Positional arguments work
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // Named arguments work
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumped over"
    );

    // Anonymous arguments with type casting works too
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    /*
     * Text align work can be specified
     * This output will be "     1"
     * 5 white spaces \x20 and a number "1"
     */
    println!("{number:>0width$}", number = 1, width = 6);

    // Rust checks to make sure correct argument counts are passed
    println!("My name is {0}, {1} {0}", "Bond", "James");

    /*
     * Complex types do not make this as simple
     * For structures, more complex handling simply will not work
     */
    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));
    /*
     * Uncommenting th eabove line returns
     * error[E0277]: `main::Structure` doesn't implement `std::fmt::Display`
     *   --> src/formatted-printing.rs:38:49
     *       |
     *   38 |     println!("This struct `{}` won't print...", Structure(3));
     *       |                                                 ^^^^^^^^^^^^ `main::Structure` cannot be formatted with the default formatter
     *       |
     *   = help: the trait `std::fmt::Display` is not implemented for `main::Structure`
     *   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
     *  = note: required by `std::fmt::Display::fmt`
     *
     *  error: aborting due to previous error
     */

     // Challenge Work
     let pi = 3.141592
     println!("Pi is roughly {0:.3}", pi);
}
