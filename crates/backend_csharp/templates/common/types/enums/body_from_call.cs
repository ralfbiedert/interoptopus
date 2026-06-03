{%- if mode_is_plugin %}
// Pass-through helpers: plugin C# methods return the full Result themselves.
// Any uncaught exception is folded into `Panic` so it travels back to Rust over the wire,
// while still notifying the global exception handler for observability.
// `OperationCanceledException` is re-thrown so the caller's `ContinueWith` can distinguish
// cancellation from a thrown fault and route to `UnsafeCompleteCancelled`.
public static {{ name }} FromCallResult(Func<{{ name }}> func)
{
    try { return func(); }
    catch (Exception e) { Trampoline.UncaughtException(e.ToString()); return Panic; }
}

public static async Task<{{ name }}> FromCallResultAsync(Func<Task<{{ name }}>> func)
{
    try { return await func(); }
    catch (OperationCanceledException) { throw; }
    catch (Exception e) { Trampoline.UncaughtException(e.ToString()); return Panic; }
}

{% endif -%}
// FromCall
{%- if ok_has_payload %}
public static {{ name }} FromCall(Func<{{ ok_type }}> func)
{
    try { return Ok(func()); }
{%- if is_try_error %}
{%- for ex in exceptions %}
    catch ({{ ex.name }}) { return Err(new {{ err_type }} { exception_id = {{ ex.id }} }); }
{%- endfor %}
    catch (Exception) { return Err(new {{ err_type }} { exception_id = 0 }); }
{%- else %}
    catch (Exception e) { Trampoline.UncaughtException(e.ToString()); return Panic; }
{%- endif %}
}

public static async Task<{{ name }}> FromCallAsync(Func<Task<{{ ok_type }}>> func)
{
    try { return Ok(await func()); }
{%- if is_try_error %}
{%- for ex in exceptions %}
    catch ({{ ex.name }}) { return Err(new {{ err_type }} { exception_id = {{ ex.id }} }); }
{%- endfor %}
    catch (Exception) { return Err(new {{ err_type }} { exception_id = 0 }); }
{%- else %}
    catch (OperationCanceledException) { throw; }
    catch (Exception e) { Trampoline.UncaughtException(e.ToString()); return Panic; }
{%- endif %}
}
{%- else %}
public static {{ name }} FromCall(Action action)
{
    try { action(); return Ok; }
{%- if is_try_error %}
{%- for ex in exceptions %}
    catch ({{ ex.name }}) { return Err(new {{ err_type }} { exception_id = {{ ex.id }} }); }
{%- endfor %}
    catch (Exception) { return Err(new {{ err_type }} { exception_id = 0 }); }
{%- else %}
    catch (Exception e) { Trampoline.UncaughtException(e.ToString()); return Panic; }
{%- endif %}
}

public static async Task<{{ name }}> FromCallAsync(Func<Task> func)
{
    try { await func(); return Ok; }
{%- if is_try_error %}
{%- for ex in exceptions %}
    catch ({{ ex.name }}) { return Err(new {{ err_type }} { exception_id = {{ ex.id }} }); }
{%- endfor %}
    catch (Exception) { return Err(new {{ err_type }} { exception_id = 0 }); }
{%- else %}
    catch (OperationCanceledException) { throw; }
    catch (Exception e) { Trampoline.UncaughtException(e.ToString()); return Panic; }
{%- endif %}
}
{%- endif %}
