//! Output pass — renders the C model into a complete `.h` header via Tera templates.
//!
//! Each type kind dispatches to its own template. The final assembly
//! concatenates header, type definitions (in topo order), function
//! declarations, dispatch table, loaders, and footer.

use crate::lang::{CFunction, CModel, CTypeKind};
use interoptopus_backends::Error;
use interoptopus_backends::template::{Context, TemplateEngine, Value};

/// A function entry annotated with whether a separator comment should precede it.
struct FnEntry<'a> {
    func: &'a CFunction,
    separator: bool,
}

/// Annotate the function list with separator flags for the user/internal boundary.
fn fn_entries(model: &CModel) -> Vec<FnEntry<'_>> {
    let mut prev_internal = false;
    let mut first_user_seen = false;
    model
        .functions
        .iter()
        .map(|f| {
            let separator = !prev_internal && f.is_internal && first_user_seen;
            prev_internal = f.is_internal;
            if !f.is_internal {
                first_user_seen = true;
            }
            FnEntry { func: f, separator }
        })
        .collect()
}

/// Convert annotated function entries into a Tera-compatible value list.
fn fn_entries_value(entries: &[FnEntry<'_>]) -> Value {
    let values: Vec<Value> = entries
        .iter()
        .map(|e| {
            let mut m = tera::Map::new();
            m.insert("name".into(), Value::String(e.func.name.clone()));
            m.insert("rval".into(), Value::String(e.func.rval.clone()));
            m.insert("param_types".into(), Value::String(e.func.param_types.clone()));
            m.insert("params".into(), Value::String(e.func.params.clone()));
            m.insert("separator".into(), Value::Bool(e.separator));
            Value::Object(m)
        })
        .collect();
    Value::Array(values)
}

/// Render a single type definition to a string fragment.
#[allow(clippy::too_many_lines)]
fn render_type(engine: &TemplateEngine, name: &str, kind: &CTypeKind) -> Result<String, Error> {
    match kind {
        CTypeKind::Primitive(_) | CTypeKind::Pointer(_) | CTypeKind::Array(_) => Ok(String::new()),

        CTypeKind::Struct(s) => {
            let fields: Vec<Value> = s
                .fields
                .iter()
                .map(|f| {
                    let mut m = tera::Map::new();
                    m.insert("name".into(), Value::String(f.name.clone()));
                    m.insert("type_name".into(), Value::String(f.type_name.clone()));
                    if let Some(len) = f.array_len {
                        m.insert("array_len".into(), Value::Number(len.into()));
                    }
                    Value::Object(m)
                })
                .collect();
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("fields", &fields);
            engine.render("types/struct.h", &ctx)
        }

        CTypeKind::SimpleEnum(e) => {
            let variants: Vec<Value> = e
                .variants
                .iter()
                .map(|v| {
                    let mut m = tera::Map::new();
                    m.insert("name".into(), Value::String(v.name.clone()));
                    m.insert("value".into(), Value::Number(v.value.into()));
                    Value::Object(m)
                })
                .collect();
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("tag_c_type", &e.tag_c_type);
            ctx.insert("variants", &variants);
            engine.render("types/simple_enum.h", &ctx)
        }

        CTypeKind::TaggedUnion(tu) => {
            let variants: Vec<Value> = tu
                .variants
                .iter()
                .map(|v| {
                    let field_name = v.name.strip_prefix(&format!("{name}_")).unwrap_or(&v.name).to_lowercase();
                    let mut m = tera::Map::new();
                    m.insert("name".into(), Value::String(v.name.clone()));
                    m.insert("tag".into(), Value::Number(v.tag.into()));
                    m.insert("field_name".into(), Value::String(field_name));
                    if let Some(dt) = &v.data_type {
                        m.insert("data_type".into(), Value::String(dt.clone()));
                    }
                    Value::Object(m)
                })
                .collect();
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("tag_name", &tu.tag_name);
            ctx.insert("tag_c_type", &tu.tag_c_type);
            ctx.insert("variants", &variants);
            engine.render("types/tagged_union.h", &ctx)
        }

        CTypeKind::Callback(cb) => {
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("fn_typedef", &cb.fn_typedef);
            ctx.insert("rval", &cb.rval);
            ctx.insert("params", &cb.params);
            engine.render("types/callback.h", &ctx)
        }

        CTypeKind::FnPointer(fp) => {
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("rval", &fp.rval);
            ctx.insert("params", &fp.params);
            engine.render("types/fn_pointer.h", &ctx)
        }

        CTypeKind::Slice(s) | CTypeKind::SliceMut(s) => {
            let const_q = if s.is_const { "const " } else { "" };
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("inner_type", &s.inner_type);
            ctx.insert("const_qualifier", const_q);
            engine.render("types/slice.h", &ctx)
        }

        CTypeKind::Vec(v) => {
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("inner_type", &v.inner_type);
            engine.render("types/vec.h", &ctx)
        }

        CTypeKind::Utf8String => {
            let mut ctx = Context::new();
            ctx.insert("name", name);
            engine.render("types/utf8string.h", &ctx)
        }

        CTypeKind::Option(o) => {
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("tag_name", &o.tag_name);
            ctx.insert("tag_c_type", &o.tag_c_type);
            ctx.insert("inner_type", &o.inner_type);
            engine.render("types/option.h", &ctx)
        }

        CTypeKind::Result(r) => {
            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("tag_name", &r.tag_name);
            ctx.insert("tag_c_type", &r.tag_c_type);
            ctx.insert("ok_type", &r.ok_type);
            ctx.insert("err_type", &r.err_type);
            engine.render("types/result.h", &ctx)
        }

        CTypeKind::Opaque => {
            let mut ctx = Context::new();
            ctx.insert("name", name);
            engine.render("types/opaque.h", &ctx)
        }
    }
}

/// Render the complete `.h` header from the model.
pub fn render_header(engine: &TemplateEngine, model: &CModel, loader_name: &str, ifndef: &str) -> Result<String, Error> {
    let mut out = String::new();

    // Precompute function entries with separator flags (used by declarations, dispatch table, and loaders).
    let entries = fn_entries(model);
    let entries_value = fn_entries_value(&entries);
    let api_name = format!("{loader_name}_api_t");
    let load_fn = format!("{loader_name}_load");

    // Header guard + includes.
    {
        let mut ctx = Context::new();
        ctx.insert("ifndef", ifndef);
        out.push_str(&engine.render("header.h", &ctx)?);
    }

    // Type definitions in topological order.
    for tid in &model.types_ordered {
        if let Some(ctype) = model.types.get(tid) {
            let fragment = render_type(engine, &ctype.name, &ctype.kind)?;
            if !fragment.is_empty() {
                out.push_str(&fragment);
                out.push('\n');
            }
        }
    }

    // Function declarations.
    for e in &entries {
        if e.separator {
            out.push_str("/* internal helpers */\n");
        }
        let mut ctx = Context::new();
        ctx.insert("rval", &e.func.rval);
        ctx.insert("name", &e.func.name);
        ctx.insert("params", &e.func.params);
        out.push_str(&engine.render("fns/declaration.h", &ctx)?);
    }
    out.push('\n');

    // Dispatch table.
    {
        let mut ctx = Context::new();
        ctx.insert("api_name", &api_name);
        ctx.insert("functions", &entries_value);
        out.push_str(&engine.render("dispatch_table.h", &ctx)?);
        out.push('\n');
    }

    // Dynamic loader (Windows).
    {
        let mut ctx = Context::new();
        ctx.insert("load_fn", &load_fn);
        ctx.insert("api_name", &api_name);
        ctx.insert("functions", &entries_value);
        out.push_str(&engine.render("loader/dynamic_win32.h", &ctx)?);
    }

    // Dynamic loader (POSIX).
    {
        let mut ctx = Context::new();
        ctx.insert("load_fn", &load_fn);
        ctx.insert("api_name", &api_name);
        ctx.insert("functions", &entries_value);
        out.push_str(&engine.render("loader/dynamic_posix.h", &ctx)?);
        out.push('\n');
    }

    // Static loader.
    {
        let static_load_fn = format!("{loader_name}_load_static");
        let guard = format!("{}_STATIC", loader_name.to_uppercase());
        let mut ctx = Context::new();
        ctx.insert("load_fn", &static_load_fn);
        ctx.insert("api_name", &api_name);
        ctx.insert("guard", &guard);
        ctx.insert("functions", &entries_value);
        out.push_str(&engine.render("loader/static.h", &ctx)?);
        out.push('\n');
    }

    // Footer.
    {
        let mut ctx = Context::new();
        ctx.insert("ifndef", ifndef);
        out.push_str(&engine.render("footer.h", &ctx)?);
    }

    Ok(out)
}
