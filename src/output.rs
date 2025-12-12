use crate::errors::CalculatorError;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Number(f64),
    Boolean(bool)
}

impl Value {
    pub fn as_number(&self) -> Result<f64, CalculatorError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(CalculatorError::TypeError(String::from("Expected number")))
        }
    }

    pub fn as_bool(&self) -> Result<bool, CalculatorError> {
        match self {
            Value::Boolean(b) => Ok(*b),
            Value::Number(n) => {
                if *n != 0.0 {
                    Ok(true)
                } else {
                    Ok(false)       
                }
            }
        }
    }
}