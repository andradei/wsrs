mod cmd;

use cmd::{Command, ErrorKind};

fn main() {
    match Command::new(std::env::args()) {
        Ok(cmd) => {
            if let Err(e) = cmd.run() {
                exit(e)
            }
        },
        Err(e) => exit(e),
    }
}

/// Print an error message based on error kind, then end the program.
fn exit(kind: ErrorKind) -> ! {
    let msg: &'static str;
    match kind {
        ErrorKind::CommandNotFound => {
            msg = "command not found";
        },
        ErrorKind::WorkspaceNotFound => {
            msg = "workspace not found";
        },
        ErrorKind::WorkspaceRequired => {
            msg = "please provide a workspace";
        },
        ErrorKind::TooManyArgs => {
            msg = "too many arguments, check \"ws help\"";
        },
    }
    eprintln!("{}", msg);
    std::process::exit(1);
}
