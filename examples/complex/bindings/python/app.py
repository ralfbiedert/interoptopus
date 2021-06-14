import example_complex

example_complex.init_api("../../../../target/debug/example_complex")

assert (example_complex.THE_MAGIC_CONSTANT == 666)
assert (example_complex.example_api_version() == 0x00_01_00_00)

# Some data
context = example_complex.ffi.new("Context**")
complex_in = example_complex.ffi.new("SuperComplexEntity[]", 1)
complex_out = example_complex.ffi.new("SuperComplexEntity[]", 1)

# Set value to observe later
complex_in[0].ammo = 10

# Call APIs
example_complex.example_create_context(context)
example_complex.example_double_super_complex_entity(context[0], complex_in, complex_out)
example_complex.example_destroy_context(context)

assert (2 * complex_in[0].ammo == complex_out[0].ammo)

# Make sure we can observe failed FFI
assert (example_complex.example_always_fails() == example_complex.FFIError.NullPointerPassed)
