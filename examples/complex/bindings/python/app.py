import example_complex as e
from example_complex import api

e.init_api("../../../../target/debug/example_complex")

assert (e.THE_MAGIC_CONSTANT == 666)
assert (api.example_api_version() == 0x00_01_00_00)

# Some data
complex_in = e.SuperComplexEntity()
complex_out = e.SuperComplexEntity()

# This is manual way, you should use `service` pattern instead.
context = e.ffi.new("cffi_context**")

# Set value to observe later
complex_in.ammo = 10

# Call APIs
api.example_create_context(context)
api.example_double_super_complex_entity(context[0], complex_in, complex_out)
api.example_destroy_context(context)

assert (2 * complex_in.ammo == complex_out.ammo)
assert (api.example_always_fails() == e.FFIError.NullPointerPassed)
