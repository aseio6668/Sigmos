use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Ensures f64 values are always valid (not NaN or infinite)
/// Replaces invalid values with safe defaults
pub fn safe_f64(value: f64, default: f64) -> f64 {
    if value.is_finite() && !value.is_nan() {
        value
    } else {
        eprintln!("Warning: Invalid f64 value {:?} replaced with {}", value, default);
        default
    }
}

/// Safe division that prevents NaN and infinity
pub fn safe_divide(numerator: f64, denominator: f64, default: f64) -> f64 {
    if denominator == 0.0 || !denominator.is_finite() {
        default
    } else {
        let result = numerator / denominator;
        safe_f64(result, default)
    }
}

/// Safe multiplication that prevents overflow/underflow
pub fn safe_multiply(a: f64, b: f64, default: f64) -> f64 {
    let result = a * b;
    safe_f64(result, default)
}

/// Safe addition that prevents overflow
pub fn safe_add(a: f64, b: f64, default: f64) -> f64 {
    let result = a + b;
    safe_f64(result, default)
}

/// Custom serializer for f64 that converts invalid values to defaults
pub fn serialize_safe_f64<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let safe_value = safe_f64(*value, 0.7); // Default to 0.7 for most consciousness values
    safe_value.serialize(serializer)
}

/// Custom deserializer for f64 that handles null values gracefully
pub fn deserialize_safe_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    match Option::<f64>::deserialize(deserializer)? {
        Some(value) => Ok(safe_f64(value, 0.7)),
        None => {
            eprintln!("Warning: null value encountered, using default 0.7");
            Ok(0.7)
        }
    }
}

/// Clamp value to a safe range
pub fn clamp_f64(value: f64, min: f64, max: f64) -> f64 {
    let safe_val = safe_f64(value, (min + max) / 2.0);
    safe_val.max(min).min(max)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_f64() {
        assert_eq!(safe_f64(1.5, 0.0), 1.5);
        assert_eq!(safe_f64(f64::NAN, 2.0), 2.0);
        assert_eq!(safe_f64(f64::INFINITY, 3.0), 3.0);
    }
    
    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0, 1.0), 5.0);
        assert_eq!(safe_divide(10.0, 0.0, 1.0), 1.0);
        assert_eq!(safe_divide(10.0, f64::INFINITY, 1.0), 1.0);
    }
}