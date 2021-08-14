use anyhow::Result;
use std::io::Read;

const DATA_SIZE: usize = 30000;

#[derive(Debug)]
enum Token {
  IncrDataPtr,
  DecrDataPtr,
  IncrData,
  DecrData,
  OutputByte,
  ReadOneByte,
  JudgeStart(usize), // save peer address
  JudgeEnd(usize),   // save peer address
}

pub struct BrainFuck {
  code: Vec<Token>,
  data: Vec<u8>,
  code_ptr: usize,
  data_ptr: usize,
}

impl BrainFuck {
  pub fn new(code: &str) -> BrainFuck {
    BrainFuck {
      code: Self::gen_token_array(code),
      data: vec![0; DATA_SIZE],
      code_ptr: 0,
      data_ptr: 0,
    }
  }

  fn gen_token_array(code: &str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    for (i, c) in code
      .chars()
      .filter(|&c| {
        c == '>' || c == '<' || c == '+' || c == '-' || c == '.' || c == ',' || c == '[' || c == ']'
      })
      .enumerate()
    {
      match c {
        '>' => result.push(Token::IncrDataPtr),
        '<' => result.push(Token::DecrDataPtr),
        '+' => result.push(Token::IncrData),
        '-' => result.push(Token::DecrData),
        '.' => result.push(Token::OutputByte),
        ',' => result.push(Token::ReadOneByte),
        '[' => {
          result.push(Token::JudgeStart(0));
          stack.push(i);
        }
        ']' => {
          let left = stack.pop().unwrap();
          result[left] = Token::JudgeStart(i);
          result.push(Token::JudgeEnd(left));
        }
        _ => {}
      }
    }
    result
  }

  pub fn exec(&mut self) -> Result<()> {
    while self.code_ptr < self.code.len() {
      let token = &self.code[self.code_ptr];
      match token {
        Token::IncrDataPtr => self.data_ptr += 1,
        Token::DecrDataPtr => self.data_ptr -= 1,
        Token::IncrData => self.data[self.data_ptr] += 1,
        Token::DecrData => self.data[self.data_ptr] -= 1,
        Token::OutputByte => print!("{}", self.data[self.data_ptr]),
        Token::ReadOneByte => {
          let mut buffer = [0];
          std::io::stdin().read_exact(&mut buffer)?;
          self.data[self.data_ptr] = buffer[0];
        }
        Token::JudgeStart(peer) => {
          if self.data[self.data_ptr] == 0 {
            self.code_ptr = *peer;
          }
        }
        Token::JudgeEnd(peer) => {
          if self.data[self.data_ptr] != 0 {
            self.code_ptr = *peer;
          }
        }
      }
      self.code_ptr += 1;
    }

    Ok(())
  }
}
