{{ header }}

{{ using }}

namespace {{ namespace }} {
    public static partial class Interop {

        public const string NativeLib = "{{ dll_name }}";
{%- for fn in fns_rust %}

        {{ fn | indent(prefix = "        ") }}
{%- endfor %}
{%- for fn in fns_overload_simple %}

        {{ fn | indent(prefix = "        ") }}
{%- endfor %}
{%- for fn in fns_overload_body %}

        {{ fn | indent(prefix = "        ") }}
{%- endfor %}
{%- for fn in fns_overload_asynk %}

        {{ fn | indent(prefix = "        ") }}
{%- endfor %}
{%- for field in async_trampoline_fields %}

        {{ field | indent(prefix = "        ") }}
{%- endfor %}
    }
{%- for enum in enums %}

    {{ enum | indent }}
{%- endfor %}
{%- for composite in composites %}

    {{ composite | indent }}
{%- endfor %}
{%- for delegate in delegates %}

    {{ delegate | indent }}
{%- endfor %}
{%- for slice in slices %}

    {{ slice | indent }}
{%- endfor %}
{%- for vec in vecs %}

    {{ vec | indent }}
{%- endfor %}
{%- for service in services %}

    {{ service | indent }}
{%- endfor %}
{%- for trampoline in async_trampolines %}

    {{ trampoline | indent }}
{%- endfor %}

{%- if pattern_bools and pattern_bools != "" %}

    {{ pattern_bools | indent }}
{%- endif %}

{%- if pattern_utf8string and pattern_utf8string != "" %}

    {{ pattern_utf8string | indent }}
{%- endif %}

{%- if pattern_wire_buffer and pattern_wire_buffer != "" %}

    {{ pattern_wire_buffer | indent }}
{%- endif %}

    {{ util | indent }}
}
