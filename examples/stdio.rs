use rsteract::stdio::interact;

fn main() -> Result<(), std::io::Error> {
    interact(|x| x.chars().rev().collect())
}
