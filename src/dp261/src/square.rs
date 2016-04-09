
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

pub enum SquareLine {
  Row(usize),
  Col(usize),
  Diag(Diagonal)
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

  pub fn is_magic(&self) -> bool {
    for i in 0..self.a {
      let row_sum = self.row_sum(i);
      let col_sum = self.col_sum(i);
      return_if!(row_sum != 15, false);
      return_if!(col_sum != 15, false);
    }

    {
      let sum = self.diag_sum(&Diagonal::First);
      return_if!(sum != 15, false);
    }

    {
      let sum = self.diag_sum(&Diagonal::Second);
      return_if!(sum != 15, false);
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
    SquareIter {
      square: self,
      start: r * self.a,
      jump: 1,
      i: 0
    }
  }

  pub fn col<'a>(&'a self, c: usize) -> SquareIter<'a> {
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

  fn row_sum(&self, r: usize) -> usize {
    self.row(r).fold(0, |acc, &x| acc + x)
  }

  fn col_sum(&self, c: usize) -> usize {
    self.col(c).fold(0, |acc, &x| acc + x)
  }

  fn diag_sum(&self, d: &Diagonal) -> usize {
    self.diag(d).fold(0, |acc, &x| acc + x)
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
}
