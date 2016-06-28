extern crate regex;

use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;


pub struct Unit {
  id: char,
  desc: String,
  base_unit: Option<char>,
  to_base_fn: Box<Fn(f64) -> f64>,
  from_base_fn: Box<Fn(f64) -> f64>
}

impl Unit {
  pub fn new<T:ToString>(id: char, desc: T, base_unit: char, to_base_fn: Box<Fn(f64) -> f64>, from_base_fn: Box<Fn(f64) -> f64>) -> Unit {
    Unit {
      id: id,
      desc: desc.to_string(),
      base_unit: Some(base_unit),
      to_base_fn: to_base_fn,
      from_base_fn: from_base_fn
    }
  }

  pub fn new_base<T:ToString>(id: char, desc: T) -> Unit {
    Unit {
      id: id,
      desc: desc.to_string(),
      base_unit: None,
      to_base_fn: Box::new(|x| x),
      from_base_fn: Box::new(|x| x)
    }
  }

  pub fn get_base_id(&self) -> char {
    match self.base_unit {
      Some(c) => c,
      None => self.id
    }
  }

  pub fn convert_to_base(&self, value: f64) -> f64 {
    (self.to_base_fn)(value)
  }

  pub fn convert_from_base(&self, value: f64) -> f64 {
    (self.from_base_fn)(value)
  }

  pub fn is_compatbile_with(&self, unit: &Unit) -> bool {
    self.get_base_id() == unit.get_base_id()
  }

}

pub struct Units {
  units: HashMap<char, Unit>
}

impl From<Vec<Unit>> for Units {
  fn from(units: Vec<Unit>) -> Units {
    let mut result = HashMap::new();

    for unit in units {
      result.insert(unit.id, unit);
    }

    Units {
      units: result
    }
  }
}

impl Units {

  pub fn create() -> Units {
    let units = vec![
      Unit::new_base('r', "Radiants"),
      Unit::new('d', "Degree", 'r', Box::new(|x| x * 0.017453293), Box::new(|x| x / 0.017453293)),

      Unit::new_base('k', "Kelvin"),
      Unit::new('c', "Celsius", 'k',
        Box::new(|x| x + 273.15),
        Box::new(|x| x - 273.15)
      ),

      Unit::new('f', "Fahrenheit", 'k',
        Box::new(|x| ((x - 32.0) / 1.8) + 273.15),
        Box::new(|x| ((x - 273.15) * 1.8) + 32.0)
      )
    ];

    Units::from(units)
  }

  pub fn get_unit(&self, id: char) -> Result<&Unit, String> {
    self.units.get(&id).ok_or(format!("unit not found: {}", id))
  }

  pub fn convert(&self, value: f64, src_id: char, dst_id: char) -> Result<f64, String> {
    let src_unit = try!( self.get_unit(src_id) );
    let dst_unit = try!( self.get_unit(dst_id) );

    if !src_unit.is_compatbile_with( dst_unit ) {
      return Err(format!("Units not compatible: {} and {}", src_unit.desc, dst_unit.desc));
    }

    let src_base_value = src_unit.convert_to_base(value);

    Ok(dst_unit.convert_from_base(src_base_value))
  }

  pub fn convert_line(&self, line: &Line) -> Result<f64, String> {
    self.convert(line.value, line.src_id, line.dst_id)
  }

}

pub struct Line {
  value: f64,
  src_id: char,
  dst_id: char
}

static LINE_RE: &'static str = r"(?P<number>[-+]?([0-9]*\.[0-9]+|[0-9]+))(?P<from_unit>[a-z])(?P<to_unit>[a-z])";

impl FromStr for Line {
  type Err = String;

  fn from_str(s: &str) -> Result<Line, String> {
    let re = Regex::new(LINE_RE).unwrap();

    let captures = try!( re.captures(s).ok_or(format!("couldn't parse: {}", s)) );

    let value_string = captures.name("number").unwrap();
    let value: f64 = try!( value_string.parse().map_err(|_| "parse bla") );

    let src_string = captures.name("from_unit").unwrap();
    let dst_string = captures.name("to_unit").unwrap();

    let src_id = src_string.chars().next().unwrap();
    let dst_id = dst_string.chars().next().unwrap();

    Ok(Line {
      value: value,
      src_id: src_id,
      dst_id: dst_id
    })
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  fn print_result(line: &Line, value: f64) {
    println!("{}{}", value, line.dst_id);
  }

  #[test]
  fn test_inputs() {
    let units = Units::create();

    let strings = vec![
      "3.1416rd",
      "90dr",
      "212fc",
      "70cf",
      "100cr",
      "315.15kc"
    ];

    for string in strings {
      let line: Line = string.parse().unwrap();
      let result = units.convert_line( &line );
      match result {
        Ok(v) => print_result( &line, v ),
        Err(e) => println!("{}", e)
      };
    }
  }
}
