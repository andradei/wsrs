extern crate colored;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::{
    Error as IoError,
    Read,
    Write,
};

use self::colored::*;

/// Retrieve the program's version from Cargo.toml.
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Represent a workspace entry found in ws.json.
#[derive(Serialize, Deserialize)]
struct Workspace {
    name: String,
    path: String, // might have to be a std::path::Path
}

/// List of possible errors.
pub enum ErrorKind {
    CommandNotFound,
    WorkspaceNotFound(String),
    WorkspaceRequired,
    WorkspaceAlreadyExist(String),
    TooManyArgs,
    DataReadError(&'static str),
}

/// Enable the Try trait to convert ErrorKind to std::io::Error when used to result a Result with ?.
impl ::std::convert::From<IoError> for ErrorKind {
    fn from(_: IoError) -> Self {
        ErrorKind::DataReadError("data file couldn't be read or found")
    }
}

/// Possible commands accepted as input.
#[derive(Debug)]
pub enum Command {
    Create(String),
    Delete(String),
    Goto(String),
    List,
    Help,
    Version,
}

impl Command {
    /// Create a Command variant based on the input argument(s). Return an error when the command
    /// doesn't exist or is not passed valid arguments.
    pub fn new(mut args: ::std::env::Args) -> Result<Command, ErrorKind> {
        // Skip the first argument because it is the fully qualified program name.
        // Also, it must exist otherwise something in the OS is very broken.
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
        match self {
            Command::Create(ws) => {
                let mut workspaces = Self::get_ws_data()?;

                // If workspace with name ws is found, return error.
                if workspaces.iter().any(|w| w.name == ws) {
                    return Err(ErrorKind::WorkspaceAlreadyExist(ws))
                }

                let current_dir = ::std::env::current_dir()?;

                if let Some(path) = current_dir.to_str() {
                    workspaces.push(Workspace{
                        name: ws,
                        path: String::from(path),
                    });

                    Self::save_ws_data(&workspaces)?;
                } else {
                    return Err(ErrorKind::DataReadError("error capturing working directory"))
                }
            },
            Command::Delete(ws) => {
                let mut workspaces = Self::get_ws_data()?;

                // If workspace with name ws isn't found, return error.
                if let Some(i) = workspaces.iter().position(|w| w.name == ws) {
                    workspaces.remove(i);

                    Self::save_ws_data(&workspaces)?;

                    return Ok(())
                }

                return Err(ErrorKind::WorkspaceNotFound(ws))
            },
            Command::Goto(ws) => {
                let workspaces = Self::get_ws_data()?;

                // If workspace with name ws isn't found, return error.
                match workspaces.iter().find(|w| w.name == ws) {
                    Some(x) => println!("{}", x.path),
                    None => return Err(ErrorKind::WorkspaceNotFound(ws)),
                }
            }
            Command::List => {
                // Try to deserialize the contents of data file.
                let workspaces = Self::get_ws_data()?;
                for ws in workspaces {
                    println!("  {}\n    {}", ws.name.yellow(), ws.path.white());
                }
            },
            Command::Help => {
                println!("ws - Directory alias manager ({})", version());
                println!("{} {}", "Isaac Andrade".white(), "<isaac.nic@gmail.com>".cyan());
                println!("\n{}", "Usage:".green());
                println!("    {} <{} [{}]> | <{}>",
                         "ws".yellow(),
                         "command".yellow(),
                         "workspace".yellow(),
                         "workspace".yellow());
                println!("\n{}", "Commands:".green());
                println!("    {} | {} | {}",
                         "list".yellow(),
                         "ls".yellow(),
                         "l".yellow());
                println!("        List all workspaces.");
                println!("    {} | {}",
                         "help".yellow(),
                         "h".yellow());
                println!("        Display this help message. Shortcut 'ws'.");
                println!("    {} | {}",
                         "version".yellow(),
                         "v".yellow());
                println!("        Display ws version.");
                println!("    {} | {} | {} | {} | {} | {} <{}>",
                         "create".yellow(),
                         "c".yellow(),
                         "new".yellow(),
                         "n".yellow(),
                         "insert".yellow(),
                         "i".yellow(),
                         "name".yellow());
                println!("        Create a new workspace with <name>.");
                println!("    {} | {} | {} | {} <{}>",
                         "delete".yellow(),
                         "d".yellow(),
                         "remove".yellow(),
                         "rm".yellow(),
                         "name".yellow());
                println!("        Delete the workspace for <name>.");

                println!("\n{}", "Examples:".green());
                println!("    {}\n        {}",
                         "Create workspace:".yellow(),
                         "ws create my_project".white());
                println!("    {}\n        {}",
                         "Delete workspace:".yellow(),
                         "ws delete my_project".white());
                println!("    {}\n        {}",
                         "Go to workspace:".yellow(),
                         "cd $(ws my_project)".white());
            },
            Command::Version => {
                println!("{}", version());
            },
        }
        Ok(())
    }

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
                // Treat cmd as a workspace to go to.
                _ => {
                    if args.len() > 1 {
                        Err(ErrorKind::TooManyArgs)
                    } else {
                        Ok(Command::Goto(cmd))
                    }
                }
            }
        } else {
            Err(ErrorKind::CommandNotFound)
        }
    }

    /// Convenience function that tries to return the workspaces JSON data file.
    fn get_ws_file() -> Result<File, IoError> {
        use std::fs::OpenOptions;

        let path_str = format!("{}/.config/ws/ws.json", env!("HOME"));
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path_str)
    }

    /// Try to retrieve the JSON workspaces data from the config file. If successful, try to result
    /// a deserialized Vec<Workspace> of them.
    fn get_ws_data() -> Result<Vec<Workspace>, IoError> {
        let mut ws_file = Self::get_ws_file()?;
        // A pre-allocated buffer might improve initial performance by avoiding an allocation.
        let mut content = String::with_capacity(500);

        // Get the contents of the data file into a String.
        ws_file.read_to_string(&mut content)?;

        Ok(serde_json::from_str(&content)?)
    }

    /// Try to serialize `workspaces` and write it to the JSON metadata file.
    fn save_ws_data(workspaces: &[Workspace]) -> Result<(), IoError> {
        let mut ws_file = Self::get_ws_file()?;
        // Try to serialze workspaces.
        let data = serde_json::to_string_pretty(&workspaces)?;

        // Truncate the file before rewriting the new JSON object with a new worskpace.
        ws_file.set_len(0)?;
        // Try to write data to metadata file.
        ws_file.write_all(data.as_ref())
    }
}
