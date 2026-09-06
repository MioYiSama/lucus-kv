use std::error::Error;

use crate::native::add;

mod native;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", unsafe { add(1, 2) });

    Ok(())
}
