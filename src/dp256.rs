
use std::fmt;

pub struct Flat2dArray<T: Default + Copy> {
  data: Vec<T>,
  width: usize,
  height: usize
}

impl<T: Default + Copy> Flat2dArray<T> {
  pub fn new(width: usize, height: usize) -> Flat2dArray<T> {
    Flat2dArray {
      data: vec![T::default(); width * height],
      width: width,
      height: height
    }
  }

  pub fn is_out_of_bounds(&self, x: usize, y: usize) -> bool {
    x >= self.width || y >= self.height
  }

  fn assert_not_out_of_bounds(&self, x: usize, y: usize) {
    if self.is_out_of_bounds(x, y) {
      panic!(format!("Out of bounds: {}, {}", x, y));
    }
  }

  fn get_index(&self, x: usize, y: usize) -> usize {
    self.assert_not_out_of_bounds(x, y);
    x + y * self.width
  }

  pub fn get(&self, x: usize, y: usize) -> T {
    let index = self.get_index(x, y);

    self.data[index]
  }

  pub fn set(&mut self, x: usize, y: usize, val: T) {
    let index = self.get_index(x, y);

    self.data[index] = val;
  }

  pub fn get_width(&self) -> usize {
    self.width
  }

  pub fn get_height(&self) -> usize {
    self.height
  }
}

impl<T> fmt::Display for Flat2dArray<T>
  where T: Default + Copy + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      writeln!(f, "width: {} height: {}", self.width, self.height);

      for y in 0 .. self.height {
        for x in 0 .. self.width {
          write!(f, "{: >3} ", self.get(x, y));
        }
        writeln!(f, "");
      }
      Ok(())
    }
}

pub fn construct_regular_matrix(width: usize, height: usize) -> Flat2dArray<usize> {
  let mut array: Flat2dArray<usize> = Flat2dArray::new(width, height);

  for y in 0..height {
    for x in 0..width {
      array.set(x, y, x + width * y);
    }
  }

  array
}

fn create_part_oblique<T>(array: &Flat2dArray<T>, start_x: usize, start_y: usize) -> Vec<T>
where T: Default + Copy {

  let mut x = start_x;
  let mut y = start_y;

  let mut part_result: Vec<T> = Vec::new();

  while !array.is_out_of_bounds(x, y) {
    part_result.push(array.get(x, y));

    if x == 0 {
      break;
    }

    x = x - 1;
    y = y + 1;
  }

  part_result
}

pub fn oblique<T>(array: &Flat2dArray<T>) -> Vec<Vec<T>>
where T: Default + Copy {
  let mut result: Vec<Vec<T>> = Vec::new();

  for start_x in 0..array.get_width() {
    let part_result = create_part_oblique(array, start_x, 0);
    result.push( part_result );
  }

  for start_y in 1..array.get_height() {
    let part_result = create_part_oblique(array, array.get_width() - 1, start_y);
    result.push( part_result );
  }

  result
}

#[cfg(test)]
mod tests {
  use std::fmt;
  use dp256::*;

  fn print_obliqued_vec<T: fmt::Display>(vv: &Vec<Vec<T>>) {
    println!("");
    for v in vv {
      for value in v {
        print!("{: >3}", value);
      }
      println!("");
    }
  }

  #[test]
  fn test_create() {
    let a2d: Flat2dArray<i32> = Flat2dArray::new( 5, 10 );

    assert_eq!( 5, a2d.get_width() );
    assert_eq!( 10, a2d.get_height() );
  }

  #[test]
  #[should_panic]
  fn test_out_of_bounds() {
    let a2d: Flat2dArray<i32> = Flat2dArray::new(5, 10);

    a2d.get(6, 5);
  }

  #[test]
  fn test_regular_matrix() {
    let a2d = construct_regular_matrix(10, 5);

    for x in 0 .. a2d.get_width() {
      for y in 0 .. a2d.get_height() {
        assert_eq!( x + y * a2d.get_width(), a2d.get(x, y) );
      }
    }

    println!("{}", a2d);
  }

  #[test]
  fn test_oblique() {
    let array = construct_regular_matrix(6, 6);

    let vec = oblique(&array);

    print_obliqued_vec(&vec);
  }

}
