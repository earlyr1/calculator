mod input;
mod maths;

use std::error::Error;
use crate::input::{getCLIArgs, splitToLexem};

fn main() -> Result<(), Box<dyn Error>> {
    let s = getCLIArgs()?;
    let expr = s.splitToLexem()?;
    dbg!(&expr);
    Ok(())
}
