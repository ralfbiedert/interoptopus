{{ header }}

{{ using }}

namespace My.Company {
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
{%- for service in services %}

    {{ service | indent }}
{%- endfor %}
{%- for trampoline in async_trampolines %}

    {{ trampoline | indent }}
{%- endfor %}

    {{ util | indent }}
}
