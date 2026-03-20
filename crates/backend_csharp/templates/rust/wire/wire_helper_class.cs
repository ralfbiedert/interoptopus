public partial class {{ class_name }}
{
{%- for field in field_decls %}
    {{ field }}
{%- endfor %}
}
