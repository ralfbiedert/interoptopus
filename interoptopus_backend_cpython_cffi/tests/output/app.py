import reference_project

reference_project.init_api("../../../target/debug/interoptopus_reference_project.dll")


@reference_project.ffi.callback(reference_project.callbacks.fn_u8_rval_u8)
def my_callback(param):
    return param * 3


# Tests some calls to verify basic functionality
assert (reference_project.raw.primitive_i8(5) == 5)
assert (reference_project.raw.callback(my_callback, 33) == 99)


print("Worked")
