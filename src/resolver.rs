use serde_json::Value;

pub fn resolve_secrets(val: &mut Value) -> Result<(), anyhow::Error> {
    let to_resolve = match val {
        Value::String(s) if s.starts_with("env://") || s.starts_with("file://") => Some(s.clone()),
        _ => None,
    };

    if let Some(s) = to_resolve {
        *val = resolve_string(&s)?;
    } else {
        match val {
            Value::Object(map) => {
                for sub_val in map.values_mut() {
                    resolve_secrets(sub_val)?;
                }
            }
            Value::Array(arr) => {
                for item in arr.iter_mut() {
                    resolve_secrets(item)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn resolve_string(s: &str) -> Result<Value, anyhow::Error> {
    if let Some(env_name) = s.strip_prefix("env://") {
        let raw_val = std::env::var(env_name)
            .map_err(|_| anyhow::anyhow!("Environment variable '{}' not set", env_name))?;
        coerce_value(raw_val.trim())
    } else if let Some(file_path) = s.strip_prefix("file://") {
        let raw_val = std::fs::read_to_string(file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read secret file '{}': {}", file_path, e))?;
        coerce_value(raw_val.trim())
    } else {
        Ok(Value::String(s.to_string()))
    }
}

fn coerce_value(trimmed: &str) -> Result<Value, anyhow::Error> {
    // 1. Boolean check
    if trimmed == "true" {
        return Ok(Value::Bool(true));
    }
    if trimmed == "false" {
        return Ok(Value::Bool(false));
    }

    // 2. Integer check
    if let Ok(parsed_int) = trimmed.parse::<i64>() {
        return Ok(Value::Number(parsed_int.into()));
    }

    // 3. Float check
    if let Some(num) = trimmed
        .parse::<f64>()
        .ok()
        .and_then(serde_json::Number::from_f64)
    {
        return Ok(Value::Number(num));
    }

    // 4. Default to String
    Ok(Value::String(trimmed.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::env;
    use std::fs;

    #[test]
    fn test_resolve_env_string() -> Result<(), anyhow::Error> {
        unsafe {
            env::set_var("TEST_ENV_STRING", "my_secret_string");
        }
        let result = resolve_string("env://TEST_ENV_STRING")?;
        assert_eq!(result, Value::String("my_secret_string".to_string()));
        unsafe {
            env::remove_var("TEST_ENV_STRING");
        }
        Ok(())
    }

    #[test]
    fn test_resolve_env_bool_true() -> Result<(), anyhow::Error> {
        unsafe {
            env::set_var("TEST_ENV_BOOL_T", "true");
        }
        let result = resolve_string("env://TEST_ENV_BOOL_T")?;
        assert_eq!(result, Value::Bool(true));
        unsafe {
            env::remove_var("TEST_ENV_BOOL_T");
        }
        Ok(())
    }

    #[test]
    fn test_resolve_env_bool_false() -> Result<(), anyhow::Error> {
        unsafe {
            env::set_var("TEST_ENV_BOOL_F", "false");
        }
        let result = resolve_string("env://TEST_ENV_BOOL_F")?;
        assert_eq!(result, Value::Bool(false));
        unsafe {
            env::remove_var("TEST_ENV_BOOL_F");
        }
        Ok(())
    }

    #[test]
    fn test_resolve_env_int() -> Result<(), anyhow::Error> {
        unsafe {
            env::set_var("TEST_ENV_INT", "8080");
        }
        let result = resolve_string("env://TEST_ENV_INT")?;
        assert_eq!(result, Value::Number(8080.into()));
        unsafe {
            env::remove_var("TEST_ENV_INT");
        }
        Ok(())
    }

    #[test]
    fn test_resolve_env_float() -> Result<(), anyhow::Error> {
        unsafe {
            env::set_var("TEST_ENV_FLOAT", "12.34");
        }
        let result = resolve_string("env://TEST_ENV_FLOAT")?;
        let expected_num =
            serde_json::Number::from_f64(12.34).ok_or_else(|| anyhow::anyhow!("invalid float"))?;
        assert_eq!(result, Value::Number(expected_num));
        unsafe {
            env::remove_var("TEST_ENV_FLOAT");
        }
        Ok(())
    }

    #[test]
    fn test_resolve_env_missing() {
        unsafe {
            env::remove_var("TEST_MISSING_ENV_VAR");
        }
        let result = resolve_string("env://TEST_MISSING_ENV_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_file_string() -> Result<(), anyhow::Error> {
        let dir = "target/test_secrets";
        let file_path = "target/test_secrets/string_secret.txt";
        fs::create_dir_all(dir)?;
        fs::write(file_path, "file_secret_data")?;

        let result = resolve_string(&format!("file://{}", file_path));
        let _ = fs::remove_file(file_path);

        let val = result?;
        assert_eq!(val, Value::String("file_secret_data".to_string()));
        Ok(())
    }

    #[test]
    fn test_resolve_file_trimmed_whitespace() -> Result<(), anyhow::Error> {
        let dir = "target/test_secrets";
        let file_path = "target/test_secrets/trimmed_secret.txt";
        fs::create_dir_all(dir)?;
        fs::write(file_path, "file_secret_data\n")?;

        let result = resolve_string(&format!("file://{}", file_path));
        let _ = fs::remove_file(file_path);

        let val = result?;
        assert_eq!(val, Value::String("file_secret_data".to_string()));
        Ok(())
    }

    #[test]
    fn test_resolve_file_coerced_int() -> Result<(), anyhow::Error> {
        let dir = "target/test_secrets";
        let file_path = "target/test_secrets/int_secret.txt";
        fs::create_dir_all(dir)?;
        fs::write(file_path, "9999\n")?;

        let result = resolve_string(&format!("file://{}", file_path));
        let _ = fs::remove_file(file_path);

        let val = result?;
        assert_eq!(val, Value::Number(9999.into()));
        Ok(())
    }

    #[test]
    fn test_resolve_file_coerced_bool() -> Result<(), anyhow::Error> {
        let dir = "target/test_secrets";
        let file_path = "target/test_secrets/bool_secret.txt";
        fs::create_dir_all(dir)?;
        fs::write(file_path, "true\n")?;

        let result = resolve_string(&format!("file://{}", file_path));
        let _ = fs::remove_file(file_path);

        let val = result?;
        assert_eq!(val, Value::Bool(true));
        Ok(())
    }

    #[test]
    fn test_resolve_file_missing() {
        let file_path = "target/test_secrets/non_existent.txt";
        let _ = fs::remove_file(file_path);
        let result = resolve_string(&format!("file://{}", file_path));
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_secrets_recursive_object() -> Result<(), anyhow::Error> {
        unsafe {
            env::set_var("TEST_REC_OBJ_ENV", "resolved_obj_val");
        }
        let mut val = json!({ "port": "env://TEST_REC_OBJ_ENV", "name": "static" });

        let result = resolve_secrets(&mut val);
        unsafe {
            env::remove_var("TEST_REC_OBJ_ENV");
        }
        result?;

        assert_eq!(val["port"], Value::String("resolved_obj_val".to_string()));
        Ok(())
    }

    #[test]
    fn test_resolve_secrets_recursive_array() -> Result<(), anyhow::Error> {
        unsafe {
            env::set_var("TEST_REC_ARR_ENV", "true");
        }
        let mut val = json!(["env://TEST_REC_ARR_ENV", 100]);

        let result = resolve_secrets(&mut val);
        unsafe {
            env::remove_var("TEST_REC_ARR_ENV");
        }
        result?;

        assert_eq!(val[0], Value::Bool(true));
        Ok(())
    }

    #[test]
    fn test_resolve_secrets_recursive_deep() -> Result<(), anyhow::Error> {
        unsafe {
            env::set_var("TEST_DEEP_ENV", "12.34");
        }
        let mut val = json!({ "sub": { "nested": "env://TEST_DEEP_ENV" } });

        let result = resolve_secrets(&mut val);
        unsafe {
            env::remove_var("TEST_DEEP_ENV");
        }
        result?;

        let expected_num =
            serde_json::Number::from_f64(12.34).ok_or_else(|| anyhow::anyhow!("invalid float"))?;
        assert_eq!(val["sub"]["nested"], Value::Number(expected_num));
        Ok(())
    }
}
