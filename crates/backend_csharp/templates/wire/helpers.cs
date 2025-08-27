public partial class WireInterop {
    [LibraryImport(Interop.NativeLib, EntryPoint = "deallocate_wire_buffer_storage")]
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static partial void deallocate_wire_buffer_storage(IntPtr data, int len, int capacity);

    #region Serialization Helpers
    #nullable enable

    public static void SerializeString(BinaryWriter writer, string value)
    {
        if (value == null)
        {
            writer.Write((ulong)0);
            return;
        }

        var bytes = Encoding.UTF8.GetBytes(value);
        writer.Write((ulong)bytes.Length);
        writer.Write(bytes);
    }

    public static string DeserializeString(BinaryReader reader)
    {
        var length = reader.ReadUInt64();
        if (length == 0)
            return string.Empty;

        var bytes = reader.ReadBytes((int)length);
        return Encoding.UTF8.GetString(bytes);
    }

    public static void SerializeVecOfByte(BinaryWriter writer, byte[] vec)
    {
        writer.Write((ulong)vec.Length);
        writer.Write(vec);
    }

    public static void SerializeVec<T>(BinaryWriter writer, IList<T> value, Action<BinaryWriter, T> serializeItem)
    {
        if (value == null)
        {
            writer.Write((ulong)0);
            return;
        }

        writer.Write((ulong)value.Count);
        foreach (var item in value)
        {
            serializeItem(writer, item);
        }
    }

    public static byte[] DeserializeVecOfByte(BinaryReader reader)
    {
        var length = reader.ReadUInt64();
        return reader.ReadBytes((int)length);
    }

    public static T[] DeserializeVec<T>(BinaryReader reader, Func<BinaryReader, T> deserializeItem)
    {
        var length = reader.ReadUInt64();
        var result = new T[(int)length];

        for (ulong i = 0; i < length; i++)
        {
            result[i] = deserializeItem(reader);
        }

        return result;
    }

    public static void SerializeMap<K,V>(BinaryWriter writer, IDictionary<K,V> value, Action<BinaryWriter, K> serializeKey, Action<BinaryWriter, V> serializeValue)
    {
        if (value == null)
        {
            writer.Write((ulong)0);
            return;
        }

        writer.Write((ulong)value.Count);
        foreach (var item in value)
        {
            serializeKey(writer, item.Key);
            serializeValue(writer, item.Value);
        }
    }

    public static Dictionary<K,V> DeserializeMap<K,V>(BinaryReader reader, Func<BinaryReader, K> deserializeKey, Func<BinaryReader, V> deserializeValue)
    {
        var length = reader.ReadUInt64();
        var result = new Dictionary<K,V>((int)length);

        for (ulong i = 0; i < length; i++)
        {
            var k = deserializeKey(reader);
            var v = deserializeValue(reader);
            result.Add(k, v);
        }

        return result;
    }

    public static void SerializeOptional<T>(BinaryWriter writer, T? value, Action<BinaryWriter, T> serializeItem)
    {
        if (value != null)
        {
            writer.Write((byte)1);
            serializeItem(writer, value);
        }
        else
        {
            writer.Write((byte)0);
        }
    }

    #nullable enable
    public static T? DeserializeOptional<T>(BinaryReader reader, Func<BinaryReader, T> deserializeValue)
    {
        var hasValue = reader.ReadByte() != 0;
        if (hasValue)
        {
            return deserializeValue(reader);
        }
        return default;
    }
    #nullable restore

    public static T? DeserializeEnum<T>(BinaryReader reader) where T: System.Enum
    {
        var discriminant = reader.ReadInt32();
        if (Enum.IsDefined(typeof(T), discriminant))
        {
            return (T)Enum.ToObject(typeof(T), discriminant);
        }
        return default(T);
    }

    public static int CalculateVariableMapSize<K,V>(IDictionary<K,V> value, Func<K, int> calculateKeySize, Func<V, int> calculateValueSize)
    {
        int size = Marshal.SizeOf<ulong>(); // length field
        if (value == null) return size;

        foreach (var item in value)
        {
            size +=
                calculateKeySize(item.Key)
                + calculateValueSize(item.Value);
            {#-
            // Calculate key size
            // if (typeof(K).IsPrimitive) -- todo! opt for fixed size k/v pairs
            // {
            //     size += Marshal.SizeOf<K>();
            // }

            // if (typeof(V).IsPrimitive)
            // {
            //     size += Marshal.SizeOf<V>();
            // }
            -#}
        }
        return size;
    }

    /// This method is called only for non-primitive inner types which require size calculations.
    public static int CalculateVariableVecSize<T>(IList<T> value, Func<T, int> calculateItemSize)
    {
        int size = Marshal.SizeOf<ulong>(); // length field
        if (value == null) return size;

        {#- This is handled by calculate_size.cs code path
        if (typeof(T).IsPrimitive)
        {
            size += Marshal.SizeOf<T>() * value.Count;
            return size;
        }-#}

        foreach (var item in value)
        {
            size += calculateItemSize(item);
        }
        return size;
    }

    #nullable restore
    #endregion
}

public static class SerdeStringExtensions
{
    public static void Serialize(this String value, BinaryWriter writer) {
        WireInterop.SerializeString(writer, value);
    }

    public static String DeserializeString(BinaryReader reader) {
        return WireInterop.DeserializeString(reader);
    }

    public static int CalculateSize(String value) {
        return Marshal.SizeOf<ulong>() + System.Text.Encoding.UTF8.GetByteCount(value ?? "");
    }
}
