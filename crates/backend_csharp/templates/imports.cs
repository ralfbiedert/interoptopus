#pragma warning disable 0105
using System;
using System.Text;
using System.Threading.Tasks;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.Marshalling;
using System.Runtime.CompilerServices;
{%- for v in namespace_imports %}
using {{v}};
{%- endfor %}
#pragma warning restore 0105
