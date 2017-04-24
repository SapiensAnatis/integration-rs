extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
//use std::io;
//use std::io::prelude::*;   

fn main() {
    //let mut input_exp = String::new();
    let preset_exp = "sin ( max ( 2, 3 ) / 3 * 3.1415 )";
    println!("Please enter an equation of the form y = f(x):\n");
    print!("y = ");
    /*io::stdout().flush();
    io::stdin().read_line(&mut input_exp);*/
    print!("{}", preset_exp);
    let (is_valid, reason) = expression_is_valid(&preset_exp);
    if is_valid {
        let exp = clean_expression(&preset_exp, &4f64);
        println!("\nExpression: y = {}", preset_exp);
        println!("Reformatted as: y = {}", exp);
        println!("Shunting Yard RPN result: ");
        for thing in shunting_yard(&exp) {
            print!("{} ", thing);
        }
    } else {
        println!("Invalid expression: {}", reason);
    }
    
}

fn expression_is_valid(exp: &str) -> (bool, &str) {
    // Used to match for non-x variables. Can't do that as I did before because we now support sin/cos/tan
    if exp.matches("(").count() != exp.matches(")").count() {
        // other cases here. example included:
        return (false, "Unclosed brackets detected.")
    } else {
        return (true, "Valid expression!");
    }
}

fn clean_expression(exp: &str, x: &f64) -> String {
    /* Make sure everything is formatted well enough for shunting yard
       e.g. sin(max(2, 3) / 3 * 3.1415)(2*x) to
       sin ( max ( 2, 3 ) / 3 * 3.1415 ) * ( 2 * 19 ) for x = 19
       Specifically, this is needed to make tokenization easier.
    */
    
    let mut modified_exp = String::from(exp.replace(" ", "")); // Create an actual string from our ref string
    // Remove all whitespace so we can be sure of what we're dealing with

    // We'll first parse x so we have a fully non-algebraic expression:
    modified_exp = modified_exp.replace(" ", "");
    // if there is no number in front of x, make it 1*x for simplicity
    for lone_x in Regex::new(r"[^ma\w]x").unwrap().find_iter(exp) {
        // True start is start+1 since I can't use lookbehinds in Rust
        // Also don't match ma so as not to get confused with max
        modified_exp.insert_str(lone_x.start()+1, "1"); // change just 'x' to '1x' so the next bit of code is universal
        println!("{}", modified_exp);
    }
    modified_exp = modified_exp.replace("x1", "1x"); // for some reason this can happen with powers. temporary hack
   
    // Now we replace 4x with 4*x so it makes sense. This is why we changed x to 1x, so it's now 1*x.
    modified_exp = modified_exp.replace("x", "*x");
    
    // Have bracket multiplication make sense to SY algorithm:
    modified_exp = modified_exp.replace(")(", ")*(");

    for c in r"+-*/,|^()".chars() {
        modified_exp = modified_exp.replace(c, &format!(" {} ", c));
        // Space everything out to make tokenization easier. The main feature of this function
    }
    
    // There will be _some_ double whitespace (bracket multiplication comes to mind), which we'll remove:
    modified_exp = Regex::new(r"\s{2,}").unwrap()
                    .replace_all(&modified_exp, " ")
                    .into_owned(); // Convert COW object into an actual usable String
    // Finally put in x:
    modified_exp = modified_exp.replace("x", &x.to_string());
    // the replace with x stuff picks on the last letter of max(a,b)
    modified_exp = modified_exp.replace(&format!("ma * {}", x), "max"); // hence, "temporary" hack 2
    
    return modified_exp;
}

fn shunting_yard(exp: &str) -> Vec<String> {
    // Shunting yard. This is way over my head to comment so refer to: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    // I mean, I know each of the steps but I have no idea why they're there
    let mut stack = Vec::new();
    let mut output = Vec::new();
    // Tokenize exp:
    for token in exp.trim().split(" ") {
        // First check if it's a number
        let token_number = token.parse::<f64>();
        if token_number.is_ok() {
            // If so, simply push to stack
            output.push(token); // must push a string to maintain typing
        } // Function token?
        else if token_is_function(token) {
            // If so, simply push to stack (yet again). Trust me, it gets worse later.
            stack.push(token);
            //TraceStackAndOutput(&stack, &output);
        } // function argument seperator
        else if token == "," {
            // getting there. Now we have to look for the next left parentheses.
            while stack.last() != Some(&"(") {
                output.push(stack.pop().unwrap());
                if stack.is_empty() { // no left parenthesis found
                    println!("Syntax error encountered.");
                    // TODO: handle
                }
            }
        } // If it's an operator. probably most complicated branch
        else if token_is_operator(token) { 
            let (o1_is_right_assoc, o1_priority) = priority_and_associativity(token);
            while let Some(&top) = stack.last() {
                if token_is_operator(top) {
                    let (_, o2_priority) = priority_and_associativity(top);
                    if (!o1_is_right_assoc && o1_priority <= o2_priority) || (o1_is_right_assoc && o1_priority < o2_priority) {
                        output.push(stack.pop().unwrap());
                    } else { break; }
                }
                else { break; }
            }
            // once iteration is over, push the original token
            stack.push(token);
            //TraceStackAndOutput(&stack, &output);
        }
        else if token == "(" {
            // Left brackets just get shoved onto the stack no questions asked
            stack.push(token);

            //TraceStackAndOutput(&stack, &output);
        }
        else if token == ")" {
            // Right brackets, however, trigger an investigation for their brother ;(
            while let Some(&element) = stack.last() {
                if element == "(" {
                    break;
                }
                else {
                    output.push(stack.pop().unwrap());
                }
            }

            stack.pop(); // the left parenthesis we were looking for
            // Pop it, but don't add it to the output. RPN doesn't use parentheses.
            
            if let Some(&element) = stack.last() {
                if token_is_function(element) {
                    output.push(stack.pop().unwrap());
                }
            }
            //TraceStackAndOutput(&stack, &output);
        } else { println!("Syntax error! Token {} was not identified as any type of token. Therefore, the resulting RPN expression is probably incorrect", token); }
    } 
    
    for _ in stack.clone() {
        output.push(stack.pop().unwrap()); // Push all remaining contents of stack to output
    }
    //TraceStackAndOutput(&stack, &output);
    // Return. We have a vec of references, so we'll need to sort that by cloning them individually.
    let mut typed_output: Vec<String> = Vec::new();
    for string_ref in output {
        typed_output.push(string_ref.to_string());
    }

    return typed_output;
} 

fn token_is_function(token: &str) -> bool { // Determines whether a given token is a function op by seeing if it's 2 or more letters
    lazy_static! {
        static ref IS_FUNC_REGEX: Regex = Regex::new(r"\w{2,}").unwrap();
        // Compiling a regex can take quite a while, so I use this crate to only do it once.
    }
    return IS_FUNC_REGEX.is_match(token);
}


fn token_is_operator(token: &str) -> bool { // kind of a hack. Determines if a token is one of the characters which constitutes an operator
    return r"+-/^*".matches(token).count() > 0
}

fn priority_and_associativity(token: &str) -> (bool, u8) { // Matches ops to associativity and precedence/priority
    match token {
        "^" => return (true, 0), // Is right associative and highest priority
        "*" | "/" => return (false, 1), // Is left associative
        "+" | "-" => return (false, 2), // Least priority
        _ => return (false, 0), // what
    }
}