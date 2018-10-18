use juniper::{FieldError, IntoFieldError};

pub enum InputError {
    ColorValidationError,
    RarityValidationError,
}

impl IntoFieldError for InputError {
    fn into_field_error(self) -> FieldError {
        match self {
            InputError::ColorValidationError => FieldError::new(
                "The input color is not a possible value",
                graphql_value!({
                    "type": "COLOR NOT FOUND"
                }),
            ),
            InputError::RarityValidationError => FieldError::new(
                "The input rarity is not a possible value",
                graphql_value!({
                    "type": "RARITY NOT FOUND"
                }),
            ),
        }
    }
}
