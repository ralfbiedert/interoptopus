use interoptopus::pattern_api_entry;

use crate::functions::{ref_mut_option, tupled};

pattern_api_entry!(MyAPIv1, my_api_init_v1, [ref_mut_option, tupled]);
