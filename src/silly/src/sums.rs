
use std::str::FromStr;
use std::fs::File;
use std::path::Path;
use std::io::{
  BufRead,
  BufReader
};

#[derive(Debug,PartialEq,Eq)]
pub struct SchoolData {
  state: String,
  city: String,
  school: String,
  grade: char,
  number: usize
}

impl FromStr for SchoolData {
  type Err = String;

  fn from_str(s: &str) -> Result<SchoolData, String> {
    let split: Vec<&str> = s.split(",").collect();

    assert_eq!(4, split.len());

    let mut grade_nr = split[3].chars();
    let grade = try!( grade_nr.next().ok_or("Grade unparsable") );
    let nr_str = grade_nr.as_str();

    let nr = try!( nr_str.parse().map_err(|_| "Couldn't parse number") );

    Ok( SchoolData {
      state: split[0].to_string(),
      city: split[1].to_string(),
      school: split[2].to_string(),
      grade: grade,
      number: nr
    } )
  }
}

struct CurrentCounter {
  state: String,
  city: String,
  school: String,
  state_count: usize,
  city_count: usize,
  school_count: usize
}

impl CurrentCounter {

  pub fn update_count(&mut self, data: SchoolData) {
    if self.state != data.state {
      println!("School: {}: {}", self.school, self.school_count);
      println!("City: {}: {}", self.city, self.city_count);
      println!("State: {}: {}", self.state, self.state_count);

      self.state = data.state;
      self.state_count = 0;
      self.city_count = 0;
      self.school_count = 0;
    }
    
    if self.city != data.city {
      println!("School: {}: {}", self.school, self.school_count);
      println!("City: {}: {}", self.city, self.city_count);
      self.city = data.city;
      self.city_count = 0;
      self.school_count = 0;
    }
    
    if self.school != data.school {
      println!("School: {}: {}", self.school, self.school_count);
      self.school = data.school;
      self.school_count = 0;
    }

    self.state_count = self.state_count + data.number;
    self.city_count = self.city_count + data.number;
    self.school_count = self.school_count + data.number;
  }

  pub fn print_final_report(&self) {
      println!("School: {}: {}", self.school, self.school_count);
      println!("City: {}: {}", self.city, self.city_count);
      println!("State: {}: {}", self.state, self.state_count);
  }

}

impl From<SchoolData> for CurrentCounter {
  fn from(data: SchoolData) -> CurrentCounter {
    CurrentCounter {
      state: data.state,
      city: data.city,
      school: data.school,
      state_count: data.number,
      school_count: data.number,
      city_count: data.number
    }
  }
}

pub fn count_redhairs<I, S>(iter: I) -> Result<(), String>
where I: Iterator<Item = S>, S: AsRef<str> {
  let mut counter: Option<CurrentCounter> = None;

  for line in iter {
    let school_data: SchoolData = try!( line.as_ref().parse() );

    if let None = counter {
      counter = Some( CurrentCounter::from(school_data) );
      continue;
    }

    counter.as_mut().map(|c| c.update_count( school_data ) );
  }

  counter.map(|c| c.print_final_report() );

  Ok(())
}

pub fn count_redhairs_from_file<P>(path: P) -> Result<(), String>
where P: AsRef<Path> {
  let file = try!(File::open(path).map_err(|_| "error opening file"));
  let reader = BufReader::new(file);

  let lines = reader.lines().map(|l| l.unwrap());

  count_redhairs( lines )
}

#[cfg(test)]
mod tests {

  const PATH: &'static str = "data.csv";

  use super::*;

  #[test]
  fn test_normal_parse() {

    let line = "A,B,C,K100";

    let school: SchoolData = line.parse().unwrap();

    assert_eq!("A".to_string(), school.state);
    assert_eq!("B".to_string(), school.city);
    assert_eq!("C".to_string(), school.school);
    assert_eq!('K', school.grade);
    assert_eq!(100, school.number);
  }

  #[test]
  fn test_count_redhairs() {
    println!("");
    count_redhairs_from_file(PATH).unwrap();
  }

  #[test]
  #[should_panic]
  fn test_unparsable() {
    let line = "A B C K100";

    line.parse::<SchoolData> ().unwrap();
  }

}
