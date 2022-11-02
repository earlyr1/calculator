#![deny(warnings)]

mod input;
mod maths;
mod exceptions;

use crate::input::{get_cliargs, SplitToLexem};
use crate::maths::ExpressionToPolishTransformer;
use exceptions::Exception;

fn main() -> Result<(), Exception> {
    let s = get_cliargs()?;
    let expr = s.split_to_lexem()?;
    let opn = ExpressionToPolishTransformer {
        expression: Some(expr),
        ..Default::default()
    };
    let res = opn.convert()?;
    dbg!(&res);
    Ok(())
}
