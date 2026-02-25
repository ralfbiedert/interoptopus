typedef {{ rval }} (*{{ fn_typedef }})({{ params }});

typedef struct {{ name }}
{
    {{ fn_typedef }} callback;
    const void* data;
    void (*destructor)(const void*);
} {{ name }};
