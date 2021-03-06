extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::io;
use std::io::prelude::*;   

fn main() {
    //let mut input_exp = String::new();
    println!("Please enter an equation of the form y = f(x):");
    let input_exp = get_input("y =");

    // The accuracy value is used as the number of strips in the trapezium rule, and its reciprocal is used as the rectangle width in the rectangle rule
    let accuracy_str = get_input("Please enter an accuracy value (higher = better, integers above zero)");
    // These next ones should be self-explanatory
    let start_str = get_input("Where on the x-axis does the region start?");
    let end_str = get_input("Where does it end?");

    // Parse args
    let accuracy = accuracy_str.trim().parse::<u64>()
        .expect("Real numbers only, please.");
    if accuracy == 0 { // Don't check for less than zero because that will fail the above parse check (it parses to unsigned)
        println!("The acuracy value has to be a non-zero positive integer!");
        return;
    }

    let start = start_str.trim().parse::<f64>()
        .expect("Real numbers only, please.");
    let end = end_str.trim().parse::<f64>()
        .expect("Real numbers only, please.");

    let (is_valid, reason) = expression_is_valid(&input_exp);
    if is_valid {
        let trap_estimate = trapezium_rule(&input_exp, &start, &end, &accuracy);
        let rect_estimate = rectangle_rule(&input_exp, &start, &end, &(1f64/(accuracy as f64)));

        let simpson_estimate = (2f64*rect_estimate + trap_estimate) / 3f64;
        println!("Simpson rule estimate for the area bounded by the curve y = {} and the x-axis between {} and {}:", input_exp, start, end);
        println!("\t{}", simpson_estimate);

    } else {
        println!("Invalid expression: {}", reason);
    }
    
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("\n{} ", prompt);
    io::stdout().flush().ok().expect("Couldn't flush stdout");
    io::stdin().read_line(&mut input).expect("Couldn't read line");
    return input;
}


// Formatting/validation funcs

fn expression_is_valid(exp: &str) -> (bool, &str) {
    // Used to match for non-x variables. Can't do that as I did before because we now support sin/cos/tan
    if exp.matches("(").count() != exp.matches(")").count() {
        // other cases here. example included:
        return (false, "Unclosed brackets detected.")
    } else {
        return (true, "Valid expression!");
    }
}

fn clean_expression(exp: &str) -> String {
    /* Make sure everything is formatted well enough for shunting yard
       e.g. sin(max(2, 3) / 3 * 3.1415)(2*x) to
       sin ( max ( 2, 3 ) / 3 * 3.1415 ) * ( 2 * 19 ) for x = 19
       Specifically, this is needed to make tokenization easier.
    */
    
    let mut modified_exp = String::from(exp.replace(" ", "")); // Create an actual string from our ref string
    // Remove all whitespace so we can be sure of what we're dealing with

    // Change 4x into 4*x
    let mut stars_added = 0;
    for not_a_function_x in Regex::new(r"\d+x").unwrap().find_iter(&exp.replace(" ", "")) {
        // println!("Found match {} at {}-{}", not_a_function_x.as_str(), not_a_function_x.start(), not_a_function_x.end());
        modified_exp.insert_str(not_a_function_x.end()-1+stars_added, "*");
        stars_added += 1; // need to update info to keep indices relevant
    }
    // println!("Fixed x coefficients: {}", modified_exp);
                    

    // Have bracket multiplication make sense to SY algorithm:
    modified_exp = modified_exp.replace(")(", ")*(");

    // modified_exp = modified_exp.replace(")x", ")*x"); Already handled by the most recent regex
    modified_exp = Regex::new(r"[^a]x\(").unwrap() // match x(, again without picking on max()
                    .replace_all(&modified_exp, "x*(")
                    .into_owned();

    for bracket_coefficient in Regex::new(r"\d\(").unwrap().find_iter(exp) {
        modified_exp.insert_str(bracket_coefficient.end()-1, "*"); // Change a(x) to a*(x) for comprehension  
    }

    modified_exp = modified_exp.replace("−", "-"); // Some people like to be special and use a weird minus sign.

    for c in r"+-*/,|^()".chars() {
        modified_exp = modified_exp.replace(c, &format!(" {} ", c));
        // Space everything out to make tokenization easier. The main feature of this function
    }
    
    // There will be _some_ double whitespace (bracket multiplication comes to mind), which we'll remove:
    // Use a regex to get 2 or more double spaces, which isn't possible with conventional replace() which will only do exactly two
    // (or however many you give it in the search string)
    modified_exp = Regex::new(r"\s{2,}").unwrap()
                    .replace_all(&modified_exp, " ")
                    .into_owned(); // Convert COW object into an actual usable String
        
    // Subsititute in constants:

    modified_exp = modified_exp.replace(" e ", "2.71828182845");
    modified_exp = modified_exp.replace(" pi ", "3.14159265359");
    modified_exp = modified_exp.replace(" π ", "3.14159265359");

    return modified_exp;
}



// Shunting yard & component funcs. Some are also used in RPN.

fn shunting_yard(exp: &str) -> Vec<String> {
    // Shunting yard. This is way over my head to comment so refer to: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    // I mean, I know each of the steps but I have no idea why they're there
    let mut stack = Vec::new();
    let mut output = Vec::new();
    // Tokenize exp:
    for token in exp.trim().split(" ") {
        // First check if it's a number
        if token_is_number(token) || token == "x" || token == "pi" || token == "π" || token == "e" {
            // If so, simply push to stack
            output.push(token);
        } // Function token?
        else if token_is_function(token) {
            // If so, simply push to stack (yet again). Trust me, it gets worse later.
            stack.push(token);
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
        }
        else if token == "(" {
            // Left brackets just get shoved onto the stack no questions asked
            stack.push(token);      
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
             
        } else { println!("Syntax error! Token {} was not identified as any type of token. Therefore, the resulting RPN expression is probably incorrect", token); }

    } 
    
    for _ in stack.clone() {
        output.push(stack.pop().unwrap()); // Push all remaining contents of stack to output
    }
     
    // Return. We have a vec of references, so we'll need to sort that by cloning them individually.
    return convert_str_vec(&output);
} 

fn token_is_number(token: &str) -> bool {
    let token_number = token.parse::<f64>();
    return token_number.is_ok();
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
        "^" => return (true, 3), // Is right associative and highest priority
        "*" | "/" => return (false, 2), // Is left associative
        "+" | "-" => return (false, 1), // Least priority
        _ => return (false, 0), // what
        // this function only gets called after it is established that token = one of the above
    }
}

fn convert_str_vec(vec: &Vec<&str>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for string_ref in vec {
        output.push(string_ref.to_string());
    }

    return output;
}

// RPN funcs


fn evaluate_postfix(stack: &Vec<String>, x: f64) -> f64 {
    let mut output: Vec<f64> = Vec::new();

    while output.len() != 1 { // Process
        for token in stack.clone() {
            if token_is_number(token.as_str()) {
                output.push(token.parse::<f64>().expect("Failed to convert numerical token to f64"));
            } 

            else if token == "x" {
                output.push(x);
            }
            
            else { // The token is an op/func
                let num_args = match token.as_str() { // How many args does our function/op need?
                    // Because we're including operators, having one argument is actually outside of the norm:
                    "sin" | "cos" | "tan" | "ln" | "sqrt" | "cbrt" | "floor" | "ceil" | "round" | "trunc" | "frac" => 1,
                    // (consdier +, -, *, etc which all need two)
                    _ => 2, // none of those supported will need 3 as far as I know
                };

                if output.len() < num_args {
                    // There was a syntax error; the values needed could not be found
                    println!("Syntax error encountered: (not enough args for token {}: needed {} but only found {} remaining values in stack).\nThe result will most likely be incorrect.", token, num_args, output.len());
                    // TODO: handle these errors properly
                }
                else {
                    let mut operands: Vec<f64> = Vec::new();
                    for _ in 0..num_args {
                        // Pop n values off of the stack (the range upper limit is exclusive, so we start at 0)               
                        let operand = output.pop().expect("Failed to pop stack into operands.");
                        operands.push(operand);
                    }
                    operands.reverse(); // For some reason, operands[1] is the first argument when there are multiple variables.
                    // Evalulate operator for 1-arg functions:
                    let result: f64 = match token.as_str() {
                        // This is the equivalent of what an eval() call would do normally.
                        // Except, we didn't include the "exploit" case, so they can't. Much safer.

                        // At the very least, I've implemented most of the functions provided by f64.
                        // Most meaning I've excluded those easily done by operators like exp2(self) which returns 2^self. 

                        // That doesn't include ln, sqrt, and cbrt which can easily be done through powers, but are
                        // still commonplace in most expressions. Those are widely used, so we'll allow them to ensure
                        // compatibility and ease of use.

                        // Basic ops:
                        "*" => operands[0] * operands[1],
                        "/" => operands[0] / operands[1],
                        "+" => operands[0] + operands[1],
                        "-" => operands[0] - operands[1],
                        "^" => operands[0].powf(operands[1] as f64), // might as well just always use float to be safe
                        // Funcs 
                        // Trig (radians is assumed):
                        "sin" => operands[0].sin(),
                        "cos" => operands[0].cos(),
                        "tan" => operands[0].tan(),
                        "arcsin" => operands[0].asin(),
                        "arccos" => operands[0].acos(),
                        "arctan" => operands[0].atan(),
                        "sinh" => operands[0].sinh(),
                        "cosh" => operands[0].cosh(),
                        "tanh" => operands[0].tanh(),
                        "asinh" => operands[0].asinh(),
                        "acosh" => operands[0].acosh(),
                        "atanh" => operands[0].atanh(),
                        // Misc funcs
                        "floor" => operands[0].floor(),
                        "ceil" => operands[0].ceil(),
                        "round" => operands[0].round(),
                        "trunc" => operands[0].trunc(),
                        "fract" => operands[0].fract(),
                        "abs" => operands[0].abs(),
                        "sqrt" => operands[0].sqrt(),
                        "ln" => operands[0].ln(),
                        "log" => operands[1].log(operands[0]), // assuming argument one is base; the func is number.log(base)
                        "max" => operands[0].max(operands[1]),
                        "min" => operands[0].min(operands[1]),
                        "cbrt" => operands[0].cbrt(),
                        "exp" => operands[0].exp(),
                        _ => 0f64,
                        // Rust's matching syntax means this block of code is a lot better than it _could_ be,
                        // but it's still pretty bad.
                    };

                    output.push(result);
                }
            }
        }
    }
    if output.len() == 1 {
        return output[0];
    } else if output.len() > 1 {
        println!("Syntax error! Too many values (stack length at the end of postfix algorithm exceeded one).");
        return 0f64;
    }
    else {
        println!("An unknown error occured.");
        return 0f64;
    }
}



fn trapezium_rule(exp: &str, min_x: &f64, max_x: &f64, strips: &u64) -> f64 { // Integrate: the actual purpose of this program, after only 350 lines
    // Calculate strip width

    let interval = (max_x - min_x).abs();
    let strip_width = interval / strips.clone() as f64;
    let mut y_values: Vec<f64> = Vec::new();
    // RPN our exp
    let clean_exp = &clean_expression(&exp);
    let rpn = shunting_yard(clean_exp);
    // Get y-values for formula
    for i in 0..strips+1 {
        // x = min_x + strip_width * i
        y_values.push(
            evaluate_postfix(&rpn, min_x + strip_width * (i as f64)) 
        );
    }

    let mut result: f64 = 0f64;
    // 0.5h(a + z + 2(b....y))
    // First add the outer ones
    result += y_values.pop().unwrap();
    result += y_values[0];
    y_values.remove(0);
    // Then add all except the outer ones again
    for value in &y_values { result += 2f64 * value; }
    // Finally multiply all by 0.5 * h
    result *= 0.5f64 * strip_width;
    return result;
}

fn rectangle_rule(exp: &str, min_x: &f64, max_x: &f64, rect_width: &f64) -> f64 {
    let interval = max_x - min_x;

    // First add up all the rectangles which fit wholly into the given interval

    // This value could be cast as a u64 immediately but would then have to be recast several times to f64 for future multiplication with f64s
    // By contrast, not initially casting it to u64 means that we only have to cast it once to u64 for use in a for loop statement below
    let rect_count = (interval / rect_width).floor(); 

    let clean_exp = &clean_expression(&exp);
    let rpn = shunting_yard(clean_exp);

    let mut result = 0f64;

    // For loop which iteratively calculates the area of each rectangle and adds it to total:   
    for i in 0..(rect_count as u64) {
        let rect_start = rect_width * (i as f64);
        let rect_end = rect_width * ((i+1) as f64);
        let rect_midpoint = (rect_start + rect_end) / 2f64;

        let y_midpoint = evaluate_postfix(&rpn, rect_midpoint);

        result += y_midpoint * rect_width;
    }

    // Now we have to consider anything left over at the end:
    let rect_remainder = interval % rect_width;
    let remainder_midpoint = ((rect_count * rect_width) + max_x) / 2f64;
    let remainder_y = evaluate_postfix(&rpn, remainder_midpoint);
    
    result += remainder_y * rect_remainder;

    return result;
}
