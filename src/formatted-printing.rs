fn main() {
    // `{}` will automatically stringify any argument value
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumped over");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
}
