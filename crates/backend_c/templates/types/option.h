typedef {{ tag_c_type }} {{ tag_name }};
#define {{ some_variant }} (({{ tag_name }})0)
#define {{ none_variant }} (({{ tag_name }})1)

typedef struct {{ name }}
{
    {{ tag_name }} tag;
    union
    {
        {{ inner_type }} some;
    };
} {{ name }};
