{%- if docs %}
{{ docs }}
{%- endif %}
{{ _fns_decorators_all }}
{{ visibility }} static {{ name }} {{ method_name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
    var self = new {{ name }}();
    self._context = Interop.{{ interop_name }}({% for arg in args %}{% if arg.is_ref %}ref {% endif %}{{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %}).AsOk();
    return self;
}
