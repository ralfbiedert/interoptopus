from cffi import FFI
from reference_project import api_definition

# This just tests the definition
ffibuilder = FFI()
ffibuilder.cdef(api_definition)


print("Worked")
