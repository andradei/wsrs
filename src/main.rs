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
    use cmd::ErrorKind::*;

    match *kind {
        WorkspaceNotFound(ref ws) => eprintln!("workspace {} not found", ws),
        WorkspaceRequired => eprintln!("please provide a workspace"),
        WorkspaceAlreadyExist(ref ws) => eprintln!("workspace {} already exist", ws),
        TooManyArgs => eprintln!("too many arguments, check \"ws help\""),
        DataReadError(e) => eprintln!("{}", e),
    }
    std::process::exit(1);
}
