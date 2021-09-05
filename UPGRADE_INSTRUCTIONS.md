

# Upgrade Instructions

Tips for solving non-trivial breaking changes when upgrading from previous versions.


### 0.9 -> 0.10

- C# backend split into `DotNet` and `Unity`. If methods are missing:
  - Add `.add_overload_writer(DotNet::new())` to `Generator`.
  - Consider adding `.add_overload_writer(Unity::new())` when targeting Unity


### 0.8 -> 0.9

- Replaced most `pattern!` macros with `#[pattern]` attributes, see individual pattern documentation for details.
- Added type hints support, upgraded minimum supported Python version to 3.7 [no workaround]
