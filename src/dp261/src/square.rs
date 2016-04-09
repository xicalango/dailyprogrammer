
macro_rules! return_if {
  ($e:expr, $r:expr) => {
    if $e {
      return $r;
    }
  }
}

pub struct Square {
  a: usize,
  data: Vec<usize>
}

pub enum Diagonal {
  First,
  Second
}

impl From<usize> for Diagonal {
  fn from(d: usize) -> Diagonal {
    match d {
      0 => Diagonal::First,
      1 => Diagonal::Second,
      _ => panic!()
    }
  }
}

pub enum SquareLine {
  Row(usize),
  Col(usize),
  Diag(Diagonal)
}

impl SquareLine {
  pub fn sum(&self, square: &Square) -> usize {
    square.sum_of(self)
  }
}

pub struct LineIter {
  a: usize,
  i: usize,
  is_col: bool
}

impl Iterator for LineIter {
  type Item = SquareLine;

  fn next(&mut self) -> Option<SquareLine> {
    if self.i >= self.a + 2 {
      return None;
    }

    if self.i >= self.a {
      let diag = self.i - self.a;
      self.i = self.i + 1;
      return Some(SquareLine::Diag(Diagonal::from(diag)));
    }

    if self.is_col {
      let result = SquareLine::Col(self.i);

      self.is_col = false;
      self.i = self.i + 1;

      return Some(result);
    } else {
      let result = SquareLine::Row(self.i);

      self.is_col = true;

      return Some(result);
    }
  }
}


impl<'a> From<&'a Square> for LineIter {
  fn from(square: &'a Square) -> LineIter {
    LineIter {
      a: square.a,
      i: 0,
      is_col: false
    }
  }
}

pub struct SquareIter<'a> {
  square: &'a Square,
  start: usize,
  jump: usize,
  i: usize
}

impl<'a> Iterator for SquareIter<'a> {
  type Item = &'a usize;

  fn next(&mut self) -> Option<&'a usize> {

    if self.i >= self.square.a {
      return None;
    }

    let value = &self.square.data[self.start + self.i * self.jump];
    self.i = self.i + 1;

    Some(value)
  }
}

impl Square {
  pub fn new(data: Vec<usize>) -> Square {
    let len = data.len();
    let a_candidate = (len as f64).sqrt().floor() as usize;

    assert_eq!(a_candidate * a_candidate, len);

    Square {
      a: a_candidate,
      data: data
    }
  }

  pub fn get_expected_sum(&self) -> usize {
    (self.a * ( (self.a * self.a) + 1)) / 2
  }

  pub fn is_magic(&self) -> bool {
    let expected_sum = self.get_expected_sum();

    for line in LineIter::from(self) {
      let sum = line.sum(self);
      return_if!(sum != expected_sum, false);
    }

    return true;
  }

  pub fn sum_of(&self, line: &SquareLine) -> usize {
    self.get_iter(line).fold(0, |acc, &x| acc + x)
  }

  pub fn get_iter<'a, 'b>(&'a self, line: &'b SquareLine) -> SquareIter<'a> {
    match line {
      &SquareLine::Row(r) => self.row(r),
      &SquareLine::Col(c) => self.col(c),
      &SquareLine::Diag(ref d) => self.diag(d)
    }
  }

  pub fn row<'a>(&'a self, r: usize) -> SquareIter<'a> {
    assert!(r < self.a);

    SquareIter {
      square: self,
      start: r * self.a,
      jump: 1,
      i: 0
    }
  }

  pub fn col<'a>(&'a self, c: usize) -> SquareIter<'a> {
    assert!(c < self.a);

    SquareIter {
      square: self,
      start: c,
      jump: self.a,
      i: 0
    }
  }

  pub fn diag<'a>(&'a self, d: &Diagonal) -> SquareIter<'a> {
    match d {
      &Diagonal::First => SquareIter {
        square: self,
        start: 0,
        jump: self.a + 1,
        i: 0
      },

      &Diagonal::Second => SquareIter {
        square: self,
        start: self.a - 1,
        jump: self.a - 1,
        i: 0
      }
    }
  }
}

#[cfg(test)]
mod test {

  use super::*;

  fn test_square() -> Square {
    Square::new(vec![8, 1, 6, 3, 5, 7, 4, 9, 2])
  }

  #[test]
  #[should_panic]
  fn test_non_square() {
    Square::new(vec![1, 2]);
  }

  #[test]
  fn test_row_iterator() {
    let square = test_square();

    assert_eq!(vec![8, 1, 6], square.row(0).map(|v| v.clone()).collect::<Vec<usize>>());
  }

  #[test]
  fn test_col_iterator() {
    let square = test_square();

    assert_eq!(vec![8, 3, 4], square.col(0).map(|v| v.clone()).collect::<Vec<usize>>());
  }

  #[test]
  fn test_diag_iterator() {
    let square = test_square();

    assert_eq!(vec![8, 5, 2], square.diag(&Diagonal::First).map(|v| v.clone()).collect::<Vec<usize>>());
    assert_eq!(vec![6, 5, 4], square.diag(&Diagonal::Second).map(|v| v.clone()).collect::<Vec<usize>>());
  }

  #[test]
  fn test_first_example() {
    let square = Square::new(vec![8, 1, 6, 3, 5, 7, 4, 9, 2]);

    assert_eq!(true, square.is_magic());
  }

  #[test]
  fn test_second_example() {
    let square = Square::new(vec![2, 7, 6, 9, 5, 1, 4, 3, 8]);

    assert_eq!(true, square.is_magic());
  }

  #[test]
  fn test_third_example() {
    let square = Square::new(vec![3, 5, 7, 8, 1, 6, 4, 9, 2]);

    assert_eq!(false, square.is_magic());
  }

  #[test]
  fn test_fourth_example() {
    let square = Square::new(vec![8, 1, 6, 7, 5, 3, 4, 9, 2]);

    assert_eq!(false, square.is_magic());
  }

  #[test]
  fn test_big() {
    let square = Square::new(vec![25, 13, 1, 19, 7, 16, 9, 22, 15, 3, 12, 5,
            18, 6, 24, 8, 21, 14, 2, 20, 4, 17, 10, 23, 11]);

    assert_eq!(true, square.is_magic());
  }
}
