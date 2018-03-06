extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::{
    Error as IoError,
    Read,
};

use self::serde_json::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Workspace {
    name: String,
    path: String, // might have to be a std::path::Path
}

/// List of possible errors.
pub enum ErrorKind {
    CommandNotFound,
    WorkspaceNotFound,
    WorkspaceRequired,
    TooManyArgs,
    DataReadError(&'static str),
}

/// Possible commands accepted as input.
#[derive(Debug)]
pub enum Command {
    Create(String),
    Delete(String),
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
        if args.next().is_none() {
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

Examples:
    Create workspace
        ws create my_project
    Delete workspace
        ws delete my_project
    Go to workspace
        cd $(ws my_project)
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
                // Get the contents of the data file into a String.
                let path_str = format!("{}/.config/ws/ws.json", env!("HOME"));
                let mut ws_file = File::open(path_str)?;
                let mut content = String::with_capacity(500);
                ws_file.read_to_string(&mut content)?;

                // Try to deserialize the contents of data file.
                if let Ok(workspaces) = serde_json::from_str::<Vec<Workspace>>(&content) {
                    // Print the workspaces.
                    for ws in workspaces {
                        // TODO: Print with color. use termcolor or equivalent.
                        println!("  {}\n    {}", ws.name, ws.path);
                    }
                } else {
                    return Err(ErrorKind::DataReadError("error deserializing workspace"));
                }
            },
            Command::Help => {
                // TODO: Print help with color. Use termcolor of equivalent.
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

impl ::std::convert::From<IoError> for ErrorKind {
    fn from(_: IoError) -> Self {
        ErrorKind::DataReadError("data file couldn't be read or found")
    }
}
