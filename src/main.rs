extern crate regex;

fn main() {
    println!("Hello, world!");
}

fn EvalulateExp(exp: &str, x: &f64) -> f64 {
    // Validate expression:
    expIsValid, reason = IsValidExp(exp);
    if expIsValid {
        // Clean up expression to by-the-book infix:
        exp = InfixAndSubstitute(exp).as_str(); // EnforceInfix returns a String which we'll convert back into a &str

        // Use a shunting yard algorithm to convert our presumably infix expression into postfix:
        unimplemented!();
    }
    else {
        print!("Expression is invalid: {}", reason);
    } 
}

fn IsValidExp(exp: &str) -> (bool, &str) {
    if Regex::new(r"[a-w]|[y-z]").unwrap().is_match(exp) {
        return (false, "Variables other than x are currently unsupported.");
    }
    else if exp.matches("(") != exp.matches(")") {
        // other cases here. example included:
        return (false, "Unclosed brackets detected.")
    }
    else {
        return (true, "Valid expression!");
    }
}

fn ShuntingYard(exp: &str) -> Vec<&str> {
    // Shunting yard
    unimplemented!();
}

fn CalculateFromOutput( /* something */ ) -> f64 {
    // Take our output from Shunting Yard and give a result
}

fn InfixAndSubstitute(exp: &str, x: &str) -> String {
    /* Make sure everything is formatted well enough for shunting yard
       e.g. sin(max(2, 3) / 3 * 3.1415)(2*x) to
       sin ( max ( 2, 3 ) / 3 * 3.1415 ) * ( 2 * 19 ) 
    */
    
    let mut modifiedExp = String::from(exp.replace(" ", "")); // Create an actual string from our ref string
    // Remove all whitespace so we can be sure of what we're dealing with

    // We'll first parse x so we have a fully non-algebraic expression:
    modifiedExp = modifiedExp.replace("x", x.to_string());

    // Turn bracket multiplication into a format understood by the SY algorithm:
    modifiedExp = modifiedExp.replace(")(", ")*(");
    // Space out ops/commas/brackets:
    for c in "+-*/,|^()".chars() {
        modifiedExp = modifiedExp.replace(c, format!(" {} ", c));
    }
    // There will be _some_ double whitespace (bracket multiplication comes to mind), which we'll remove:
    modifiedExp = Regex::new(r"\s{2,}";).unwrap().replace_all(modifiedExp, " ")

    return modifiedExp;
}