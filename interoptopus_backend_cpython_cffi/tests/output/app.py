import reference_project
import os

reference_project.init_api("../../../target/debug/interoptopus_reference_project.dll")


@reference_project.ffi.callback(reference_project.callbacks.fn_u8_rval_u8)
def my_callback(param):
    return param * 3


# Tests some calls to verify basic functionality
assert (reference_project.raw.primitive_i8(5) == 5)
assert (reference_project.raw.callback(my_callback, 33) == 99)

# Test class pattern
cls = reference_project.Context(123)
cls.method()
cls.method_success_enum_ok()

try:
    cls.method_success_enum_fail()
    os.abort() # should not reach this line
except BaseException:
    print("Error observed successfully")


# Test singular struct getter and setter
some_vec = reference_project.Vec3f32()
some_vec.x = 1
some_vec.y = 2
some_vec.z = 3

assert (some_vec.x == 1)
assert (some_vec.y == 2)
assert (some_vec.z == 3)

# Test array getters and setters
some_vec_array = reference_project.Vec3f32.array(100)
some_vec_array[50].x = 11
some_vec_array[50].y = 22
some_vec_array[50].z = 33

assert (some_vec_array[50].x == 11)
assert (some_vec_array[50].y == 22)
assert (some_vec_array[50].z == 33)


# Test creation of AsciiStrings
ascii_pattern = reference_project.UseAsciiStringPattern()
ascii_pattern.ascii_string = reference_project.ascii_string(b"aaa")
total_length = reference_project.raw.pattern_ascii_pointer(reference_project.ascii_string(b"bbbbbbb"), ascii_pattern)
assert (total_length == 10)


print("Everything Worked")
