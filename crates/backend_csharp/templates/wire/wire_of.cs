/// <summary>
/// FFI-safe wire representation for {{type}}.
/// Mirrors the Rust Wire&lt;{{type}}&gt; structure layout.
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public unsafe struct WireOf{{type}}
{
    /// <summary>Pointer to buffer data</summary>
    public byte* Data;

    /// <summary>Length of valid data in buffer</summary>
    public int Length;

    /// <summary>
    /// Encoded capacity: 0=borrowed, >0=Rust-allocated, <0=C#-allocated (abs value = actual capacity)
    /// </summary>
    public int Capacity;

    /// <summary>
    /// Create a Wire from a managed {{type}} object with owned buffer
    /// </summary>
    public static WireOf{{type}} From({{type}} value)
    {
        var size = value.CalculateSize();
        var buffer = Marshal.AllocHGlobal(size);
        var wire = new WireOf{{type}}
        {
            Data = (byte*)buffer,
            Length = (int)size,
            Capacity = -(int)size
        };

        try
        {
            value.Serialize(wire.Writer());
            return wire;
        }
        catch
        {
            Marshal.FreeHGlobal(buffer);
            throw;
        }
    }

    /// <summary>
    /// Create a Wire from a managed {{type}} object using provided buffer
    /// </summary>
    public static WireOf{{type}} From({{type}} value, byte* buffer, int bufferSize)
    {
        var size = value.CalculateSize();
        if (size > bufferSize)
            throw new ArgumentException($"Buffer size {bufferSize} is too small for data size {size} when serializing {{type}}");

        var wire = new WireOf{{type}}
        {
            Data = buffer,
            Length = (int)size,
            Capacity = 0 // Indicates borrowed buffer
        };

        value.Serialize(wire.Writer());
        return wire;
    }

    public BinaryReader Reader()
    {
        // UIntPtr Ptr = (UIntPtr)Data;
        // throw new ArgumentException($"Creating a reader for wire with {Length} bytes in it, {Ptr} ptr and {Capacity} capacity");
        var reader = new BinaryReader(new UnmanagedMemoryStream(Data, Length));
        return reader;
    }

    public BinaryWriter Writer()
    {
        var writer = new BinaryWriter(new UnmanagedMemoryStream(Data, Length, Length, FileAccess.Write));
        return writer;
    }

    /// <summary>
    /// Free the buffer if this wire owns it
    /// </summary>
    public void Dispose()
    {
        if (Data != null)
        {
            if (IsOwned) {
                if (Capacity > 0) {
                    WireInterop.deallocate_wire_buffer_storage((IntPtr)Data, Length, Capacity);
                } else {
                    Marshal.FreeHGlobal((IntPtr)Data);
                }
            }
            Data = null;
            Length = 0;
            Capacity = 0;
        }
    }

    /// <summary>
    /// Check if this wire owns its buffer
    /// </summary>
    public bool IsOwned => Capacity != 0;

    /// <summary>
    /// Check if the wire buffer is empty
    /// </summary>
    public bool IsEmpty => Length == 0;
}

/// <summary>
/// Extension methods for {{type}} to create Wire instances
/// </summary>
public static class WireOf{{type}}Extensions
{
    /// <summary>
    /// Create a Wire with owned buffer from this {{type}} instance
    /// </summary>
    {# this makes code slower, do NOT enable [MethodImpl(MethodImplOptions.AggressiveOptimization)] -#}
    public static WireOf{{type}} Wire(this {{type}} value)
    {
        return WireOf{{type}}.From(value);
    }

    /// <summary>
    /// Create a Wire with borrowed buffer from this {{type}} instance
    /// </summary>
    {# this makes code slower, do NOT enable [MethodImpl(MethodImplOptions.AggressiveOptimization)] -#}
    public static unsafe WireOf{{type}} WireWithBuffer(this {{type}} value, byte* buffer, int bufferSize)
    {
        return WireOf{{type}}.From(value, buffer, bufferSize);
    }

    /// <summary>
    /// Calculate the wire size needed for this {{type}} instance
    /// </summary>
    {# this makes code slower, do NOT enable [MethodImpl(MethodImplOptions.AggressiveOptimization)] -#}
    public static int WireSize(this {{type}} value)
    {
        return value.CalculateSize();
    }

    /// <summary>
    /// Unwire a WireOf{{type}} back to a managed {{type}} object
    /// </summary>
    {# this makes code slower, do NOT enable [MethodImpl(MethodImplOptions.AggressiveOptimization)] -#}
    public static {{type}} Unwire(this WireOf{{type}} wire)
    {
        return {{type}}.Deserialize(wire.Reader());
    }
}
