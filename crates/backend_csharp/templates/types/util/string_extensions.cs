public static class StringExtensions
{
    public static Utf8String Utf8(this string s) { return Utf8String.From(s); }
}
