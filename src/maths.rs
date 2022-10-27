use std::error::Error;

pub type Expression = Vec::<Lexem>;

#[derive(Debug)]
pub enum Sign {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    OBr, // open bracket
    CBr, // close bracket
}

#[derive(Debug)]
pub enum Lexem {
    Sign(Sign),
    F32(f32)
}

pub trait toSign {
    fn toSign (&self) -> Result::<Option::<Sign>, Box<dyn Error>>;
}

impl toSign for char {
    fn toSign (&self) -> Result::<Option::<Sign>, Box<dyn Error>> {
        match *self {
            '+' => Ok(Some(Sign::Add)),
            '-' => Ok(Some(Sign::Sub)),
            '*' => Ok(Some(Sign::Mul)),
            '/' => Ok(Some(Sign::Div)),
            '^' => Ok(Some(Sign::Pow)),
            '(' => Ok(Some(Sign::OBr)),
            ')' => Ok(Some(Sign::CBr)),
            _ => match "1234567890.".contains(*self) {
                true => Ok(None),
                false => Err("Invalid character".into())
            }
        }
    }
}
