{{header}}

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.Marshalling;

namespace A {
    public static partial class Interop {

        public const string NativeLib = "{{ dll_name }}";

        {% for fn in fn_imports %}
        {{ fn | indent(prefix = "        ") }}
        {% endfor %}

        {% for enum in enums %}
        {{ enum | indent(prefix = "        ") }}
        {% endfor %}

        {% for composite in composites %}
        {{ composite | indent(prefix = "        ") }}
        {% endfor %}

        {% for delegate in delegates %}
        {{ delegate | indent(prefix = "        ") }}
        {% endfor %}
    }
}



