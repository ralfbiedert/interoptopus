#pragma warning disable 0105
using System;
using System.IO;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.ComponentModel;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
#if NET7_0_OR_GREATER
using System.Runtime.InteropServices.Marshalling;
#endif
using System.Runtime.CompilerServices;
{% for extra in extra_imports %}using {{ extra }};
{% endfor %}
#pragma warning restore 0105
