use std::io::{stdin, stdout};

pub fn interact(f: impl Fn(String) -> String) -> Result<(), std::io::Error> {
    super::interact(stdin().lock(), stdout(), f)
}
