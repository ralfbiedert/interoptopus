// Auto-generated plugin interop

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.Marshalling;

namespace {{ namespace }};
{% for composite in composites %}

{{ composite }}
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
