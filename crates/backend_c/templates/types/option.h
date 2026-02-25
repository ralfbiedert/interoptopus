typedef enum {{ tag_name }}
{
    {{ name }}_SOME = 0,
    {{ name }}_NONE = 1,
} {{ tag_name }};

typedef struct {{ name }}
{
    {{ tag_name }} tag;
    union
    {
        {{ inner_type }} some;
    };
} {{ name }};
