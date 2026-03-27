typedef enum {{ tag_name }}
{
    {{ ok_variant }} = 0,
    {{ err_variant }} = 1,
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
