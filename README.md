# curly_bits

A struct-enhanced, macro-powered crate, that replaces the {{curly_bits}} in a template file with the desired text content.<br>
(This crate is for personal use - it's really, really simple... but it does the job!)

---
*Motivation*

I wanted a simple **file templating library** that could:
 1. Load a template file (HTML, Markdown, etc.) at **macro-evaluation/pre-compilation** time
 2. Extract all the `{{curly_bits}}` in the file, and generate a single `struct` containing them
 3. Allow me to populate the fields of the struct however I please
 4. At **runtime**, replace the `{{curly_bits}}` with the desired struct contents.

---
*Result*

To use this crate, call the `load_curly_bits!` macro, pointed to a template file path. <br>
Then, it's as easy as working with a `struct` full of `String` fields, and printing it.


`$ cat ./src/main.rs | head -n14`
```
extern crate curly_bits;
use curly_bits::load_curly_bits;

load_curly_bits!("tests/simple.txt");

fn main() {
    let mut data : SimpleTemplate = SimpleTemplate::default();
    data.greeting = "Hey".to_string();
    data.name = "Dude".to_string();

    println!("#### template ({})\n{}\n", data.template_file(), data.template());
    println!("#### struct\n{:#?}\n", data);
    println!("#### result\n{}\n", data);
}
```


This results in...

`$ cargo run`
```
#### template (tests/simple.txt)
{{greeting}}, {{name}}

#### struct
SimpleTemplate {
    greeting: "Hey",
    name: "Dude",
}

#### result
Hey, Dude

```
