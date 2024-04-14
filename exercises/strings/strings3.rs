// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let rets: String = String::from(input); // note: &str is immutable type!
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut lb: bool = false;
    let mut i: usize = 0;

    for ch in (rets.clone()).chars() { //String cannot be directly access byte!
        if ch != ' ' {
            if lb == false {
                l = i;
                lb = true;
            } else {
                r = i;
            }
        }
        i += 1;
    }
      
    String::from(&input[l..=r])
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut rets = String::from(input);
    rets.push_str(" world!");
    rets
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut rets = String::from(input);
    rets = rets.replace("cars", "balloons");
    rets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
