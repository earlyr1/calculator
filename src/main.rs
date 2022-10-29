#![deny(warnings)]

mod input;
mod maths;

use std::error::Error;
use crate::input::{get_cliargs, SplitToLexem};
use crate::maths::ExpressionToPolishTransformer;

fn main() -> Result<(), Box<dyn Error>> {
    let s = get_cliargs()?;
    let expr = s.split_to_lexem()?;
    let opn = ExpressionToPolishTransformer {
        expression: Some(expr),
        ..Default::default()
    };
    let res = opn.convert();
    dbg!(&res);
    Ok(())
}
