use std::error::Error;

pub type Expression = Vec::<Lexem>;

#[derive(PartialEq)]
#[derive(Hash)]
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
    fn ToSign (&self) -> Result::<Option::<Sign>, Box<dyn Error>>;
}

impl ToSign for char {  // Returns Ok(sign) if valid sign, Ok(None) if valid numerical symbol or err if invalid one
    fn ToSign (&self) -> Result::<Option::<Sign>, Box<dyn Error>> {
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


struct ExpressionToPolishTransformer {
    priority: std::collections::HashMap<Sign, u8>,
    expression: Expression
}

impl ExpressionToPolishTransformer {
    fn Convert(&self) -> Expression {
        let mut res = Expression::new();
        let mut stack: Vec::<Sign> = Vec::<Sign>::new();
        for lx in &self.expression {
            match lx {
                Lexem::F32(val) => res.push(Lexem::F32(*val)),
                Lexem::Sign(s) => {},
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::maths::{Sign, Lexem};

    use super::ToSign;

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
        assert_eq!(c.ToSign().unwrap().unwrap(), Sign::Add);
        let c = '.';
        assert_eq!(c.ToSign().unwrap(), None);
        let c = 's';
        assert!(c.ToSign().is_err());
    }
}
