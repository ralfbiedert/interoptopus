import ctypes
import example_complex as e


e.init_lib("../../../../target/debug/example_complex")

assert (e.THE_MAGIC_CONSTANT == 666)
assert (e.example_api_version() == 0x00_01_00_00)

# Some data
complex_in = e.SuperComplexEntity()
complex_out = e.SuperComplexEntity()

# This is manual way, you should use `service` pattern instead.
context = ctypes.c_void_p()

# Set value to observe later
complex_in.ammo = 10

# Call APIs
e.example_create_context(context)
e.example_double_super_complex_entity(context, complex_in, complex_out)
e.example_destroy_context(context)

assert (2 * complex_in.ammo == complex_out.ammo)
