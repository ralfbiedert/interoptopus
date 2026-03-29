+++
title = "Interoptopus"
template = "landing.html"

[extra]
section_order = ["hero", "code_tabs", "features", "final_cta"]

[extra.hero]
badge = "Productive. Performant. Robust."
title = "Interoptopus 🐙"
description = "Focus on the business logic, get Rust interop (almost) for free."
cta_buttons = [
    { text = "Get Started", url = "/docs/introduction", style = "primary" },
    { text = "GitHub", url = "https://github.com/ralfbiedert/interoptopus", style = "secondary" },
]

[extra.code_tabs_section]
title = "Bindings better than hand-written."
description = "Writing good interop code is hard. Writing interop code that's fast and robust and nice to use is extra hard ... or it has been."

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
    { icon = "bolt", title = "Zero Cost", desc = "Simple calls add nanosecond overhead. For complex patterns, the generated marshaling is as fast as anything you would have written yourself (and usually nicer and safer)." },
    { icon = "right-left", title = "Works Both Ways", desc = "Need to expose your Rust library to other languages? Or load legacy code too big to vibe-code into Rust as a plugin? Doesn't matter, it does both." },
    { icon = "layer-group", title = "Expressive", desc = "Go beyond structs and ints. Use services, data enums, callbacks, idiomatic error handling, and transparent async calls from Rust, and in any language that supports it." },
]


[extra.final_cta_section]
title = "Ready to deliver Rust?"
description = ""
button = { text = "Read the Docs", url = "/docs" }
+++

