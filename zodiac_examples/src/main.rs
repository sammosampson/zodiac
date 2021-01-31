extern crate zodiac;
use zodiac::initialisation::{ initialise, Error };

fn main() {
    if let Err(e) = run() {
        println!("{:?}", e);
    }
}

fn run() -> Result<(), Error> {
    initialise("assets/test_zods")?;
    Ok(())
}