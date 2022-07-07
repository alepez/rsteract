use std::io::{Read, Write};

pub fn interact(
    mut r: impl Read,
    mut w: impl Write,
    f: impl Fn(String) -> String,
) -> Result<(), std::io::Error> {
    let mut input = String::new();
    r.read_to_string(&mut input)?;
    let output = f(input);
    w.write_all(output.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interact_with_write_read_traits() {
        let mut input = b"1234".as_ref();
        let mut output = Vec::<u8>::new();
        let res = interact(&mut input, &mut output, |x| x.chars().rev().collect());
        assert!(res.is_ok());
        assert_eq!(b"4321".as_ref(), &output);
    }
}
