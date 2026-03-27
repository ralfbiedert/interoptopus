// FromCall
{%- if ok_has_payload %}
public static {{ name }} FromCall(Func<{{ ok_type }}> func)
{
    try { return Ok(func()); }
{%- if is_try_error %}
{%- for ex in exceptions %}
    catch ({{ ex.name }}) { return Err(new {{ err_type }} { exception_id = {{ ex.id }} }); }
{%- endfor %}
    catch (Exception) { return Err(new {{ err_type }}()); }
{%- else %}
    catch (Exception) { return Panic; }
{%- endif %}
}

public static async Task<{{ name }}> FromCallAsync(Func<Task<{{ ok_type }}>> func)
{
    try { return Ok(await func()); }
{%- if is_try_error %}
{%- for ex in exceptions %}
    catch ({{ ex.name }}) { return Err(new {{ err_type }} { exception_id = {{ ex.id }} }); }
{%- endfor %}
    catch (Exception) { return Err(new {{ err_type }}()); }
{%- else %}
    catch (Exception) { return Panic; }
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
    catch (Exception) { return Err(new {{ err_type }}()); }
{%- else %}
    catch (Exception) { return Panic; }
{%- endif %}
}

public static async Task<{{ name }}> FromCallAsync(Func<Task> func)
{
    try { await func(); return Ok; }
{%- if is_try_error %}
{%- for ex in exceptions %}
    catch ({{ ex.name }}) { return Err(new {{ err_type }} { exception_id = {{ ex.id }} }); }
{%- endfor %}
    catch (Exception) { return Err(new {{ err_type }}()); }
{%- else %}
    catch (Exception) { return Panic; }
{%- endif %}
}
{%- endif %}
