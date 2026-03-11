{{ header }}

{{ using }}

namespace A {
    public static partial class Interop {

        public const string NativeLib = "{{ dll_name }}";

        {% for fn in fns_rust %}
        {{ fn | indent(prefix = "        ") }}
        {% endfor %}

        {% for fn in fns_overload_simple %}
        {{ fn | indent(prefix = "        ") }}
        {% endfor %}

        {% for fn in fns_overload_body %}
        {{ fn | indent(prefix = "        ") }}
        {% endfor %}
    }

    {% for enum in enums %}
    {{ enum | indent }}
    {% endfor %}

    {% for composite in composites %}
    {{ composite | indent }}
    {% endfor %}

    {% for delegate in delegates %}
    {{ delegate | indent }}
    {% endfor %}

    {% for service in services %}
    {{ service | indent }}
    {% endfor %}

    {{ util | indent }}
}



