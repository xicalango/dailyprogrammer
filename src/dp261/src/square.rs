
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

enum Diagonal {
  First,
  Second
}

enum SquareLine {
  Row(usize),
  Col(usize),
  Diag(Diagonal)
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
    let first_sum = self.row_sum(0);

    for i in 1..self.a {
      let sum = self.row_sum(i);
      return_if!(sum != first_sum, false);
    }

    for i in 0..self.a {
      let sum = self.col_sum(i);
      return_if!(sum != first_sum, false);
    }

    {
      let sum = self.diag_sum(&Diagonal::First);
      return_if!(sum != first_sum, false);
    }

    {
      let sum = self.diag_sum(&Diagonal::Second);
      return_if!(sum != first_sum, false);
    }

    return true;
  }

  fn sum_of(&self, line: &SquareLine) -> usize {
    match line {
      &SquareLine::Row(r) => self.row_sum(r),
      &SquareLine::Col(c) => self.col_sum(c),
      &SquareLine::Diag(ref d) => self.diag_sum(d)
    }
  }

  fn row_sum(&self, r: usize) -> usize {
    let mut sum = 0;

    let start = r * self.a;

    for i in 0..self.a {
      sum = sum + self.data[start + i];
    }

    sum
  }

  fn col_sum(&self, u: usize) -> usize {
    let mut sum = 0;

    for i in 0..self.a {
      sum = sum + self.data[u + self.a * i];
    }

    sum
  }

  fn diag_sum(&self, d: &Diagonal) -> usize {
    match d {
      &Diagonal::First => self.diag_sum_first(),
      &Diagonal::Second => self.diag_sum_second()
    }
  }

  fn diag_sum_first(&self) -> usize {
    let mut sum = 0;

    let jump = self.a + 1;

    for i in 0..self.a {
      sum = sum + self.data[i * jump];
    }

    sum
  }

  fn diag_sum_second(&self) -> usize {
    let mut sum = 0;

    let start = self.a - 1;

    for i in 0..self.a {
      let index = start + i * start;
      println!("diag2: i: {} index: {} data: {}", i, index, self.data[index]);
      sum = sum + self.data[index]
    }

    sum
  }


}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  #[should_panic]
  fn test_non_square() {
    Square::new(vec![1, 2]);
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
