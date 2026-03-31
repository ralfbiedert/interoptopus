static Interop()
{
    var api_version = Interop.{{ fn_name }}();
    if (api_version != 0x{{ hash_hex }})
    {
        throw new TypeLoadException($"API reports hash 0x{api_version:X} which differs from hash in bindings (0x{{ hash_hex }}). You probably forgot to update / copy either the bindings or the library.");
    }
}