fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let output = input.trim();
    output
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.

    //e Owned string is a heap allocated,growable string managed by your program .It ownss this data, meaning it:
    //Controls the memory, can grow or shrink,gets automatically cleaned up when it goes
    input.to_string() + "world" //e It requires a string on the left and a &str on the right
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let replaced_string = input.replace("cars", "balloons");
    replaced_string
}

fn main() {
    // You can optionally experiment here.

    let hello = String::from("      Hello,World!!       ");
    println!("{}", hello.trim());

    let string = "Abel Cherian Babu";
    let replaced_string = string.replace("Abel Cherian Babu", "Allen George Babu");
    println!("{}", replaced_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
