use std::{io::{self, Write}, process::{Command, self}};

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
    },
    command => {
      let command_status = Command::new(command).args(args).output();
      match command_status {
        Ok(output) => {
          if output.status.success() {
            io::stdout().write_all(&output.stdout).unwrap();
          } else {
            io::stderr().write_all(&output.stderr).unwrap();
          }
        },
        Err(err) => println!("fatal: failed to execute command: {}", err),
      }
    }
  }
}

fn main() {
  loop {
    let mut line = String::new();
    print!("% ");
    io::stdout().flush().expect("Failed to flush the stdout buffer!");
    // TODO: handle backspace and other keypresses
    io::stdin().read_line(&mut line).expect("fatal: failed to read from stdin");
    let args = ztesh_split_line(&line);
    ztesh_exec(&args); 
  }
}
