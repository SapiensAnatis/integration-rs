extern crate regex;

fn main() {
    println!("Hello, world!");
}

fn Substitute(exp: &str, x: &f64) -> f64 {
    let mut accumulator: f64 = 0f64;
    let mut currentOp: &str = "";
    //let mut lastNumber: f64 = 0;
    // logic
    let comps = exp.split(" "); // split by whitespace
    for component in comps {
        // Handle each component and change the accumulator accordingly. We'll sort of do BIDMAS:
        // First and foremost, if it's a bracketed expression, we'll need to evaluate that (B):
        let bracketRegex = Regex::new("(?<=\()\S+(?=\))|(?<=\)\^)\d+"); 
        /* Let me (attempt) to explain:

           You have two capturing groups separated by an OR operator as the second one checks for indices,
           and so is optional. 

           The first one is ((?<=\()\S+(?=\))), which is a lookbehind+ahead for bracket characters,
           greedily matching any non-whitespace characters (our expression inside the brackets) within.
           The second one is ((?<=\)\^)\d+), which is again a lookbehind, but this time for a closing bracket character
           and a hat, then matches the number (power) that the bracket is being raised to. */
        // So, yeah, bracket logic:
        if bracketRegex.is_match(component) {
            
            // Before adding to the accumulator we need to decide if we're going to raise what we add to any powers:
            let mut power: u32 = 1;

            let matches = bracketRegex.captures_iter(component);
            let internalExpression = matches.nth(0).as_str(); // &str
            if matches.nth(1) != None { // if there was a power
                // Wait until here to get our string; I believe calling as_str() on None will cause an error
                power = matches.nth(1).trim().parse()
                    .expect("Power was not a positive integer number!");
            } else {
                power = 1;
            }
            
            powerAccumulator: f64 = 1f64; // Initialize our acc to 1
            for i in 0..power+1 { // Upper bound is exclusive, so to perform an operation *n* times, we go to n+1
                powerAccumulator *= Substitute(internalExpression, x); // Multiply it by the value of our bracket
            }

            accumulator += powerAccumulator; // evaluate the regex match with our existing value of x
        }
        // Indices (non-bracket wise) logic (I):
        unimplemented!();
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
        // other cases here. example included:
        return (false, "Maths is currently broken. Please try again later.")
    }
    else {
        return (true, "Valid expression!");
    }
}