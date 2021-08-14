use anyhow::Result;
use brainfuck1::BrainFuck;

fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("usage: ./brainfuck script.bf");
    std::process::exit(0);
  }
  let code = std::fs::read_to_string(&args[1])?;
  let mut bf = BrainFuck::new(&code);
  bf.exec()?;
  Ok(())
}
