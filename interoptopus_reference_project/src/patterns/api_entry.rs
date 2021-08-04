use interoptopus::pattern_api_entry;

use crate::functions::{ref_mut_option, tupled};

pattern_api_entry!(MyAPIv1, pattern_my_api_init_v1, [ref_mut_option, tupled]);

// struct MyAPIv1 {
//     ref_mut_option: Function,
// }
//

// fn f() {
//     let x = ref_mut_option;
//     let z = tupled;
// }
