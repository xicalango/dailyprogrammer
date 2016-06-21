
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialEq)]
pub enum StatementType {
  Normal,
  For,
  If
}

#[derive(Debug, PartialEq)]
pub enum IndentChange {
  None,
  Increase,
  Decrease
}

#[derive(Debug)]
pub struct Line {
  line: String,
  indent_change: IndentChange,
  statement_type: StatementType
}

impl Line {

  pub fn parse<T: AsRef<str>> (line_str: T) -> Line {
    let line_str_ref = line_str.as_ref().trim();
    let mut split_line = line_str_ref.split(" ");
    let first_token = split_line.next();

    let (indent_change, statement_type) = match first_token {
      Some("FOR") => (IndentChange::Increase, StatementType::For),
      Some("NEXT") => (IndentChange::Decrease, StatementType::For),
      Some("IF") => (IndentChange::Increase, StatementType::If),
      Some("ENDIF") => (IndentChange::Decrease, StatementType::If),
      _ => (IndentChange::None, StatementType::Normal)
    };

    Line {
      line: line_str_ref.to_string(),
      indent_change: indent_change,
      statement_type: statement_type
    }
  }

}

impl FromStr for Line {
  type Err = String;
  fn from_str(s: &str) -> Result<Line, String> {
    Ok(Line::parse(s))
  }
}

#[derive(Debug)]
pub struct Document {
  lines: Vec<Line>
}

#[derive(Debug)]
struct DocumentRenderer<'a> {
  doc: &'a Document,
  statement_stack: Vec<&'a StatementType>,
  cur_indent: usize,
  indent_str: String
}

impl<'a> DocumentRenderer<'a> {

  pub fn new<T: ToString>(doc: &'a Document, indent_str: T) -> DocumentRenderer<'a> {
    DocumentRenderer {
      doc: doc,
      statement_stack: Vec::new(),
      cur_indent: 0,
      indent_str: indent_str.to_string()
    }
  }

  fn print_indent(&self) {
    for _ in 0..self.cur_indent {
      print!("{}", self.indent_str);
    }
  }

  fn change_indent(&mut self, change: isize) {
    self.cur_indent = (self.cur_indent as isize + change) as usize;
  }

  pub fn render(&mut self) {
    for line in self.doc.lines.iter() {

      match line.indent_change {
        IndentChange::Increase => {
          self.print_indent();
          self.change_indent(1);
          self.statement_stack.push(&line.statement_type);
        },

        IndentChange::Decrease => {
          let statement_type = self.statement_stack.pop().unwrap();
          assert_eq!(statement_type, &line.statement_type);
          self.change_indent(-1);
          self.print_indent();
        },

        IndentChange::None => self.print_indent()
      };

      println!("{}", line.line);
    }
  }

}

impl FromStr for Document {
  type Err = String;

  fn from_str(s: &str) -> Result<Document, String> {
    let parsed_lines = s.split("\n").map(|l| l.parse::<Line>().unwrap()).collect::<Vec<Line>>();
    Ok(Document::new(parsed_lines))
  }
}

impl Document {

  pub fn new(lines: Vec<Line>) -> Document {
    Document {
      lines: lines
    }
  }

  pub fn render(&self) {
    let mut renderer = DocumentRenderer::new(self, "    ");

    renderer.render();
  }

}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_line_parse() {

    let normal_line: Line = "BLA".parse().unwrap();
    let indent_increase_line: Line = "IF blabla".parse().unwrap();
    let indent_decrease_line: Line = "NEXT blubb".parse().unwrap();

    assert_eq!(IndentChange::None, normal_line.indent_change);
    assert_eq!(IndentChange::Increase, indent_increase_line.indent_change);
    assert_eq!(IndentChange::Decrease, indent_decrease_line.indent_change);

    assert_eq!(StatementType::Normal, normal_line.statement_type);
    assert_eq!(StatementType::If, indent_increase_line.statement_type);
    assert_eq!(StatementType::For, indent_decrease_line.statement_type);
  }

  #[test]
  fn test_render_lines() {

    println!("");

    let lines = r#"
VAR I
 FOR I=1 TO 31
        IF !(I MOD 3) THEN
  PRINT "FIZZ"
      ENDIF
            IF !(I MOD 5) THEN
          PRINT "BUZZ"
                ENDIF
        IF (I MOD 3) && (I MOD 5) THEN
      PRINT "FIZZBUZZ"
      ENDIF
         NEXT
"#;

    let document: Document = lines.parse().unwrap();
    document.render();


  }
}
