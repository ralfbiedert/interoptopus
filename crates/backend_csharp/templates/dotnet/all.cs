// Auto-generated plugin interop

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.Marshalling;

namespace {{ namespace }};
{% if pattern_bools %}

{{ pattern_bools }}
{% endif %}
{% for delegate in delegates %}

{{ delegate }}
{% endfor %}
{% for composite in composites %}

{{ composite }}
{% endfor %}
{% for enum in enums %}

{{ enum }}
{% endfor %}
{% if util %}

{{ util }}
{% endif %}
{% if trampoline_class %}

{{ trampoline_class }}
{% endif %}
{% if wire_buffer %}

{{ wire_buffer }}
{% endif %}
{% for wire in wires %}

{{ wire }}
{% endfor %}
{% if plugin_interface %}

{{ plugin_interface }}
{% endif %}
{% for svc_interface in service_interfaces %}

{{ svc_interface }}
{% endfor %}

public static class Interop
{
{% for trampoline in trampolines %}
{{ trampoline }}

{% endfor %}
}
