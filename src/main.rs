extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::io;
use std::io::prelude::*;   
extern crate eval;
use eval::eval;     

const FUNCS: &'static [&'static str] = &["sin", "cos", "tan", "min", "max", "ln", "log"];                                                   


fn main() {
    let mut inputExp = String::new();
    println!("Please enter an expression of the form y = f(x):\n");
    print!("y = ");
    io::stdout().flush();
    io::stdin().read_line(&mut inputExp);
    let truExp = InfixAndSubstitute(&inputExp, &4f64);
    println!("Will feed {} to eval", truExp);
    let result = eval(&truExp).unwrap();
    println!("Eval result: ");
    println!("{}", result)

}

fn PrintVec(vecToPrint: &Vec<&str>) {
    for item in vecToPrint {
        print!("{}", item);
    }
}


fn IsValidExp(exp: &str) -> (bool, &str) {
    if Regex::new(r"[a-w]|[y-z]").unwrap().is_match(exp) {
        return (false, "Variables other than x are currently unsupported.");
    }
    else if exp.matches("(").count() != exp.matches(")").count() {
        // other cases here. example included:
        return (false, "Unclosed brackets detected.")
    }
    else {
        return (true, "Valid expression!");
    }
}

/* fn ShuntingYard(exp: &str) -> Vec<&str> {
    // Shunting yard. This is way over my head to comment so refer to: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    let mut stack = Vec::new();
    let mut output = Vec::new();
    // Tokenize exp:
    for token in exp.trim().split(" ") {
        println!("Evaluating token {}", token);
        // First check if it's a number
        let tokenNumber = token.parse::<f64>();
        if tokenNumber.is_ok() {
            println!("\tThe token was a number, so it was pushed to the output stack.");
            output.push(token); // must push a string to maintain typing
            continue;
        } // Function token?
        else if TokenIsFunction(token) {
            println!("\tThe token as a function, so it was pushed to the operator stack.");
            stack.push(token);
            continue;
        } // function argument seperator
        else if token == "," {
            println!("\tThe token was a comma, so we'll keep pushing until we find a left parenthesis.");
            while stack.last() != Some(&"(") {
                output.push(stack.pop().unwrap());
                if stack.is_empty() { // no left parenthesis found
                    println!("Syntax error encountered.");
                    // TODO: handle
                }
            }
            continue;
        } // If it's an operator
        else if TokenIsOperator(token) { 
            println!("\tThe token was an operator, so we'll establish precedence and do a bunch of weird shit");
            let (o1IsRightAssoc, o1Priority) = PrecAndAssoc(token);
            println!("\t\tThe operator {} was established to have right associativity {} and precedence {}", token, o1IsRightAssoc, o1Priority);
            loop {
                if stack.is_empty() { 
                    println!("\t\tThe stack was empty, so we stopped.");
                    stack.push(token);
                    break; 
                }

                if TokenIsOperator(stack.last().expect("wtf")) {
                    println!("\t\tThe value at the top of the stack, {}, was an operator.", stack.last().expect("wtf"));
                    let (o2IsRightAssoc, o2Priority) = PrecAndAssoc(stack.last().expect("wtf"));
                    if (!o1IsRightAssoc && o1Priority <= o2Priority) || (o1IsRightAssoc && o1Priority < o2Priority) {
                        println!("\t\teither o1 is left-associative and its precedence is less than or equal to that of o2, or o1 is right associative, and has precedence less than that of o2");
                        output.push(stack.pop().unwrap());
                        continue;
                    }
                }
                println!("\t\tNow pushing {} to the stack", token);
                stack.push(token);
                break;
            }
        }
        else if token == "(" {
            println!("\tThe token was a bracket, so we'll push it to the stack.");
            stack.push(token);
        }
        else if token == ")" {
            println!("\tThe token was a right bracket so we'll keep going until we find the next bracket.");
            while stack.last() != Some(&"(") {
                output.push(stack.pop().unwrap());
            }

            stack.pop(); // the left parenthesis we were looking for
            
            if TokenIsFunction(stack.last().unwrap()) {
                output.push(stack.pop().unwrap());
            }
        }
    }
    
    if !stack.is_empty() {
        for token in stack {
            if TokenIsOperator(token) {
                output.push(token);
                // Don't need to remove from stack as we won't use it after this.
            }
        } // very surprised if this works
    }
    output
} */

fn PrecAndAssoc(token: &str) -> (bool, u8) {
    match token {
        "^" => return (true, 0), // Is right associative and highest priority
        "*" | "/" => return (false, 1), // Is left associative
        "+" | "-" => return (false, 2), // Least priority
        _ => return (false, 0), // what
    }
}

fn TokenIsOperator(token: &str) -> bool { // kind of a hack
    return (r"+-*/^".matches(token).count() > 0)
}

fn TokenIsFunction(token: &str) -> bool {
    lazy_static! {
        static ref IS_FUNC_REGEX: Regex = Regex::new(r"\w{2,}").unwrap();
        // Compiling a regex can take quite a while, so I use this crate to only do it once.
    }
    return IS_FUNC_REGEX.is_match(token);
}

fn CalculateFromOutput( /* something */ ) -> f64 {
    // Take our output from Shunting Yard and give a result
    unimplemented!();
}

fn InfixAndSubstitute(exp: &str, x: &f64) -> String {
    /* Make sure everything is formatted well enough for shunting yard
       e.g. sin(max(2, 3) / 3 * 3.1415)(2*x) to
       sin ( max ( 2, 3 ) / 3 * 3.1415 ) * ( 2 * 19 ) for x = 19
       Specifically, this is needed to make tokenization easier.
    */
    
    let mut modifiedExp = String::from(exp.replace(" ", "")); // Create an actual string from our ref string
    // Remove all whitespace so we can be sure of what we're dealing with

    // We'll first parse x so we have a fully non-algebraic expression:
    modifiedExp = modifiedExp.replace(" ", "");
    // if there is no number in front of x, make it 1*x for simplicity
    for loneX in Regex::new(r"[^ma\w]x").unwrap().find_iter(exp) {
        // True start is start+1 since I can't use lookbehinds in Rust
        // Also don't match ma so as not to get confused with max
        modifiedExp.insert_str(loneX.start()+1, "1"); // change just 'x' to '1x' so the next bit of code is universal
        println!("{}", modifiedExp);
    }
    modifiedExp = modifiedExp.replace("x1", "1x"); // for some reason this can happen with powers. temporary hack
   
    // Now we replace 4x with 4*x so it makes sense. This is why we changed x to 1x, so it's now 1*x.
    modifiedExp = modifiedExp.replace("x", "*x");
    println!("EXP--------------------------------------------{}", modifiedExp);
    
    for c in r"+-*/,|^()".chars() {
        modifiedExp = modifiedExp.replace(c, &format!(" {} ", c));
    }
    // There will be _some_ double whitespace (bracket multiplication comes to mind), which we'll remove:
    modifiedExp = Regex::new(r"\s{2,}").unwrap()
                    .replace_all(&modifiedExp, " ")
                    .into_owned(); // Convert COW object into an actual usable String
    // Finally put in x:
    modifiedExp = modifiedExp.replace("x", &x.to_string());
    modifiedExp = modifiedExp.replace(&format!("ma * {}", x), "max"); // "temporary" hack 2
    // the replace with x stuff picks on the last letter of max(a,b)
    return modifiedExp;
}

