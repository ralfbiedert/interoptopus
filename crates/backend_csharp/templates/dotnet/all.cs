// Auto-generated plugin interop
using System.Runtime.InteropServices;

namespace {{ namespace }};
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
