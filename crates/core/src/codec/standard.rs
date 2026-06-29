//! Standard JSON object codec.
//!
//! snake_case Rust field name → camelCase wire key by default; `wire_name`
//! overrides it. Read-only fields (`#[id]`, `#[wire(read_only)]`) are skipped on
//! encode. `Option::None` is omitted. Secrets are exposed to their plaintext
//! string at this boundary. Nested resources recurse via the erased view.

use serde_json::{Map, Number, Value};

use crate::described::Described;
use crate::descriptor::FieldDescriptor;
use crate::field::FieldRef;

/// Encode a resource to its wire JSON object.
pub fn encode<T: Described>(value: &T) -> anyhow::Result<Value> {
    let desc = T::descriptor();
    let mut map = Map::new();
    for f in desc.fields {
        emit_field(&mut map, f, (f.get)(value))?;
    }
    Ok(Value::Object(map))
}

/// Write one field into the parent object. Read-only fields are skipped;
/// `#[flatten]` nested fields have their keys hoisted into the parent rather
/// than nested under their own key.
fn emit_field<T: 'static>(
    map: &mut Map<String, Value>,
    f: &FieldDescriptor<T>,
    r: FieldRef<'_>,
) -> anyhow::Result<()> {
    if f.read_only {
        return Ok(());
    }
    if f.flatten
        && let FieldRef::Nested(n) = &r
    {
        if let Value::Object(m) = n.encode_self()? {
            map.extend(m);
        }
        return Ok(());
    }
    // `VecNested` owns a boxed iterator → encode it here, by value; everything
    // else goes through the borrowing `field_to_json`.
    match r {
        FieldRef::VecNested(it) => {
            let mut arr = Vec::new();
            for n in it {
                arr.push(n.encode_self()?);
            }
            map.insert(wire_key(f.name, f.wire_name), Value::Array(arr));
        }
        other => {
            if let Some(v) = field_to_json(&other)? {
                map.insert(wire_key(f.name, f.wire_name), v);
            }
        }
    }
    Ok(())
}

/// Convert one borrowed field value to JSON. `Ok(None)` means "omit this key"
/// (a `None` optional).
pub(crate) fn field_to_json(r: &FieldRef<'_>) -> anyhow::Result<Option<Value>> {
    let v = match r {
        FieldRef::Bool(b) => Value::Bool(**b),
        FieldRef::Int32(i) => Value::Number((**i).into()),
        FieldRef::Int64(i) => Value::Number((**i).into()),
        FieldRef::Float64(f) => num_f64(**f)?,
        FieldRef::String(s) => Value::String((*s).clone()),

        FieldRef::OptBool(o) => return Ok((**o).map(Value::Bool)),
        FieldRef::OptInt32(o) => return Ok((**o).map(|i| Value::Number(i.into()))),
        FieldRef::OptInt64(o) => return Ok((**o).map(|i| Value::Number(i.into()))),
        FieldRef::OptFloat64(o) => return (**o).map(num_f64).transpose(),
        FieldRef::OptString(o) => return Ok((*o).clone().map(Value::String)),

        FieldRef::VecBool(s) => Value::Array(s.iter().map(|b| Value::Bool(*b)).collect()),
        FieldRef::VecInt32(s) => {
            Value::Array(s.iter().map(|i| Value::Number((*i).into())).collect())
        }
        FieldRef::VecInt64(s) => {
            Value::Array(s.iter().map(|i| Value::Number((*i).into())).collect())
        }
        FieldRef::VecString(s) => Value::Array(s.iter().cloned().map(Value::String).collect()),

        FieldRef::Secret(sv) => Value::String(sv.expose().to_string()),
        FieldRef::OptSecret(o) => {
            return Ok((**o)
                .as_ref()
                .map(|sv| Value::String(sv.expose().to_string())));
        }

        FieldRef::Nested(n) => n.encode_self()?,
        // `VecNested` owns its iterator and is consumed by the standard codec's
        // `emit_field`; it never reaches the borrowing `field_to_json`.
        FieldRef::VecNested(_) => {
            anyhow::bail!(
                "vec-nested fields are encoded by the standard codec, not the fields-blob"
            )
        }

        FieldRef::Json(v) => (**v).clone(),
        FieldRef::OptJson(o) => return Ok((**o).clone()),
        FieldRef::VecJson(s) => Value::Array(s.to_vec()),
    };
    Ok(Some(v))
}

fn num_f64(x: f64) -> anyhow::Result<Value> {
    Number::from_f64(x)
        .map(Value::Number)
        .ok_or_else(|| anyhow::anyhow!("non-finite float cannot be encoded as JSON: {x}"))
}

/// Wire key: explicit `wire_name` override, else snake_case → camelCase.
pub(crate) fn wire_key(name: &str, wire_name: Option<&str>) -> String {
    match wire_name {
        Some(w) => w.to_string(),
        None => to_camel_case(name),
    }
}

fn to_camel_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper = false;
    for c in s.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.extend(c.to_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// Decode a resource from its wire JSON object.
///
/// Builds an empty instance, then populates each present (non-null) field via
/// its `set` closure. `#[default]` is deliberately not applied — this decodes
/// the server's full wire object; default-filling is the config codec's job
/// ([`super::config`]), on ingest.
pub fn decode<T: Described>(value: &Value) -> anyhow::Result<T> {
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("standard::decode expected a JSON object"))?;

    let mut out = T::empty();
    let desc = T::descriptor();
    for f in desc.fields {
        // A flattened nested struct reads its fields from the parent object.
        if f.flatten {
            let fv = crate::field::FieldValue::Nested(value.clone());
            (f.set)(&mut out, fv).map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
            continue;
        }
        let key = wire_key(f.name, f.wire_name);
        let Some(jv) = obj.get(&key) else { continue };
        if jv.is_null() {
            continue;
        }
        let fv = json_to_field_value(f.kind, f.secret, jv)
            .map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
        (f.set)(&mut out, fv).map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
    }
    Ok(out)
}

/// Convert a JSON value to the owned [`FieldValue`] matching `kind`.
pub(crate) fn json_to_field_value(
    kind: &crate::field::FieldKind,
    secret: bool,
    jv: &Value,
) -> anyhow::Result<crate::field::FieldValue> {
    use crate::field::{FieldKind as K, FieldValue as V};

    Ok(match kind {
        K::Bool => V::Bool(jv.as_bool().ok_or_else(|| err("bool", jv))?),
        K::Int32 => V::Int32(jv.as_i64().ok_or_else(|| err("int", jv))? as i32),
        K::Int64 => V::Int64(jv.as_i64().ok_or_else(|| err("int", jv))?),
        K::Float64 => V::Float64(jv.as_f64().ok_or_else(|| err("float", jv))?),
        K::String if secret => V::Secret(jv.as_str().ok_or_else(|| err("string", jv))?.to_string()),
        K::String => V::String(jv.as_str().ok_or_else(|| err("string", jv))?.to_string()),

        K::Optional(inner) => match &**inner {
            K::Bool => V::OptBool(Some(jv.as_bool().ok_or_else(|| err("bool", jv))?)),
            K::Int32 => V::OptInt32(Some(jv.as_i64().ok_or_else(|| err("int", jv))? as i32)),
            K::Int64 => V::OptInt64(Some(jv.as_i64().ok_or_else(|| err("int", jv))?)),
            K::Float64 => V::OptFloat64(Some(jv.as_f64().ok_or_else(|| err("float", jv))?)),
            K::String if secret => {
                V::Secret(jv.as_str().ok_or_else(|| err("string", jv))?.to_string())
            }
            K::String => V::OptString(Some(
                jv.as_str().ok_or_else(|| err("string", jv))?.to_string(),
            )),
            K::Json => V::Json(jv.clone()),
            // `Option<Nested>` — the `set` closure wraps the decoded value in `Some`.
            K::Nested { .. } => V::Nested(jv.clone()),
            other => anyhow::bail!("optional decode unsupported for {other:?}"),
        },

        K::Vec(inner) => {
            let arr = jv.as_array().ok_or_else(|| err("array", jv))?;
            match &**inner {
                K::Bool => V::VecBool(collect(arr, |x| x.as_bool(), "bool")?),
                K::Int32 => V::VecInt32(collect(arr, |x| x.as_i64().map(|i| i as i32), "int")?),
                K::Int64 => V::VecInt64(collect(arr, |x| x.as_i64(), "int")?),
                K::String => {
                    V::VecString(collect(arr, |x| x.as_str().map(str::to_string), "string")?)
                }
                K::Json => V::VecJson(arr.clone()),
                K::Nested { .. } => V::VecNested(arr.clone()),
                other => anyhow::bail!("vec decode unsupported for {other:?}"),
            }
        }

        K::Json => V::Json(jv.clone()),

        // Hand the sub-object to the field's `set`, which knows the concrete
        // nested type and calls its decode.
        K::Nested { .. } => V::Nested(jv.clone()),
    })
}

fn err(ty: &str, jv: &Value) -> anyhow::Error {
    anyhow::anyhow!("expected {ty}, got {jv}")
}

fn collect<T>(arr: &[Value], f: impl Fn(&Value) -> Option<T>, ty: &str) -> anyhow::Result<Vec<T>> {
    arr.iter().map(|x| f(x).ok_or_else(|| err(ty, x))).collect()
}
