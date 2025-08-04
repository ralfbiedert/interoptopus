public class WireInterop {
    #region Serialization Helpers
    #nullable disable

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static string DeserializeString(BinaryReader reader)
    {
        var length = reader.ReadUInt64();
        if (length == 0)
            return string.Empty;

        var bytes = reader.ReadBytes((int)length);
        return Encoding.UTF8.GetString(bytes);
    }

    // TODO replace with precise serializers
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void SerializeItem<T>(BinaryWriter writer, T item)
    {
        if (typeof(T).IsPrimitive)
        {
            SerializePrimitive(writer, item);
        }
        else if (typeof(T) == typeof(string))
        {
            SerializeString(writer, item as string);
        }
        else
        {
            ((dynamic)item).Serialize(writer);
        }
    }

    // TODO replace with precise deserializers
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static T DeserializeItem<T>(BinaryReader reader)
    {
        if (typeof(T).IsPrimitive)
        {
            return (T)DeserializePrimitive<T>(reader);
        }
        else if (typeof(T) == typeof(string))
        {
            return (T)(object)DeserializeString(reader);
        }
        else
        {
            // For other types, try dynamic dispatch
            return ((dynamic)typeof(T)).Deserialize(reader);
        }
    }

    // TODO: pass itemSerializerDelegate
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void SerializeVec<T>(BinaryWriter writer, IList<T> value)
    {
        if (value == null)
        {
            writer.Write((ulong)0);
            return;
        }

        writer.Write((ulong)value.Count);
        foreach (var item in value)
        {
            SerializeItem(writer, item);
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static byte[] DeserializeVecOfByte(BinaryReader reader)
    {
        var length = reader.ReadUInt64();
        return reader.ReadBytes((int)length);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void SerializeMap<K,V>(BinaryWriter writer, IDictionary<K,V> value)
    {
        if (value == null)
        {
            writer.Write((ulong)0);
            return;
        }

        writer.Write((ulong)value.Count);
        foreach (var item in value)
        {
            WireInterop.SerializeItem(writer, item.Key);
            WireInterop.SerializeItem(writer, item.Value);
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void SerializeOption<T>(BinaryWriter writer, T? value) where T : struct
    {
        if (value.HasValue)
        {
            writer.Write((byte)1);
            SerializeItem<T>(writer, value.Value);
        }
        else
        {
            writer.Write((byte)0);
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static T? DeserializeOption<T>(BinaryReader reader) where T : struct
    {
        var hasValue = reader.ReadByte() != 0;
        if (hasValue)
        {
            return WireInterop.DeserializeItem<T>(reader);
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static T? DeserializeEnum<T>(BinaryReader reader) where T: System.Enum
    {
        var discriminant = reader.ReadInt32();
        if (Enum.IsDefined(typeof(T), discriminant))
        {
            return (T)Enum.ToObject(typeof(T), discriminant);
        }
        return default(T);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void SerializePrimitive<T>(BinaryWriter writer, T value)
    {
        switch (value)
        {

            case bool b: writer.Write((byte)(b ? 1 : 0)); break;
            case sbyte sb: writer.Write(sb); break;
            case byte b: writer.Write(b); break;
            case short s: writer.Write(s); break;
            case ushort us: writer.Write(us); break;
            case int i: writer.Write(i); break;
            case uint ui: writer.Write(ui); break;
            case long l: writer.Write(l); break;
            case ulong ul: writer.Write(ul); break;
            case float f: writer.Write(f); break;
            case double d: writer.Write(d); break;
            default: throw new NotSupportedException($"Primitive type {typeof(T)} not supported");
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static object DeserializePrimitive<T>(BinaryReader reader)
    {
        return typeof(T).Name switch
        {
            "Boolean" => reader.ReadByte() != 0,
            "SByte" => reader.ReadSByte(),
            "Byte" => reader.ReadByte(),
            "Int16" => reader.ReadInt16(),
            "UInt16" => reader.ReadUInt16(),
            "Int32" => reader.ReadInt32(),
            "UInt32" => reader.ReadUInt32(),
            "Int64" => reader.ReadInt64(),
            "UInt64" => reader.ReadUInt64(),
            "Single" => reader.ReadSingle(),
            "Double" => reader.ReadDouble(),
            _ => throw new NotSupportedException($"Primitive type {typeof(T)} not supported")
        };
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static int SizeOf<T>()
    {
        return typeof(T).Name switch
        {
            "Boolean" or "SByte" or "Byte" => 1,
            "Int16" or "UInt16" => 2,
            "Int32" or "UInt32" or "Single" => 4,
            "Int64" or "UInt64" or "Double" => 8,
            _ => Marshal.SizeOf<T>()
        };
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static int CalculateMapSize<K,V>(IDictionary<K,V> value)
    {
        if (value == null) return 8; // size of length field

        int size = 8; // length field
        foreach (var item in value)
        {
            // Calculate key size
            if (typeof(K).IsPrimitive)
            {
                size += SizeOf<K>();
            }
            else if (typeof(K) == typeof(string))
            {
                var keyStr = item.Key as string;
                size += 8 + (keyStr != null ? Encoding.UTF8.GetByteCount(keyStr) : 0);
            }
            else
            {
                size += ((dynamic)item.Key).CalculateSize();
            }

            // Calculate value size
            if (typeof(V).IsPrimitive)
            {
                size += SizeOf<V>();
            }
            else if (typeof(V) == typeof(string))
            {
                var valueStr = item.Value as string;
                size += 8 + (valueStr != null ? Encoding.UTF8.GetByteCount(valueStr) : 0);
            }
            else
            {
                size += ((dynamic)item.Value).CalculateSize();
            }
        }
        return size;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static int CalculateVecSize<T>(IList<T> value)
    {
        if (value == null) return 8; // size of length field

        int size = 8; // length field
        foreach (var item in value)
        {
            if (typeof(T).IsPrimitive)
            {
                size += SizeOf<T>();
            }
            else if (typeof(T) == typeof(string))
            {
                var str = item as string;
                size += 8 + (str != null ? Encoding.UTF8.GetByteCount(str) : 0);
            }
            else
            {
                size += ((dynamic)item).CalculateSize();
            }
        }
        return size;
    }

    #nullable restore
    #endregion
}

public static class DeserStringExtensions
{
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void Serialize(this String value, BinaryWriter writer) {
        WireInterop.SerializeString(writer, value);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static String DeserializeString(BinaryReader reader) {
        return WireInterop.DeserializeString(reader);
    }
}

public static class WireListExtensions
{
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void Serialize<T>(this List<T> value, BinaryWriter writer) {
        WireInterop.SerializeVec(writer, value);
    }
}

public static class WireDictionaryExtensions
{
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void Serialize<K,V>(this Dictionary<K,V> value, BinaryWriter writer) {
        WireInterop.SerializeMap(writer, value);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void Serialize<K,V>(this IDictionary<K,V> value, BinaryWriter writer) {
        WireInterop.SerializeMap(writer, value);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static int CalculateSize<K,V>(this Dictionary<K,V> value) {
        return WireInterop.CalculateMapSize(value);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static int CalculateSize<K,V>(this IDictionary<K,V> value) {
        return WireInterop.CalculateMapSize(value);
    }
}

public static class WireArrayExtensions
{
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void Serialize<T>(this T[] value, BinaryWriter writer) {
        WireInterop.SerializeVec(writer, value);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static int CalculateSize<T>(this T[] value) {
        return WireInterop.CalculateVecSize(value);
    }
}
