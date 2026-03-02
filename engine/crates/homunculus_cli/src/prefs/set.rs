use homunculus_prefs::{PrefsDatabase, SqlValue};
use homunculus_utils::error::{UtilError, UtilResult};

pub(super) fn cmd_set(db: &PrefsDatabase, key: &str, input: &str) -> UtilResult {
    let (value, value_type) = infer_value(input);
    if let Err(e) = db.save(key, value, value_type) {
        return Err(UtilError::Other(anyhow::anyhow!(e)));
    }
    Ok(())
}

/// Infers the SQLite type from a CLI input string.
///
/// Priority: null → bool → integer → float → JSON object/array → string
fn infer_value(input: &str) -> (SqlValue, &'static str) {
    match input {
        "null" => (SqlValue::Null, "null"),
        "true" => (SqlValue::Integer(1), "bool"),
        "false" => (SqlValue::Integer(0), "bool"),
        _ if input.starts_with('{') || input.starts_with('[') => {
            if serde_json::from_str::<serde_json::Value>(input).is_ok() {
                (SqlValue::Text(input.to_owned()), "json")
            } else {
                (SqlValue::Text(input.to_owned()), "string")
            }
        }
        _ => {
            if let Ok(i) = input.parse::<i64>() {
                (SqlValue::Integer(i), "number")
            } else if let Ok(f) = input.parse::<f64>() {
                (SqlValue::Real(f), "number")
            } else {
                (SqlValue::Text(input.to_owned()), "string")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prefs::set::infer_value;
    use homunculus_prefs::SqlValue;

    #[test]
    fn test_infer_null() {
        let (val, vt) = infer_value("null");
        assert_eq!(val, SqlValue::Null);
        assert_eq!(vt, "null");
    }

    #[test]
    fn test_infer_bool_true() {
        let (val, vt) = infer_value("true");
        assert_eq!(val, SqlValue::Integer(1));
        assert_eq!(vt, "bool");
    }

    #[test]
    fn test_infer_bool_false() {
        let (val, vt) = infer_value("false");
        assert_eq!(val, SqlValue::Integer(0));
        assert_eq!(vt, "bool");
    }

    #[test]
    fn test_infer_integer() {
        let (val, vt) = infer_value("42");
        assert_eq!(val, SqlValue::Integer(42));
        assert_eq!(vt, "number");
    }

    #[test]
    fn test_infer_negative_integer() {
        let (val, vt) = infer_value("-7");
        assert_eq!(val, SqlValue::Integer(-7));
        assert_eq!(vt, "number");
    }

    #[test]
    fn test_infer_float() {
        let (val, vt) = infer_value("3.14");
        assert_eq!(val, SqlValue::Real(3.14));
        assert_eq!(vt, "number");
    }

    #[test]
    fn test_infer_json_object() {
        let (val, vt) = infer_value(r#"{"x":1}"#);
        assert_eq!(val, SqlValue::Text(r#"{"x":1}"#.to_owned()));
        assert_eq!(vt, "json");
    }

    #[test]
    fn test_infer_json_array() {
        let (val, vt) = infer_value("[1,2,3]");
        assert_eq!(val, SqlValue::Text("[1,2,3]".to_owned()));
        assert_eq!(vt, "json");
    }

    #[test]
    fn test_infer_string() {
        let (val, vt) = infer_value("dark");
        assert_eq!(val, SqlValue::Text("dark".to_owned()));
        assert_eq!(vt, "string");
    }

    #[test]
    fn test_infer_invalid_json_as_string() {
        let (val, vt) = infer_value("{not json");
        assert_eq!(val, SqlValue::Text("{not json".to_owned()));
        assert_eq!(vt, "string");
    }
}
