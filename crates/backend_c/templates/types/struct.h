typedef struct {{ name }}
{
{%- for field in fields %}
{%- if field.array_len %}
    {{ field.type_name }} {{ field.name }}[{{ field.array_len }}];
{%- else %}
    {{ field.type_name }} {{ field.name }};
{%- endif %}
{%- endfor %}
} {{ name }};
