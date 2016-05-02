use std::process::{Stdio, Command, Child, ChildStdin, ChildStdout, ChildStderr};
use ::{Result, Error, ErrorKind};

/// Extension methods for the standard [`Command` type][link] in `std::process`.
///
/// [link]: https://doc.rust-lang.org/std/process/struct.Command.html
pub trait CommandExt {
    fn piped_spawn(&mut self) -> Result<Child>;
}

impl CommandExt for Command {
    fn piped_spawn(&mut self) -> Result<Child> {
        self.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| err.into())
    }
}

/// Extension methods for the standard [`Child` type][link] in `std::process`.
///
/// [link]: https://doc.rust-lang.org/std/process/struct.Child.html
pub trait ChildExt {
    fn map_all_pipes<U, F>(&mut self, op: F) -> Result<U>
        where F: FnOnce(&mut ChildStdin, &mut ChildStdout, &mut ChildStderr) -> Result<U>;
}

impl ChildExt for Child {
    fn map_all_pipes<U, F>(&mut self, op: F) -> Result<U>
        where F: FnOnce(&mut ChildStdin, &mut ChildStdout, &mut ChildStderr) -> Result<U>
    {
        if let Child { stdin: Some(ref mut child_in), stdout: Some(ref mut child_out), stderr: Some(ref mut child_err), .. } = *self {
            op(child_in, child_out, child_err)
        } else {
            Err(Error::new(ErrorKind::Inconsistent, "some pipes were missing from the child process"))
        }
    }
}
