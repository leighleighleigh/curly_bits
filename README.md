# curly_bits

I wanted a simple **file templating library** that could:
 1. Load a template file (HTML, Markdown, etc.) at **macro-evaluation/pre-compilation** time
 2. Extract all the `{{curly_bits}}` in the file, and generate a single `struct` containing them
 3. Allow me to populate the fields of the struct however I please
 4. At **runtime**, replace the `{{curly_bits}}` with the desired struct contents.

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
