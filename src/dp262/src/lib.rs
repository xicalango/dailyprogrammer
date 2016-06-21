
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Value {
  Number(u64),
  Float(f64),
  String(String),
  Array(Vec<Value>)
}

impl Value {
  pub fn new_string<S: ToString>(s: S) -> Value {
    Value::String(s.to_string())
  }
}

#[derive(Debug, PartialEq)]
pub struct ParseError;

impl From<ParseIntError> for ParseError {
  fn from(_: ParseIntError) -> ParseError {
    ParseError
  }
}

#[derive(Debug, PartialEq)]
pub struct Row {
  values: Vec<Value>
}

impl Row {
  pub fn new(values: Vec<Value>) -> Row {
    Row {
      values: values
    }
  }
}

impl FromStr for Row {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Row, ParseError> {
    let values = s.split("`")
     .map(Value::from_str)
     .map(Result::unwrap)
     .collect();

    Ok(Row::new(values))
  }
}

pub struct RowStore {
  rows: Vec<Row>
}



impl<'a> IntoIterator for &'a Row {
  type Item = &'a Value;
  type IntoIter = std::slice::Iter<'a, Value>;

  fn into_iter(self) -> std::slice::Iter<'a, Value> {
    self.values.iter()
  }
}

impl Value {
  pub fn parse_single_value(s: &str) -> Result<Value, ParseError> {
    if let Ok(num) = s.parse::<u64>() {
      return Ok(Value::Number(num));
    }

    if let Ok(num) = s.parse::<f64>() {
      return Ok(Value::Float(num));
    }

    Ok(Value::String(s.to_string()))
  }
}

impl FromStr for Value {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Value, ParseError> {

    let splitted = s.split(" ");

    let v: Vec<Value> = splitted.map(Value::parse_single_value).map(Result::unwrap).collect();

    if v.len() == 0 {
      Err::<Value, ParseError>(ParseError)
    } else if v.len() == 1 {
      v.into_iter().nth(0).ok_or(ParseError)
    } else {
      Ok(Value::Array(v))
    }
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn tdd1() {
    let val = "".parse::<Value>();

    assert_eq!(Ok(Value::String("".to_string())), val);
  }

  #[test]
  fn tdd2() {
    let val = "123".parse::<Value>();

    assert_eq!(Ok(Value::Number(123)), val)
  }

  #[test]
  fn tdd3() {
    let val = "44.234".parse::<Value>();

    assert_eq!(Ok(Value::Float(44.234)), val)
  }

  #[test]
  fn tdd4() {
    let val = "0x123N".parse::<Value>();

    assert_eq!(Ok(Value::String("0x123N".to_string())), val);
  }

  #[test]
  fn tdd5() {
    let val = "123 456 789".parse::<Value>().unwrap();

    assert_eq!(Value::Array(vec![Value::Number(123), Value::Number(456), Value::Number(789)]), val);
  }

  #[test]
  fn tdd6() {
    let val: Value = "asd fgh jkl".parse().unwrap();

    assert_eq!(Value::Array(vec![
      Value::String("asd".to_string()),
      Value::String("fgh".to_string()),
      Value::String("jkl".to_string())
    ]), val);
  }

  #[test]
  fn tdd7() {
    let row: Row = "2015 4 4`Challenge #`261`Easy".parse().unwrap();

    let row_data: Vec<Value> = vec![
      Value::Array(vec![Value::Number(2015), Value::Number(4), Value::Number(4)]),
      Value::Array(vec![Value::new_string("Challenge"), Value::new_string("#")]),
      Value::Number(261),
      Value::new_string("Easy")
    ];

    let expected = Row::new( row_data );

    assert_eq!(expected, row);
  }
}
