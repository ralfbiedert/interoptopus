{{ header }}

{{ using }}

namespace {{ namespace }};

public static partial class Interop {

    public const string NativeLib = "{{ dll_name }}";
{%- if api_guard and api_guard != "" %}

    {{ api_guard | indent }}
{%- endif %}
{%- for fn in fns_rust %}

    {{ fn | indent }}
{%- endfor %}
{%- for fn in fns_overload_simple %}

    {{ fn | indent }}
{%- endfor %}
{%- for fn in fns_overload_body %}

    {{ fn | indent }}
{%- endfor %}
{%- for fn in fns_overload_asynk %}

    {{ fn | indent }}
{%- endfor %}
{%- for field in async_trampoline_fields %}

    {{ field | indent }}
{%- endfor %}
}
{%- for enum in enums %}

{{ enum }}
{%- endfor %}
{%- for composite in composites %}

{{ composite }}
{%- endfor %}
{%- for delegate in delegates %}

{{ delegate }}
{%- endfor %}
{%- for slice in slices %}

{{ slice }}
{%- endfor %}
{%- for vec in vecs %}

{{ vec }}
{%- endfor %}
{%- for service in services %}

{{ service }}
{%- endfor %}
{%- for trampoline in async_trampolines %}

{{ trampoline }}
{%- endfor %}

{%- if pattern_bools and pattern_bools != "" %}

{{ pattern_bools }}
{%- endif %}

{%- if pattern_utf8string and pattern_utf8string != "" %}

{{ pattern_utf8string }}
{%- endif %}

{%- if pattern_wire_buffer and pattern_wire_buffer != "" %}

{{ pattern_wire_buffer }}
{%- endif %}
{%- for wire in wires %}

{{ wire }}
{%- endfor %}

{{ util }}
