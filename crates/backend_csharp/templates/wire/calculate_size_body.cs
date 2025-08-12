int size = 0;

{%- for field in fields %}
size += {% include "wire/size_calculation.cs" %};
{%- endfor %}

return size;
