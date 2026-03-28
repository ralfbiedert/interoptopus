+++
title = "Interoptopus"
template = "landing.html"

[extra]
section_order = ["hero", "code_tabs", "features", "final_cta"]

[extra.hero]
badge = "Polyglot FFI for Rust"
title = "Interoptopus 🐙"
description = "Productive. Performant. Robust."
cta_buttons = [
    { text = "Get Started", url = "/docs/introduction", style = "primary" },
    { text = "GitHub", url = "https://github.com/ralfbiedert/interoptopus", style = "secondary" },
]

[extra.code_tabs_section]
title = "Bindings you will like."
description = "Focus on business logic, we do the rest."

[[extra.code_tabs_section.tabs]]
name = "Rust"
code = """
```rust
#[ffi(service)]
pub struct Hello {}

#[ffi]
impl Hello {
    pub fn world() -> Result<Self, Error> { Ok(Self {}) }
}
```
"""

[[extra.code_tabs_section.tabs]]
name = "C#"
code = """
```cs
try
{
    var service = Hello.World();
    service.Dispose();
} 
catch(Exception) { }
```
"""

[[extra.code_tabs_section.tabs]]
name = "Other Languages"
code = """
```python
# Only on Interoptopus <= 0.14 for now, the Python and C backends 
# haven't been ported to 0.16 yet. Help wanted!
service = my_lib.Hello.world()
del service
"""


[extra.easy_command_section]
title = "One attribute. Every language."
description = "Annotate your Rust types and functions with #[ffi], register them in an inventory, and generate bindings."
tabs = [
    { name = "Rust", command = '#[ffi]\npub struct Vec2 { pub x: f32, pub y: f32 }\n\n#[ffi]\npub fn my_function(input: Vec2) -> f32 { input.x }' },
    { name = "Docs", link = "/docs" },
]

[extra.features_section]
title = "More than just performance."
description = "Made to support real-world interop and rich APIs."
features = [
    { icon = "bolt", title = "Zero Cost", desc = "Simple calls add nanosecond overhead. For complex patterns, the generated marshaling is as fast as anything you would have written yourself." },
    { icon = "right-left", title = "Works Both Ways", desc = "Need to expose your Rust library to other languages? Or load legacy code into Rust as a plugin? Doesn't matter, we got you covered." },
    { icon = "layer-group", title = "Expressive", desc = "Go beyond structs and ints. Use services, data enums, callbacks, idiomatic error handling, and transparent async calls from Rust, and in any language that supports it." },
]



[extra.final_cta_section]
title = "Ready to deliver Rust?"
description = "Stop writing glue code. Start shipping."
button = { text = "Read the Docs", url = "/docs/introduction" }
+++

