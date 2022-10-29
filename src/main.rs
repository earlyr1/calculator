mod input;
mod maths;

use std::error::Error;
use crate::input::{GetCLIArgs, SplitToLexem};
use crate::maths::ExpressionToPolishTransformer;

fn main() -> Result<(), Box<dyn Error>> {
    let s = GetCLIArgs()?;
    let expr = s.SplitToLexem()?;
    let opn = ExpressionToPolishTransformer {
        expression: Some(expr),
        ..Default::default()
    };
    let res = opn.Convert();
    dbg!(&res);
    Ok(())
}
