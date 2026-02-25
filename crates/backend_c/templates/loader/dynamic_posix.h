#else
#include <dlfcn.h>
#include <string.h>
static int {{ load_fn }}(const char* path, {{ api_name }}* api)
{
    void* lib = dlopen(path, RTLD_NOW);
    if (!lib) return -1;
    void* sym;
{%- for fn in functions %}
{%- if fn.separator %}
    /* internal helpers */
{%- endif %}
    sym = dlsym(lib, "{{ fn.name }}");
    if (!sym) return -1;
    memcpy(&api->{{ fn.name }}, &sym, sizeof(sym));
{%- endfor %}
    return 0;
}
#endif
