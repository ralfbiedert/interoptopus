c_lib = None

def init_lib(path):
    """Initializes the native library. Must be called at least once before anything else."""
    global c_lib
    c_lib = ctypes.cdll.LoadLibrary(path)

    {%- for name, param in functions %}
    c_lib.{{name}}.argtypes = [{{param.signature}}]
    {%- endfor %}

    {%- for name, param in functions %}
    c_lib.{{name}}.restype = {{param.restype}} {#- TODO: ONLY IF NOT EMPTY!! #}
    {%- endfor %}
