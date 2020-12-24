fn main() {
    println!("!fmt experiments!");
    println!("`std:fmt` are Utilities for formatting and printing Strings.");
    println!("The `format!` macro is implemented in the compiler to emit calls to this module in order to format arguments at runtime into strings");
    println!("-------");
    println!("the first thing I notice is that to represent the 'strings' that display `format!` commands literally, and print them to stdout with a call to `println!(...); in the source code, I need to use escape characters. This is obvious at a glance, while inspecting the source.");
    println!("format!(\"Hi {{}} you!\", \"there\");");
    let x:String = format!("Hi {} you!", "there");
    println!("{}", x);
    println!("Hi there you!");
    let y:&str = "Hi there you!"; 
    println!("{}", y);
    println!("{}", y.to_string());
    println!("format creates a String. the first argument of format! is a format string, and must be a string literal. it cannot be a variable passed in.");
    println!("Above, a String, a String literal, an &str, and an &str.to_string() an are used.");
    println!("A common use for format! is concatenation and interpolation of strings. The same convention is used with print! and write! macros, depending on the intended destination of the string.");
    println!("positional parameters might be useful in parsing?");
    println!("{}{}{}{}{}{}", "{", "}", "[", "]", "(", ")");
    println!("{2}{3}{0}{1}{4}{5}", "{", "}", "[", "]", "(", ")");
    println!("named parameters are also allowed");
    println!("my name is {}", name="bob");
    println!("I think the advantage here would be mostly to provide a shorthand/handle in a complicated formatting expression, to help make it more readable.");
    println!("");
    println!("There are odd looking formatting parameters, involving minimum width, display column alignment, precision of long numeric values, and signs altering the behavior of the formatter. For example:");
    println!("Hello {:5}...", "x");
    println!("full grammar of format strings:");
    println!("");
    let format_string_grammar = "format_string := <text> [ maybe-format <text> ] *
maybe-format := '{' '{' | '}' '}' | <format>
format := '{' [ argument ] [ ':' format_spec ] '}'
argument := integer | identifier

format_spec := [[fill]align][sign]['#']['0'][width]['.' precision][type]
fill := character
align := '<' | '^' | '>'
sign := '+' | '-'
width := count
precision := count | '*'
type := identifier | '?' | ''
count := parameter | integer
parameter := argument '$'";
    println!("{}", format_string_grammar);
    println!("");
    println!("Passing this whole block of text in as a variable defined before the formatting was much easier than trying to worry about escape characters");
    println!("The above table can almost been seen like a style sheet, reading top to down with different functionality. ");
    println!("this next example is using a sign to use an alternate form of printing (transforming the value to hex), as well as prepending the result of this evaluation with `0x` ");
    println!("format!(\"{{:#x}}\", 27);");
    let hex_example = format!("{:#x}", 27);
    println!("{}", hex_example);
    println!("-------");
    println!("further notes:");
    println!("because one goal is to be able to inspect bytes, we should understand that Strings are made of bytes, the same as as a vector of bytes, Vec<u8>. To convert between the two demostring.into_bytes or vecdeom.from_utf8() (which checks for valid UTF-8) could be used. ");
    println!("");

}
