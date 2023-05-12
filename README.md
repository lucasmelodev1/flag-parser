# Flag Parser

Provides tools for parsing flags in command line arguments.

## Usage

#### get_flags()

```Rust
   let input = "-a -b --long-flag-a --long-flag-b";
let flags = flag_parser::get_flags(input);
// flags = ["a", "b", "long-flag-a", "long-flag-b"]

flags.contains("a") // true
```