{{header}}

namespace A {
    public static partial class Interop {

        public const string NativeLib = "{{ dll_name }}";

        {% for fn in fn_imports %}
        {{ fn | indent(prefix = "        ") }}
        {% endfor %}

        {% for enum in enums %}
        {{ enum | indent(prefix = "        ") }}
        {% endfor %}

        {{ types | indent }}
    }
}



