use std::{
  io::{self, Write},
  process::{self, Command},
};
use termion::{event::{Event, Key}, cursor::{Left, Goto}, clear::All};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn ztesh_split_line(line: &str) -> Vec<&str> {
  // TODO: take whole quote string as one, escape quotes
  let args = line.trim().split(" ").collect();
  args
}

fn ztesh_exec(args: &Vec<&str>) {
  let command = args[0];
  let args = &args[1..];
  match command {
    "exit" => {
      process::exit(0);
    }
    command => {
      let command_status = Command::new(command).args(args).spawn();
      match command_status {
        Ok(mut child) => {
          // TODO: handle input, output and errors properly
          child.wait().expect("Could not run command!");
        }
        Err(err) => print!("failed to execute command: {}\r\n", err),
      }
    }
  }
}

fn main() {
  let mut stdout = io::stdout().into_raw_mode().unwrap();
  print!("{}{}", All, Goto(1, 1));

  loop {
    let mut line = String::new();
    print!("% ");
    stdout.flush().unwrap();
    for c in io::stdin().events() {
      let evt = c.unwrap();
      // TODO: refactor
      match evt {
        Event::Key(Key::Backspace) => {
          if line != "" {
            line.pop();
            print!("{} {}", Left(1u16), Left(1u16));
          }
        },
        Event::Key(Key::Char('\n')) => {
          if line != "" {
            let args = ztesh_split_line(&line);
            print!("\r\n");
            ztesh_exec(&args);
            print!("\r");
            break;
          } else {
            print!("\r\n");
            break;
          }
        },
        Event::Key(Key::Char(any_key)) => {
          line = line + &any_key.to_string();
          print!("{}", any_key);
        },
        _ => (),
      }
      stdout.flush().unwrap();
    }
  }
}
