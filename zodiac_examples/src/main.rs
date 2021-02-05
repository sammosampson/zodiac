extern crate zodiac;
use zodiac::initialisation::{ initialise, ZodiacError };

fn main() {
    if let Err(e) = run() {
        println!("{:?}", e);
    }
}

fn run() -> Result<(), ZodiacError> {
    initialise("assets/test_zods")?;
    Ok(())
}