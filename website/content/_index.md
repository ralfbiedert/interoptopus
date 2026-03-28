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
title = "Zero-cost. Idiomatic. Decoupled."
description = "Generated bindings are as clean as if you had written them by hand — without the tedium."
features = [
    { icon = "circle-nodes", title = "Zero Cost", desc = "Bindings have nanosecond overhead, and anything that doesn't you'd have a hard time writing it faster." },
    { icon = "shield-halved", title = "Works Both Ways", desc = "Define a Rust library to be used from other languages; or a plugin API to load other languages into Rust." },
    { icon = "cubes", title = "Expressive", desc = "Supports functions, structs, services (classes), data enums, callbacks, transparent async calls, and much more." },
]



[extra.final_cta_section]
title = "Ready to ship your Rust library everywhere?"
description = "One library. Any language. Zero compromises."
button = { text = "Read the Docs", url = "/docs/introduction" }
+++

