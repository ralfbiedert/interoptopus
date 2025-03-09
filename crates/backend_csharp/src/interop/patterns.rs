pub mod abi_guard;
pub mod callbacks;
pub mod options;
pub mod results;
pub mod services;
pub mod slices;

use crate::Interop;
use crate::interop::patterns::services::write_pattern_service;
use interoptopus::Error;
use interoptopus::patterns::LibraryPattern;
use interoptopus::writer::IndentWriter;

pub fn write_patterns(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for pattern in i.inventory.patterns() {
        match pattern {
            LibraryPattern::Service(cls) => {
                if i.should_emit_by_meta(cls.the_type().meta()) {
                    write_pattern_service(i, w, cls)?;
                }
            }
            LibraryPattern::Builtins(_) => {}
            _ => panic!("Pattern not explicitly handled"),
        }
    }

    Ok(())
}
