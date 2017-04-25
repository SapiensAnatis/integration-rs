# Integration with rust

Uses trapezium rule to estimate areas under curves. Currently only supports integration with respect to x

Supported functions are all of those which are members of f64 in Rust, notably including sin/cos/tan/ln/log(https://doc.rust-lang.org/std/primitive.f64.html) with the exception of exp2, log2, log10 (these can easily be done with existing ops/funcs) and any of those listed as deprecated in the above page.

Supported operators are all standard mathematical operators, i.e. plus (+), minus (-), multiplication (*), division (/), and exponention (^). Brackets are also supported, and several multiplication shorthands are also supported. Please note that 4(8) and similar expressions currently aren't supported, but x(4) is, for example.

Pi and e are automatically substituted and pi can either be typed as "pi" or "π", with e obviously just being "e". The values of these are taken as exactly 3.14159265359 and 2.71828182845, respectively.