mod input;
mod maths;

use std::error::Error;
use crate::input::{GetCLIArgs, SplitToLexem};

fn main() -> Result<(), Box<dyn Error>> {
    let s = GetCLIArgs()?;
    let expr = s.SplitToLexem()?;
    dbg!(&expr);
    Ok(())
}
