use std::env;
use std::error::Error;
use std::io;

use crate::maths::{Expression, Sign, Lexem, toSign};


pub fn getCLIArgs () -> Result::<String, Box<dyn Error>> {
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



pub trait splitToLexem {
    fn splitToLexem (&self) -> Result::<Expression, Box<dyn Error>>;
}

impl splitToLexem for String {
    fn splitToLexem (&self) -> Result::<Expression, Box<dyn Error>> {
        let mut res = Expression::new();
        let mut tmp = String::new();
        for chr in self.chars() {
            let sym = chr.toSign()?;
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
mod tests {
    #[test]
    fn test_splitToLexem() {
        let test_string = String::From("3 +2-(6^3-85)");
        let test_res = test_string.splitToLexem();
        assert_eq!(test_res, vec![
            Lexem::F32(3.), 
            Lexem::Sign::Add, 
            Lexem::F32(2.), 
            Lexem::Sign::Sub, 
            Lexem::Sign::OBr,
            Lexem::F32(6.),
            Lexem::Sign::Pow,
            Lexem::F32(3.),
            Lexem::Sign::Sub,
            Lexem::F32(85),
            Lexem::Sign::CBr
        ]);
    }
}
