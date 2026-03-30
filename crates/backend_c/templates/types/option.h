typedef enum {{ tag_name }}
{
    {{ some_variant }} = 0,
    {{ none_variant }} = 1,
} {{ tag_name }};

typedef struct {{ name }}
{
    {{ tag_name }} tag;
    union
    {
        {{ inner_type }} some;
    };
} {{ name }};
