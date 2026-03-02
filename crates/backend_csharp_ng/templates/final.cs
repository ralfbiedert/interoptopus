{{header}}

namespace A {
    public static partial class Interop {

        {% for fn in fn_imports %}
        {{ fn | indent(prefix = "        ") }}
        {% endfor %}

        {{ types | indent }}
    }
}



