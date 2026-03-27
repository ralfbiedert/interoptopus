#if defined(_WIN32)
#include <windows.h>
static int {{ load_fn }}(const char* path, {{ api_name }}* api)
{
    int len = MultiByteToWideChar(CP_UTF8, 0, path, -1, NULL, 0);
    if (len <= 0) return -1;
    wchar_t* wpath = (wchar_t*)_alloca(len * sizeof(wchar_t));
    MultiByteToWideChar(CP_UTF8, 0, path, -1, wpath, len);
    HMODULE lib = LoadLibraryW(wpath);
    if (!lib) return -1;
{%- for fn in functions %}
{%- if fn.separator %}
    /* internal helpers */
{%- endif %}
    api->{{ fn.name }} = ({{ fn.rval }} (*)({{ fn.param_types }}))(void*)GetProcAddress(lib, "{{ fn.symbol }}");
    if (!api->{{ fn.name }}) return -1;
{%- endfor %}
    return 0;
}
