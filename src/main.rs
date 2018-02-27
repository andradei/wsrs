fn main() {
    let cmd = Command::new(std::env::args());
    if let Err(e) = cmd.run() {

    }
}

/// List of possible errors.
enum ErrorKind {
    CommandNotFound,
    WorkspaceNotFound,
    WorkspaceRequired,
    TooManyArgs,
}

/// Type alias for semantic assistance. It helps convey what the meaning that certain
/// Command variants take a String that represents a workspace (a directory path).
type Workspace = String;

/// Possible commands accepted as input.
#[derive(Debug)]
enum Command {
    Create(Workspace),
    Delete(Workspace),
    List,
    Help,
}

impl Command {
    /// Return a Command variant based on the input argument.
    fn new(mut args: std::env::Args) -> Command {
        // Skip the first argument because it is the fully qualified program name.
        args.next();

        match args.len() {
            0 => Command::Help,
            1 ... 2 => {
                if let Some(cmd) = args.next() {
                    match cmd.as_str() {
                        "list" | "ls" | "l" => Command::List,
                        "help" | "h" => Command::Help,
                        "create" | "c" | "new" | "n" | "insert" | "i" => {
                            if let Some(ws) = args.next() {
                                Command::Create(ws)
                            } else {
                                error(ErrorKind::WorkspaceRequired);
                            }
                        },
                        "delete" | "d" | "remove" | "rm" => {
                            if let Some(ws) = args.next() {
                                Command::Delete(ws)
                            } else {
                                error(ErrorKind::WorkspaceRequired);
                            }
                        },
                        _ => error(ErrorKind::CommandNotFound),
                    }
                } else {
                    error(ErrorKind::CommandNotFound);
                }
            },
            _ => error(ErrorKind::TooManyArgs),
        }
    }

    fn run(self) -> Result<(), ErrorKind> {
        match self {
            Command::Create(ws) => {
                // TODO: Check workspace exists
                // TODO: Create workspace
            },
            Command::Delete(ws) => {
                // TODO: Check workspace exists
                // TODO: Delete workspace
            },
            Command::List => {},
            Command::Help => {
                // TODO: Print help with color
            },
        }
        Ok(())
    }
}

/// Print an error message based on error kind.
fn error(kind: ErrorKind) -> ! {
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
