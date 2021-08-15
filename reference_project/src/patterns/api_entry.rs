use interoptopus::api_entry;

use crate::functions::{ref_mut_option, tupled};

api_entry!(MyAPIv1, pattern_my_api_init_v1, [ref_mut_option, tupled]);

// fn f() {
//     let x = ref_mut_option;
// }
