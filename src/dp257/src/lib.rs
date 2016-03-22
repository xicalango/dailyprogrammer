// dp257.rs <weldale@gmail.com>

extern crate csv;
extern crate chrono;
extern crate rustc_serialize;

use chrono::{
  DateTime,
  UTC,
  TimeZone
};

use std::collections::HashMap;


#[derive(RustcDecodable, Debug)]
struct ParsedPresident {
  name: String,
  birth_date: String,
  birth_place: String,
  death_date: String,
  death_place: String
}

#[derive(Debug)]
pub struct President {
  name: String,
  birth_date: DateTime<UTC>,
  birth_place: String,
  death_date: Option<DateTime<UTC>>,
  death_place: Option<String>
}

#[derive(Debug)]
pub enum EventType {
  Born,
  Died
}

#[derive(Debug)]
pub struct Event<'a, T: 'a> {
  value: &'a T,
  date: DateTime<UTC>,
  event_type: EventType
}

impl<'a, T: 'a> Event<'a, T> {
  pub fn new(value: &'a T, date: DateTime<UTC>, event_type: EventType) -> Event<'a, T> {
    Event {
      value: value,
      date: date,
      event_type: event_type
    }
  }
}

trait IntoEvent : Sized {
  fn into_event<'a>(&'a self, event_type: EventType) -> Option<Event<'a, Self>>;
}

fn replace_abbrev(s: &String) -> String {
  s.replace("July", "Jul").replace("June", "Jun")
}

fn parse_date(s: &String) -> Option<DateTime<UTC>> {
  let trim_s = s.trim().to_string();
  if trim_s.len() == 0 {
    return Option::None;
  }

  let replaced = replace_abbrev(&trim_s);
  let with_time = format!("{} 00:00:00", replaced);

  Some(UTC.datetime_from_str(&with_time, "%b %e %Y %H:%M:%S").unwrap())
}

impl From<ParsedPresident> for President {

  fn from(p: ParsedPresident) -> President {

    let birth_date = parse_date(&p.birth_date).unwrap();
    let death_date = parse_date(&p.death_date);

    let death_place_trim = p.death_place.trim().to_string();

    let death_place = if death_place_trim.len() == 0 {
      Option::None
    } else {
      Option::Some(death_place_trim)
    };

    President {
      name: p.name.trim().to_string(),
      birth_date: birth_date,
      birth_place: p.birth_place.trim().to_string(),
      death_date: death_date,
      death_place: death_place
    }
  }
}

impl IntoEvent for President {
  fn into_event<'a>(&'a self, event_type: EventType) -> Option<Event<'a, President>> {
    match event_type {
      EventType::Born => Some(Event::new(self, self.birth_date, EventType::Born)),
      EventType::Died => self.death_date.map(|date| Event::new(self, date, EventType::Died))
    }
  }
}

pub fn read_csv(file_path: &str) -> Result<Vec<President>, String> {
  let mut rdr = try!(csv::Reader::from_file(file_path).map_err(|e| e.to_string())).has_headers(true);

  let mut result: Vec<President> = Vec::new();

  for record in rdr.decode() {
    let parsed: ParsedPresident = try!(record.map_err(|e| e.to_string()));
    let president = President::from(parsed);
    result.push(president);
  }

  Ok(result)
}

pub fn create_events<'a>(presidents: &'a Vec<President>) -> Vec<Event<'a, President>> {
  let mut result: Vec<Event<'a, President>> = Vec::new();

  for president in presidents {
    result.push( president.into_event(EventType::Born).unwrap() );
    if let Some(event) = president.into_event(EventType::Died) {
      result.push(event);
    }
  }

  result
}

pub fn create_alive_map<'a>(events: &Vec<Event<'a, President>>) -> HashMap<i32, isize> {
  use chrono::Datelike;

  let mut alive_map = HashMap::new();

  let mut count = 0;

  for event in events {

    let year = event.date.year();

    if !alive_map.contains_key( &year ) {
      alive_map.insert(year, 0);
    }

    count = count + match event.event_type {
      EventType::Born => 1,
      EventType::Died => -1
    };

    alive_map.insert(year, count);
  }

  alive_map
}

fn main() {

}

#[cfg(test)]
mod tests {

  use super::*;

  use super::chrono;
  use super::chrono::offset::TimeZone;

  #[test]
  fn test_csv() {
    read_csv("data/presidents.csv");
  }

  #[test]
  fn test_into_events() {
    let presidents = read_csv("data/presidents.csv").unwrap();
    let mut events = create_events(&presidents);

    events.sort_by_key(|e| e.date);

    let alive_map = create_alive_map(&events);

    let mut alive_tuples: Vec<(&i32, &isize)> = alive_map.iter().collect();

    alive_tuples.sort_by_key(|&(_, v)| -v);

    println!("");
    for (k, v) in alive_tuples {
      println!("{}: {}", k, v);
    }

  }

  #[test]
  fn test_parsing() {
    let time = chrono::UTC.datetime_from_str("Jul 22 1732 00:00:00", "%b %e %Y %H:%M:%S").unwrap().date();
    println!("{:?}", time);
  }

}
