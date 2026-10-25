//! Reads for imports: a mount serving the `.py` file a host searches for when
//! it answers an `import` call itself.

use monty_types::{ExtFunctionResult, ImportArgs, ModuleRead, MontyPath, OsFunctionCall, find_module_file};

use crate::mount_table::{MountCallOutcome, MountTable};

impl MountTable {
    /// Reads one candidate module file through the mounts, as `Path.read_text`
    /// would: [`ModuleRead::NotHandled`] when no mount covers `path`.
    pub fn read_module(&mut self, path: &str) -> ModuleRead {
        match self.handle_os_call(OsFunctionCall::ReadText(MontyPath::new(path.to_owned()))) {
            MountCallOutcome::Handled(Ok(text)) => ModuleRead::from_result(text.into()),
            MountCallOutcome::Handled(Err(err)) => ModuleRead::from_exception(err.into_exception()),
            MountCallOutcome::NotHandled(_) => ModuleRead::NotHandled,
        }
    }

    /// Searches the mounts for `args`' module file, for a host with no other
    /// handler to consult: `None` when no candidate exists.
    pub fn find_module(&mut self, args: &ImportArgs) -> Option<ExtFunctionResult> {
        find_module_file(args, |path| self.read_module(path))
    }
}
