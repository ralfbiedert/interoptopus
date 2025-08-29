mod async_helper;
mod exception;
mod utf8string;

use crate::Interop;
use crate::interop::builtins::async_helper::write_async_helper;
use crate::interop::builtins::exception::write_interop_exception;
use crate::interop::builtins::utf8string::write_utf8_string;
use interoptopus_backend_utils::{Error, IndentWriter};

pub fn write_builtins(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    write_interop_exception(i, w)?;
    write_async_helper(i, w)?;
    write_utf8_string(i, w)?;
    Ok(())
}
