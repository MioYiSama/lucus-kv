use std::error::Error;

use crate::native::add;

mod arena;
mod mem_table;
mod native;
mod skip_list;
mod ss_table;
mod wal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", unsafe { add(1, 2) });

    Ok(())
}
