use std::env;
use std::io;
use crate::maths::{Sign, Expression, Lexem, ToSign};
use crate::exceptions::Exception;


pub fn get_cliargs () -> Result::<String, Exception> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut res = args[1..].join("");
        res.retain(|c| !c.is_whitespace());
        return Ok(res)
    }
    let mut res = String::new();
    match io::stdin().read_line(&mut res) {
        Ok(_) => {},
        Err(_) => return Err(Exception::IOError)
    };
    res.retain(|c| !c.is_whitespace());
    Ok(res)
}



pub trait SplitToLexem {
    fn split_to_lexem (&self) -> Result::<Expression, Exception>;
}

impl SplitToLexem for String {
    fn split_to_lexem (&self) -> Result::<Expression, Exception> {
        let mut res = Expression::new();
        let mut tmp = String::new();
        for chr in self.chars() {
            match chr.to_sign() {
                Err(e) => return Err(e),
                Ok(sym) => match sym {
                    None => tmp.push(chr),
                    Some(mut sign) => {
                        if tmp.len() == 0 {
                            if sign == Sign::Sub {sign = Sign::Umn}
                            res.push(Lexem::Sign(sign));
                            continue;
                        }
                        match tmp.parse::<f32>() {
                            Ok(num) => {
                                res.push(Lexem::F32(num));
                                res.push(Lexem::Sign(sign));
                                tmp.clear();
                            },
                            Err(_) => return Err(Exception::WrongNumberFormat)
                        };
                    }
                }
            }
        }
        if tmp.len() != 0 {
            match tmp.parse::<f32>() {
                Ok(num) => {
                    res.push(Lexem::F32(num));
                },
                Err(_) => return Err(Exception::WrongNumberFormat)
            };
        }
        Ok(res)
    }
}


#[cfg(test)]
mod test_input {
    use crate::maths::{Lexem, Sign};
    use assert::equal;
    use super::SplitToLexem;
    #[test]
    fn test_split_to_lexem() {
        let s = String::from("3+(4.8-5^2.7)");
        match s.split_to_lexem() {
            Ok(e) => equal(e, vec![
                Lexem::F32(3.),
                Lexem::Sign(Sign::Add),
                Lexem::Sign(Sign::OBr),
                Lexem::F32(4.8),
                Lexem::Sign(Sign::Sub),
                Lexem::F32(5.),
                Lexem::Sign(Sign::Pow),
                Lexem::F32(2.7),
                Lexem::Sign(Sign::CBr)
            ]),
            Err(_) => panic!("Test failed")
        }
    }
}