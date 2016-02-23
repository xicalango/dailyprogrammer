// /r/dailyprogrammer, #255, <weldale@gmail.com>

use std::ops::Range;
use std::path::Path;
use std::fs::File;
use std::io::{
    BufRead,
    BufReader
};

#[derive(Debug)]
pub struct Room {
    num_switches: usize,
    switches: Vec<bool>
}

impl Room {

    pub fn from_str(s: &str) -> Result<Room, String> {
        let mut line_iter = s.lines();
        let size_str = try!(line_iter.next().ok_or("missing # lightswitches"));
        let size = try!(size_str.parse::<usize>().map_err(|err| err.to_string()));

        let mut room = Room::new( size );

        for line in line_iter {
            let range = try!(range_from_str( line ));
            room.toggle_range( range );
        }

        Ok(room)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Room, String> {
        let file = try!(File::open(path).map_err(|err| err.to_string()));
        let reader = BufReader::new(file);
        let mut line_iter = reader.lines();

        let size_str = try!(try!(line_iter.next().ok_or("missing lines")).map_err(|err| err.to_string()));
        let size = try!(size_str.parse::<usize>().map_err(|err| err.to_string()));

        let mut room = Room::new( size );

        for line_res in line_iter {
            let line = try!(line_res.map_err(|err| err.to_string()));
            let range = try!(range_from_str(line));
            room.toggle_range( range );
        }

        Ok(room)
    }

    pub fn new(size: usize) -> Room {
        Room {
            num_switches: size,
            switches: vec![false; size]
        }
    }

    pub fn toggle_range(&mut self, range: Range<usize>) {
        let actual_start = if range.start > range.end { range.end } else { range.start };
        let actual_end = if range.start > range.end { range.start + 1 } else { range.end + 1 };

        let actual_range = Range {
            start: actual_start,
            end: actual_end
        };

        for i in actual_range {
            self.switches[i] = !self.switches[i];
        }
    }

    pub fn get_enabled_light_count(&self) -> usize {
        self.switches.iter().filter( |&&p| p == true ).count()
    }
}

fn range_from_str<T: ToString>(s: T) -> Result<Range<usize>, String> {
    let string: String = s.to_string();
    let v: Vec<&str> = string.trim().split(' ').collect();

    assert_eq!( 2, v.len() );

    let start = try!(v[0].parse::<usize>().map_err(|err| err.to_string()));
    let end = try!(v[1].parse::<usize>().map_err(|err| err.to_string()));

    Ok( Range {
        start: start,
        end: end
    })
}

#[cfg(test)]
mod tests {

    use std::path::Path;
    use dp255::*;

    #[test]
    fn test_simple() {
        let room = Room::from_str("10\n\
3 6\n\
0 4\n\
7 3\n\
9 9").unwrap();

        let e = room.get_enabled_light_count();
        assert_eq!( 7, e );
    }

    #[test]
    fn test_file_1() {
        test_file("data/small.txt", Some(7));
    }

    #[test]
    fn test_file_2() {
        test_file("data/normal.txt", None);
    }

    #[test]
    #[ignore]
    fn test_file_3() {
        test_file("data/lots_of_switches.txt", None);
    }

    fn test_file<P: AsRef<Path>>(path: P, num_sw: Option<usize>) {
        let room = Room::from_path(path).unwrap();

        let light_sw = room.get_enabled_light_count();

        println!("light switches: {}", light_sw );

        if let Some(expected_sw) = num_sw {
            assert_eq!( expected_sw, light_sw );
        }
    }

}
