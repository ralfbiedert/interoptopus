return 0
{%- for field in fields %}
    + {% include "wire/size_calculation.cs" %}
{%- endfor %}
;
