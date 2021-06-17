import reference_project
import os

reference_project.init_api("../../../target/debug/interoptopus_reference_project.dll")


@reference_project.ffi.callback(reference_project.callbacks.fn_u8_rval_u8)
def my_callback(param):
    return param * 3


# Tests some calls to verify basic functionality
assert (reference_project.raw.primitive_i8(5) == 5)
assert (reference_project.raw.callback(my_callback, 33) == 99)

cls = reference_project.Context(123)
cls.pattern_class_method()
cls.pattern_class_method_success_enum_ok()

try:
    cls.pattern_class_method_success_enum_fail()
    os.abort() # should not reach this line
except BaseException:
    print("Error observed successfully")

print("Everything Worked")
