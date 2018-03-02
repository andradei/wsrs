/// List of possible errors.
pub enum ErrorKind {
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
pub enum Command {
    Create(Workspace),
    Delete(Workspace),
    List,
    Help,
    Version,
}

impl Command {
    /// Delegate that returns a command variant or an error when the command doesn't exist or is
    /// not passed valid arguments.
    fn find_cmd(mut args: ::std::env::Args) -> Result<Command, ErrorKind> {
        if let Some(cmd) = args.next() {
            match cmd.as_str() {
                "list" | "ls" | "l" => Ok(Command::List),
                "help" | "h" => Ok(Command::Help),
                "version" | "v" => Ok(Command::Version),
                "create" | "c" | "new" | "n" | "insert" | "i" => {
                    if let Some(ws) = args.next() {
                        Ok(Command::Create(ws))
                    } else {
                        Err(ErrorKind::WorkspaceRequired)
                    }
                },
                "delete" | "d" | "remove" | "rm" => {
                    if let Some(ws) = args.next() {
                        Ok(Command::Delete(ws))
                    } else {
                        Err(ErrorKind::WorkspaceRequired)
                    }
                },
                _ => Err(ErrorKind::CommandNotFound),
            }
        } else {
            Err(ErrorKind::CommandNotFound)
        }
    }

    /// Create a Command variant based on the input argument(s). Return an error when the command
    /// doesn't exist or is not passed valid arguments.
    pub fn new(mut args: ::std::env::Args) -> Result<Command, ErrorKind> {
        // Skip the first argument because it is the fully qualified program name.
        if let None = args.next() {
            return Err(ErrorKind::CommandNotFound)
        }

        match args.len() {
            // Show help message as a conviniente to the user. Same as typing `ws help`.
            0 => Ok(Command::Help),
            1 ... 2 => {
                Self::find_cmd(args)
            },
            _ => Err(ErrorKind::TooManyArgs),
        }
    }

    pub fn run(self) -> Result<(), ErrorKind> {
        let help_msg: &str = &format!("
ws {}
Isaac Andrade <isaac.nic@gmail.com>

ws is a workspace list manager.

Usage:
    ws <command> [workspace] | <workspace>

Commands:
    list | ls | l
        List all workspaces.
    help | h
        Display this help message. Shortcut 'ws'.
    version | v
        Display ws version.
    create | c | new | n | insert | i <name>
        Create a new workspace with <name>.
    delete | d | remove | rm <name>
        Delete the workspace for <name>.
        ", version());

        match self {
            Command::Create(ws) => {
                println!("{}", ws);
                // TODO: Check workspace exists
                // TODO: Create workspace
            },
            Command::Delete(ws) => {
                println!("{}", ws);
                // TODO: Check workspace exists
                // TODO: Delete workspace
            },
            Command::List => {
                println!("list");
            },
            Command::Help => {
                // TODO: Print help with color
                println!("{}", help_msg);
            },
            Command::Version => {
                println!("{}", version());
            }
        }
        Ok(())
    }
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
