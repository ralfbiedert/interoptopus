typedef enum {{ tag_name }}
{
    {{ name }}_OK = 0,
    {{ name }}_ERR = 1,
} {{ tag_name }};

typedef struct {{ name }}
{
    {{ tag_name }} tag;
    union
    {
        {{ ok_type }} ok;
        {{ err_type }} err;
    };
} {{ name }};
