[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
internal delegate {{ rval_managed }} {{ name }}({% for arg in args %}{{ arg.managed_type }} {{ arg.name }}{% if not loop.last %}, {% endif %}{% endfor %});
