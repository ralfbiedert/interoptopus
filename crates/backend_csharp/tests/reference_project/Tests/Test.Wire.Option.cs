using System.Collections.Generic;
using My.Company;
using Xunit;
using Interop = My.Company.Interop;

public class TestWireOption
{
    private static OptionLeaf MakeLeaf(uint? score1, uint? score2, string? name1, string? name2)
    {
        return new OptionLeaf
        {
            tags = new Dictionary<string, string> { { "key", "value" } },
            score_1 = score1,
            score_2 = score2,
            name_1 = name1!,
            name_2 = name2!
        };
    }

    [Fact]
    public void wire_option_1a()
    {
        var leaf = MakeLeaf(10, 20, "hello", "world");
        var root = new OptionRoot
        {
            id = 42,
            middle_1 = new OptionMiddle
            {
                label = "mid",
                leaf_1 = leaf,
                leaf_2 = null
            },
            items =
            [
                leaf
            ],
            more_items =
            [
            ],
            middle_2 = null
        };

        var result = Interop.wire_option_1(root.Wire()).Unwire();
        Assert.Equal(42u, result.id);
        Assert.NotNull(result.middle_1);
        Assert.Equal("mid", result.middle_1!.label);
        Assert.NotNull(result.middle_1.leaf_1);
        Assert.Equal(10u, result.middle_1.leaf_1!.score_1);
        Assert.Equal(20u, result.middle_1.leaf_1.score_2);
        Assert.Equal("hello", result.middle_1.leaf_1.name_1);
        Assert.Equal("world", result.middle_1.leaf_1.name_2);
        Assert.Null(result.middle_1.leaf_2);
        Assert.Null(result.middle_2);
        Assert.Single(result.items);
    }

    [Fact]
    public void wire_option_1b()
    {
        var leaf = MakeLeaf(null, null, null, null);
        var root = new OptionRoot
        {
            id = 1,
            items =
            [
                leaf
            ],
            more_items =
            [
            ],
            middle_1 = null,
            middle_2 = null
        };

        var result = Interop.wire_option_1(root.Wire()).Unwire();
        Assert.Equal(1u, result.id);
        Assert.Null(result.middle_1);
        Assert.Null(result.middle_2);
        Assert.Single(result.items);
        Assert.Null(result.items[0].score_1);
        Assert.Null(result.items[0].score_2);
        Assert.Null(result.items[0].name_1);
        Assert.Null(result.items[0].name_2);
    }

    [Fact]
    public void wire_option_2()
    {
        var root = new OptionRoot
        {
            id = 0,
            items =
            [
                MakeLeaf(10,
                    20,
                    null,
                    null),
                MakeLeaf(null,
                    30,
                    null,
                    null),
                MakeLeaf(5,
                    null,
                    null,
                    null)
            ],
            more_items =
            [
            ],
            middle_1 = null,
            middle_2 = null
        };

        var sum = Interop.wire_option_2(root.Wire());
        Assert.Equal(65u, sum);
    }

    [Fact]
    public void wire_option_3()
    {
        var root = new OptionRoot
        {
            id = 0,
            items =
            [
                MakeLeaf(null,
                    null,
                    "a",
                    null),
                MakeLeaf(null,
                    null,
                    null,
                    null),
                MakeLeaf(null,
                    null,
                    "b",
                    null)
            ],
            more_items =
            [
            ],
            middle_1 = null,
            middle_2 = null
        };

        var count = Interop.wire_option_3(root.Wire());
        Assert.Equal(2u, count);
    }

    [Fact]
    public void wire_option_4a()
    {
        var root = new OptionRoot
        {
            id = 0,
            middle_1 = new OptionMiddle
            {
                label = "x",
                leaf_1 = MakeLeaf(null,
                    99,
                    null,
                    null),
                leaf_2 = null
            },
            items =
            [
            ],
            more_items =
            [
            ],
            middle_2 = null
        };

        var score = Interop.wire_option_4(root.Wire());
        Assert.Equal(99u, score);
    }

    [Fact]
    public void wire_option_4b()
    {
        var root = new OptionRoot
        {
            id = 0,
            items =
            [
            ],
            more_items =
            [
            ],
            middle_1 = null,
            middle_2 = null
        };

        var score = Interop.wire_option_4(root.Wire());
        Assert.Equal(0u, score);
    }
}