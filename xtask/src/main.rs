extern crate alloc;

mod ungram;

use core::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    ungram::policy::generate()?;
    ungram::schema::generate()?;

    Ok(())
}
