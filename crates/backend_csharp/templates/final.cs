{{ header }}

{{ using }}

namespace A {
    public static partial class Interop {

        public const string NativeLib = "{{ dll_name }}";

        {% for fn in fn_imports %}
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

    {{ util | indent }}
}



