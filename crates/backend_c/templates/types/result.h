typedef {{ tag_c_type }} {{ tag_name }};
#define {{ ok_variant }} (({{ tag_name }})0)
#define {{ err_variant }} (({{ tag_name }})1)

typedef struct {{ name }}
{
    {{ tag_name }} tag;
    union
    {
        {{ ok_type }} ok;
        {{ err_type }} err;
    };
} {{ name }};
