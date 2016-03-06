
use std::fmt;

pub struct Flat2dArray<T: Copy> {
  data: Vec<T>,
  width: usize,
  height: usize
}

impl<T: Default + Copy> Flat2dArray<T> {
  pub fn new_default(width: usize, height: usize) -> Flat2dArray<T> {
    Flat2dArray {
      data: vec![T::default(); width * height],
      width: width,
      height: height
    }
  }
}

impl<T: Copy> Flat2dArray<T> {
  pub fn new(width: usize, height: usize, value: T) -> Flat2dArray<T> {
    Flat2dArray {
      data: vec![value; width * height],
      width: width,
      height: height
    }
  }

  pub fn new_zero_sized() -> Flat2dArray<T> {
    Flat2dArray {
      data: Vec::new(),
      width: 0,
      height: 0
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

impl<T> From<Vec<Vec<T>>> for Flat2dArray<T>
where T: Copy {
  fn from(vec: Vec<Vec<T>>) -> Flat2dArray<T> {
    let height = vec.len();

    if height == 0 {
      return Flat2dArray::new_zero_sized();
    }

    assert!(height > 0);

    let first_width = vec[0].len();

    if first_width == 0 {
      return Flat2dArray::new_zero_sized();
    }

    assert!(first_width > 0);

    let mut result = Flat2dArray::new( first_width, height, vec[0][0] );

    for y in 0..height {
      let width = vec[y].len();
      assert_eq!(first_width, width);

      for x in 0..width {
        result.set(x, y, vec[y][x]);
      }
    }

    result
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
  let mut array: Flat2dArray<usize> = Flat2dArray::new_default(width, height);

  for y in 0..height {
    for x in 0..width {
      array.set(x, y, x + width * y);
    }
  }

  array
}

fn create_part_oblique<T>(array: &Flat2dArray<T>, start_x: usize, start_y: usize) -> Vec<T>
where T: Copy {

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

pub fn de_oblique_square<T>(vec: &Vec<Vec<T>>) -> Flat2dArray<T>
where T: Copy {
  unimplemented!();
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
    let a2d: Flat2dArray<i32> = Flat2dArray::new_default( 5, 10 );

    assert_eq!( 5, a2d.get_width() );
    assert_eq!( 10, a2d.get_height() );
  }

  #[test]
  #[should_panic]
  fn test_out_of_bounds() {
    let a2d: Flat2dArray<i32> = Flat2dArray::new_default(5, 10);

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

  #[test]
  fn test_from_vec() {
    let mut vec: Vec<Vec<usize>> = Vec::new();
    for y in 0..6 {
      let mut part_vec: Vec<usize> = Vec::new();
      for x in 0..7 {
        part_vec.push(x * y);
      }
      vec.push(part_vec);
    }

    let vec_copy: Vec<Vec<usize>> = vec.to_vec();

    let array = Flat2dArray::from(vec);

    assert_eq!(6, array.get_height());
    assert_eq!(7, array.get_width());

    for y in 0..6 {
      for x in 0..7 {
        assert_eq!(vec_copy[y][x], array.get(x, y));
      }
    }
  }

}
