
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
    NewLine,
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

    }


    Ok( vec![Command::Nop] )
}

pub trait CommandInterpreter {
    fn eval(&mut self, cmd: &Command);

    fn eval_all<'a, I>(&mut self, cmds: I) 
    where I: Iterator<Item=&'a Command> {
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
}

impl CommandInterpreter for Buffer {
    fn eval(&mut self, cmd: &Command) {
        match cmd {
            _ => panic!()
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

        let cmds: Vec<Command> = parse_commands(" ").unwrap();

        b.eval_all( cmds.iter() );

    }

}

