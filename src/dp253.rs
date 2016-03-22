
use std::str::FromStr;

#[derive(Debug)]
pub enum Mode {
    Insert,
    Overwrite
}

#[derive(Debug)]
pub enum Command {
    Nop,
    ClearScreen,
    Home,
    CarriageReturn,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    DelRight,
    ChangeMode(Mode),
    WriteChar(char),
    Move(u8, u8)
}

pub fn parse_commands(s: &str) -> Result<Vec<Command>, String> {

  let mut cmds: Vec<Command> = Vec::new();

  for line in s.lines() {
    let chars: Vec<char> = line.trim().chars().collect();

    if chars.len() == 0 {
        continue;
    }

    let mut index = 0;

    while index < chars.len() {

      if chars[index] != '^' {
        cmds.push( Command::WriteChar(chars[index]) );
      } else {

        index = index + 1;

        let cmd = match chars[index] {
          'c' => Command::ClearScreen,
          'h' => Command::Home,
          'b' => Command::CarriageReturn,
          'd' => Command::MoveDown,
          'u' => Command::MoveUp,
          'l' => Command::MoveLeft,
          'r' => Command::MoveRight,
          'e' => Command::DelRight,
          'i' => Command::ChangeMode(Mode::Insert),
          'o' => Command::ChangeMode(Mode::Overwrite),
          '^' => Command::WriteChar('^'),
          c => return Err(format!("Unknown control char: {}", c))
        };

        cmds.push( cmd );

      }

      index = index + 1;
    }
  }


  Ok( cmds )
}

pub trait CommandInterpreter {
    fn eval(&mut self, cmd: &Command);

    fn eval_all<'a>(&mut self, cmds: &'a Vec<Command>) {
        for cmd in cmds {
            self.eval(cmd);
        }
    }
}

#[derive(Debug)]
pub struct Buffer {
    width: u8,
    height: u8,
    cur_x: u8,
    cur_y: u8,
    buf: Vec<char>
}

impl Buffer {
  pub fn new(width: u8, height: u8) -> Buffer {
      Buffer {
          width: width,
          height: height,
          cur_x: 0,
          cur_y: 0,
          buf: vec![' '; (width * height) as usize]
      }
  }

  pub fn default_buffer() -> Buffer {
      Buffer::new(10, 10)
  }

  fn assert_in_bounds(&self, x: u8, y: u8) {
    if x >= self.width || y >= self.height {
      panic!(format!("Out of bounds: {}, {}", x, y));
    }
  }

  fn pos_to_index(&self, x: u8, y: u8) -> usize {
    self.assert_in_bounds(x, y);

    x as usize + y as usize * self.width as usize
  }

  pub fn get_char_at(&self, x: u8, y: u8) -> char {
    let index = self.pos_to_index(x, y);

    self.buf[index]
  }

  pub fn set_pos(&mut self, x: u8, y: u8) {
    self.assert_in_bounds(x, y);

    self.cur_x = x;
    self.cur_y = y;
  }

  pub fn set_x(&mut self, x: u8) {
    self.assert_in_bounds(x, self.cur_y);

    self.cur_x = x;
  }

  pub fn set_y(&mut self, y: u8) {
    self.assert_in_bounds(self.cur_x, y);

    self.cur_y = y;
  }

  pub fn move_cur(&mut self, x: i8, y: i8) {
    let new_x: u8 = (self.cur_x as i16 + x as i16) as u8;
    let new_y: u8 = (self.cur_y as i16 + y as i16) as u8;

    self.assert_in_bounds(new_x, new_y);

    self.cur_x = new_x;
    self.cur_y = new_y;
  }

  pub fn put_char(&mut self, c: char) {
    let index = self.pos_to_index(self.cur_x, self.cur_y);

    self.buf[index] = c;
  }

  pub fn write_char(&mut self, c: char) {
    self.put_char(c);

    self.cur_x = self.cur_x + 1;

    if self.cur_x >= self.width {
      self.cur_x = 0;
      self.cur_y = self.cur_y + 1;

      if self.cur_y >= self.height {
        self.cur_y = 0;
      }
    }
  }

  pub fn cls(&mut self) {
    self.buf = vec![' '; (self.width * self.height) as usize]
  }

  pub fn render(&self) {
    for y in 0..self.height {
      for x in 0..self.width {
        let c = self.get_char_at(x, y);
        print!("{}", c);
      }
      println!("");
    }
  }
}

impl CommandInterpreter for Buffer {
    fn eval(&mut self, cmd: &Command) {
        match *cmd {
          Command::WriteChar(c) => self.write_char(c),
          Command::ClearScreen => self.cls(),
          Command::Home => self.set_pos(0, 0),
          Command::CarriageReturn => self.set_x(0),
            _ => unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use dp253::*;

    #[test]
    fn test_create() {
      let mut b = Buffer::default_buffer();

      println!("{:?}", b);

      let cmds: Vec<Command> = parse_commands("Mein name ist Alex^h^cD").unwrap();

      println!("{:?}", cmds);

      b.eval_all(&cmds);

      b.render();
    }



}

