use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

pub fn try_variant_from_value<T, U, E>(value: JsonValue, variant: fn(T) -> U) -> Result<U, E>
where
    T: DeserializeOwned,
    E: serde::de::Error,
{
    serde_json::from_value(value).map(variant).map_err(serde_json_error_to_generic_de_error)
}

pub fn serde_json_error_to_generic_de_error<E: serde::de::Error>(error: serde_json::Error) -> E {
    E::custom(error.to_string())
}

pub fn get_field<T, E>(value: &JsonValue, field: &'static str) -> Result<T, E>
where
    T: DeserializeOwned,
    E: serde::de::Error,
{
    serde_json::from_value(value.get(field).cloned().ok_or_else(|| E::missing_field(field))?)
        .map_err(serde_json_error_to_generic_de_error)
}
