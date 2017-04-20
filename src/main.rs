extern crate regex;

fn main() {
    println!("Hello, world!");
}

fn Substitute(exp: &str, x: &f64) -> f64 {
    let mut accumulator: f64 = 0;
    //let mut lastNumber: f64 = 0;
    // logic
    let comps = exp.split(" "); // split by whitespace
    for component in comps {
        // Handle each component and change the accumulator accordingly:
        // First and foremost, if it's a bracketed expression, we'll need to evaluate that (B):
        let bracketRegex = Regex::new("(?<=\()\S+(?=\))"); // Greedy non-whitespace quantifier inbetween a lookbehind & ahead for bracket characters
        if bracketRegex.is_match(component) {
            let internalExpression = bracketRegex.find(component).unwrap();
            accumulator += substitute(internalExpression, x); // evaluate the regex match with our existing value of x
        }
    }
    // return value
    return accumulator
    
}

fn IsValidExp(exp: &str) -> (bool, &str) {
    otherVariablesRegex = Regex::new("[a-w]|[y-z]");
    if otherVariablesRegex.is_match(exp) {
        return (false, "Variables other than x are currently unsupported!");
    }
    else if (4 == 2) {
        // other cases here
    }
    else {
        return (true, "Valid expression!");
    }
}