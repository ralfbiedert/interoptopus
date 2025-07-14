#[ffi_type(wired)]
struct T{x: X, y: Y}

X must be wired
Y must be wired
But we don't know!

basically every type inside that is not a builtin we need to add to "emit list"
and then check that all items that are there by name only (no impl) also have an impl
(referring X but not having a (wired)X anywhere is an error)
