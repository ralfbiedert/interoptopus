/// <summary> A Unit type to represent Rust's <c>()</c>. </summary>
public struct Unit : IEquatable<Unit>
{
    /// <summary> The default unit value. </summary>
    public static readonly Unit Default;

    /// <inheritdoc/>
    public bool Equals(Unit other) => true;

    /// <inheritdoc/>
    public override bool Equals(object? obj) => obj is Unit;
    /// <inheritdoc/>
    public override int GetHashCode() => 0;

    /// <inheritdoc/>
    public override string ToString() => "()";
}

/// <summary> A result type to represent Rust's <c>Result&lt;T, E&gt;</c>. </summary>
/// <typeparam name="T"> The <c>Ok</c> type. </typeparam>
/// <typeparam name="TErr"> The <c>Err</c> type. </typeparam>
public interface IResult<out T, out TErr>
{
    /// <summary> Returns <c>true</c> if the result is <c>Ok</c>; otherwise, <c>false</c>. </summary>
    bool IsOk { get; }
    /// <summary> Returns <c>true</c> if the result is <c>Err</c>; otherwise, <c>false</c>. </summary>
    bool IsErr { get; }
    /// <summary> Returns <c>true</c> if the Rust implementation has panicked; otherwise, <c>false</c>. </summary>
    bool IsPanic { get; }
    /// <summary> Returns <c>true</c> if an invalid null was passed; otherwise, <c>false</c>. </summary>
    bool IsNull { get; }

    /// <summary> Unwraps the value of the result. </summary>
    /// <returns> The value, if the result is <see cref="IsOk"/>. </returns>
    /// <exception cref="EnumException{T}"> Thrown if the result is not <c>Ok</c>. </exception>
    T AsOk();
    /// <summary> Unwraps the error of the result. </summary>
    /// <returns> The error, if the result is <see cref="IsErr"/>. </returns>
    /// <exception cref="EnumException{TErr}"> Thrown if the result is not <c>Err</c>. </exception>
    TErr AsErr();

    /// <summary> Returns the exception for the internal result variant. </summary>
    /// <returns> An <see cref="EnumException{T}"/> or <see cref="EnumException{TErr}"/> for <c>Ok</c> / <c>Err</c> state, or an <see cref="EnumException"/> otherwise. </returns>
    Exception ExceptionForVariant();
}
