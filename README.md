# rsteract

A clone of *Haskell* interact function.

The interact function takes a function of type `Fn(String) -> String` as its argument.

The entire input from the standard input device is passed to this function as
its argument, and the resulting string is output on the standard output device.

## Example

If this is `your_program`:

```rust
use rsteract::stdio::interact;

fn main() -> Result<(), std::io::Error> {
    interact(|x| x.chars().rev().collect())
}
```

```shell
❯ echo 1234 | your_program
4321
```

This crate also provide a generic version for `Read` and `Write` traits.

## Use cases

Competitive programming, when you need to quickly write code to read from stdin
and write to stdout.

Feel free to copy this your code in your program, if using external crates is
not an option.
