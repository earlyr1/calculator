use std::env;
use std::error::Error;
use std::io;

use crate::maths::{Expression, Lexem, ToSign};


pub fn GetCLIArgs () -> Result::<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut res = args[1..].join("");
        res.retain(|c| !c.is_whitespace());
        return Ok(res)
    }
    let mut res = String::new();
    io::stdin().read_line(&mut res)?;
    res.retain(|c| !c.is_whitespace());
    Ok(res)
}



pub trait SplitToLexem {
    fn SplitToLexem (&self) -> Result::<Expression, Box<dyn Error>>;
}

impl SplitToLexem for String {
    fn SplitToLexem (&self) -> Result::<Expression, Box<dyn Error>> {
        let mut res = Expression::new();
        let mut tmp = String::new();
        for chr in self.chars() {
            let sym = chr.ToSign()?;
            match sym {
                None => tmp.push(chr),
                Some(sign) => {
                    if tmp.len() == 0 { 
                        res.push(Lexem::Sign(sign));
                        continue;
                    }
                    match tmp.parse::<f32>() {
                        Ok(num) => {
                            res.push(Lexem::F32(num));
                            res.push(Lexem::Sign(sign));
                            tmp.clear();
                        },
                        Err(err) => return Err(Box::new(err))
                    };
                }
            }
        }
        if tmp.len() != 0 {
            match tmp.parse::<f32>() {
                Ok(num) => {
                    res.push(Lexem::F32(num));
                },
                Err(err) => return Err(Box::new(err))
            };
        }
        Ok(res)
    }
}


#[cfg(test)]
mod test_input {
    use crate::maths::{Expression, Lexem, Sign};
    use assert::equal;
    use super::SplitToLexem;
    #[test]
    fn test_split_to_lexem() {
        let s = String::from("3+(4.8-5^2.7)");
        let e: Expression = s.SplitToLexem().unwrap();
        equal(e, vec![
            Lexem::F32(3.),
            Lexem::Sign(Sign::Add),
            Lexem::Sign(Sign::OBr),
            Lexem::F32(4.8),
            Lexem::Sign(Sign::Sub),
            Lexem::F32(5.),
            Lexem::Sign(Sign::Pow),
            Lexem::F32(2.7),
            Lexem::Sign(Sign::CBr)
        ])
    }
}