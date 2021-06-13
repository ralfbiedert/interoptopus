import example_complex

api = example_complex.api_from_dll("../../../../target/debug/example_complex")

assert(api.THE_MAGIC_CONSTANT == 666)
assert(api.example_api_version() == 0x00_01_00_00)

