use std::collections::HashMap;
use std::error::Error;

pub type Expression = Vec::<Lexem>;

pub trait TopElement<T> {
    fn top(&self) -> Option<&T>;
}

impl<T> TopElement<T> for Vec<T> {
    fn top(&self) -> Option<&T> {
        match self.len() {
            0 => None,
            n => Some(&self[n - 1])
        }
    }
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub enum Sign {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    OBr, // open bracket
    CBr, // close bracket
    Umn // unary minus
}

#[derive(Debug)]
pub enum Lexem {
    Sign(Sign),
    F32(f32)
}

impl PartialEq for Lexem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Lexem::Sign(ref a), &Lexem::Sign(ref b)) => a == b,
            (&Lexem::F32(ref a), &Lexem::F32(ref b)) => a == b,
            _ => false
        }
    }
}

pub trait ToSign {
    fn to_sign (&self) -> Result::<Option::<Sign>, Box<dyn Error>>;
}

impl ToSign for char {  // Returns Ok(sign) if valid sign, Ok(None) if valid numerical symbol or err if invalid one
    fn to_sign (&self) -> Result::<Option::<Sign>, Box<dyn Error>> {
        match *self {
            '+' => Ok(Some(Sign::Add)),
            '-' => Ok(Some(Sign::Sub)),
            '*' => Ok(Some(Sign::Mul)),
            '/' => Ok(Some(Sign::Div)),
            '^' => Ok(Some(Sign::Pow)),
            '(' => Ok(Some(Sign::OBr)),
            ')' => Ok(Some(Sign::CBr)),
            '~' => Ok(Some(Sign::Umn)),
            _ => match "1234567890.".contains(*self) {
                true => Ok(None),
                false => Err("Invalid character".into())
            }
        }
    }
}


pub struct ExpressionToPolishTransformer {
    pub priority: HashMap::<Sign, u8>,
    pub expression: Option::<Expression>
}

impl Default for ExpressionToPolishTransformer {
    fn default() -> ExpressionToPolishTransformer {
        ExpressionToPolishTransformer { 
            priority: HashMap::from([
                (Sign::OBr, 5),
                (Sign::CBr, 5),
                (Sign::Umn, 4),
                (Sign::Pow, 3),
                (Sign::Div, 2),
                (Sign::Mul, 2),
                (Sign::Add, 1),
                (Sign::Sub, 1)
            ]), 
            expression: None
        }
    }
}

impl ExpressionToPolishTransformer {
    pub fn convert(&self) -> Result<Expression, Box<dyn Error>> {
        match &(*self).expression {
            None => Err("Calling shit".into()),
            Some(expr) => {
                let mut res = Expression::new();
                let mut stack: Vec::<Sign> = Vec::<Sign>::new();
                for lx in expr {
                    match lx {
                        Lexem::F32(val) => res.push(Lexem::F32(*val)),
                        Lexem::Sign(s) => {
                            match s {
                                Sign::OBr => {},
                                Sign::CBr => {
                                    loop {
                                        match stack.pop() {
                                            None => return Err("Wrong expression".into()),
                                            Some(Sign::OBr) => break,
                                            Some(tmp_sgn_1) => res.push(Lexem::Sign(tmp_sgn_1))
                                        }
                                    }
                                },
                                _ => {
                                    loop {
                                        match stack.top() {
                                            None | Some(Sign::OBr) => break,
                                            Some(tmp_sgn_1) => {
                                                if self.priority[tmp_sgn_1] < self.priority[s] {break}
                                                res.push(Lexem::Sign(stack.pop().unwrap()));
                                            }
                                        }
                                    }                                        
                                }
                            }   
                            if *s != Sign::CBr {stack.push(*s);}
                        }
                    }
                }
                loop {
                    match stack.pop() {
                        None => break,
                        Some(Sign::CBr) => {},
                        Some(sgn) => res.push(Lexem::Sign(sgn))
                    }
                }
                Ok(res)
            }
        }
    }
}    


#[cfg(test)]
mod test_maths {
    use crate::input::SplitToLexem;
    use super::{Sign, Lexem, ToSign, ExpressionToPolishTransformer};

    #[test]
    fn test_lexem_eq() {
        let one = Lexem::Sign(Sign::Add);
        let two = Lexem::Sign(Sign::Add);
        assert_eq!(&one, &two);
        let two = Lexem::Sign(Sign::Pow);
        assert_ne!(&one, &two);
        let two = Lexem::F32(5.7);
        assert_ne!(&one, &two);
    }

    #[test]
    fn test_to_sign() {
        let c = '+';
        assert_eq!(c.to_sign().unwrap().unwrap(), Sign::Add);
        let c = '.';
        assert_eq!(c.to_sign().unwrap(), None);
        let c = 's';
        assert!(c.to_sign().is_err());
    }
    #[test]
    fn test_conversion() {
        let expr = String::from("~4+(5-5^7)*3-5.8+16*2*~3").split_to_lexem().unwrap();
        let mut opn = ExpressionToPolishTransformer {
            ..Default::default()
        };
        //
        opn.expression = Some(expr);
        let pol_expr = opn.convert();
        assert_eq!(pol_expr.unwrap(), vec![
            Lexem::F32(4.0),
            Lexem::Sign(Sign::Umn),
            Lexem::F32(5.0),
            Lexem::F32(5.0),
            Lexem::F32(7.0),
            Lexem::Sign(Sign::Pow),
            Lexem::Sign(Sign::Sub),
            Lexem::F32(3.0),
            Lexem::Sign(Sign::Mul),
            Lexem::Sign(Sign::Add),
            Lexem::F32(5.8),
            Lexem::Sign(Sign::Sub),
            Lexem::F32(16.0),
            Lexem::F32(2.0),
            Lexem::Sign(Sign::Mul),
            Lexem::F32(3.0),
            Lexem::Sign(Sign::Umn),
            Lexem::Sign(Sign::Mul),
            Lexem::Sign(Sign::Add)
        ]);
    }
}
