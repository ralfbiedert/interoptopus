using My.Company;
using Xunit;

public class TestCoreAlignment
{
    [Fact]
    public void alignment_1()
    {
        var p1 = new Packed1 { x = 12, y = 34 };
        var p2 = Interop.alignment_1(p1);
        Assert.Equal(p1.x, p2.x);
        Assert.Equal(p1.y, p2.y);
    }

    // [Fact]
    // public void boolean_alignment()
    // {
    //     const ulong BIT_PATTERN = 0x5555555555555555;
    //
    //     for (var i = 0; i < 16; i++)
    //     {
    //         var x = new BooleanAlignment { is_valid = true, id = BIT_PATTERN, datum = BIT_PATTERN };
    //
    //         x = Interop.boolean_alignment(x);
    //         Assert.Equal(x.is_valid, false);
    //         Assert.Equal(x.id, BIT_PATTERN);
    //         Assert.Equal(x.datum, BIT_PATTERN);
    //
    //         x = Interop.boolean_alignment(x);
    //         Assert.Equal(x.is_valid, true);
    //         Assert.Equal(x.id, BIT_PATTERN);
    //         Assert.Equal(x.datum, BIT_PATTERN);
    //
    //         x = Interop.boolean_alignment2(true);
    //         Assert.Equal(x.is_valid, true);
    //
    //         x = Interop.boolean_alignment2(false);
    //         Assert.Equal(x.is_valid, false);
    //     }
    // }
    
}