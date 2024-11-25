import platform

if platform.uname()[0] == "Linux":
    prefix = "lib"
else:
    prefix = ""


if platform.uname()[0] == "Windows":
    suffix = ".dll"
elif platform.uname()[0] == "Linux":
    suffix = ".so"
else:
    suffix = ""

DLL = "../../../target/debug/" + prefix + "interoptopus_reference_project" + suffix
