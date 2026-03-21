use interoptopus::wire::Wire;
use std::collections::HashMap;

interoptopus::plugin!(Wired {
    fn wire_string(nested: Wire<String>) -> Wire<String>;
    fn wire_hashmap_string(nested: Wire<HashMap<String, String>>) -> Wire<HashMap<String, String>>;
});
