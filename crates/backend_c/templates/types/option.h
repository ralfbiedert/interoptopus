typedef {{ tag_c_type }} {{ tag_name }};
#define {{ name }}_SOME (({{ tag_name }})0)
#define {{ name }}_NONE (({{ tag_name }})1)

typedef struct {{ name }}
{
    {{ tag_name }} tag;
    union
    {
        {{ inner_type }} some;
    };
} {{ name }};
