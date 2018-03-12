#[macro_use] extern crate serde_derive;

mod cmd;

use cmd::{Command, ErrorKind};

fn main() {
    match Command::new(std::env::args()) {
        Ok(cmd) => {
            if let Err(e) = cmd.run() {
                exit(&e)
            }
        },
        Err(e) => exit(&e),
    }
}

/// Print an error message based on error kind, then end the program.
fn exit(kind: &ErrorKind) -> ! {
    let msg: &'static str;
    match *kind {
        ErrorKind::CommandNotFound => eprintln!("command not found"),
        ErrorKind::WorkspaceNotFound(ref ws) => eprintln!("workspace {} not found", ws),
        ErrorKind::WorkspaceRequired => eprintln!("please provide a workspace"),
        ErrorKind::TooManyArgs => eprintln!("too many arguments, check \"ws help\""),
        ErrorKind::DataReadError(e) => eprintln!("{}", e),
    }
    std::process::exit(1);
}
