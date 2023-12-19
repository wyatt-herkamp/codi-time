use std::str::FromStr;
pub mod api_token;
pub mod bio;
pub mod group;
pub mod preferences;
pub mod public_user;
pub mod report_intervals;
use thiserror::Error;
use utoipa::{
    openapi::{schema::Schema, ObjectBuilder, RefOr},
    ToSchema,
};

macro_rules! new_user_type {
    ($struct_name:ident,
        $error_message:literal,
        $error:ident) => {
        #[derive(Debug, Error)]
        #[error($error_message)]
        pub struct $error(&'static str);

        #[derive(Debug, Clone, Hash, Eq, PartialOrd, Ord)]
        pub struct $struct_name(String);

        impl Into<String> for $struct_name {
            fn into(self) -> String {
                self.0
            }
        }
        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
        impl std::ops::Deref for $struct_name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<S: AsRef<str>> PartialEq<S> for $struct_name {
            fn eq(&self, other: &S) -> bool {
                <Self as AsRef<str>>::as_ref(self) == other.as_ref()
            }
        }
        impl From<$struct_name> for sea_orm::Value {
            fn from(value: $struct_name) -> Self {
                sea_orm::Value::String(Some(Box::new(value.0)))
            }
        }
        impl digestible::Digestible for $struct_name {
            fn digest<B: digestible::byteorder::ByteOrder, D: digestible::DigestWriter>(
                &self,
                digest: &mut D,
            ) {
                self.0.digest::<B, D>(digest);
            }
        }
    };
    ($struct_name:ident,
        $error_message:literal,
        $error:ident,
        pub fn sanitize($sanitize_p:ident: &str) -> String {
         $sanitize:stmt
        },
        pub fn validate($validate_p:ident: &str) -> Result<(), $error_result:ty>
            $validate:block

    ) => {
        new_user_type!($struct_name, $error_message, $error);
        impl $struct_name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, $error> {
                let value = Self::sanitize(value.as_ref());
                Self::validate(&value)?;
                Ok(Self(value))
            }
            pub fn sanitize($sanitize_p: &str) -> String {
                $sanitize
            }
            pub fn validate($validate_p: &str) -> Result<(), $error_result> {
                $validate
            }
        }

        impl TryFrom<String> for $struct_name {
            type Error = $error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                <$struct_name>::new(value)
            }
        }

        impl FromStr for $struct_name {
            type Err = $error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$struct_name>::new(s)
            }
        }

        impl sea_orm::sea_query::ValueType for $struct_name {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                match v {
                    sea_orm::Value::String(Some(s)) => {
                        Ok(Self::new(*s).map_err(|_| sea_orm::sea_query::ValueTypeErr)?)
                    }
                    _ => Err(sea_orm::sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($struct_name).to_owned()
            }

            fn array_type() -> sea_orm::sea_query::ArrayType {
                sea_orm::sea_query::ArrayType::String
            }

            fn column_type() -> sea_orm::ColumnType {
                sea_orm::ColumnType::Text
            }
        }
        impl sea_orm::TryGetable for $struct_name {
            fn try_get_by<I: sea_orm::ColIdx>(
                res: &sea_orm::QueryResult,
                index: I,
            ) -> Result<Self, sea_orm::TryGetError> {
                let value = sea_orm::TryGetable::try_get_by(res, index)?;
                <Self as TryFrom<String>>::try_from(value)
                    .map_err(|v| sea_orm::TryGetError::DbErr(sea_orm::DbErr::Type(v.to_string())))
            }
        }
        impl sea_orm::sea_query::Nullable for $struct_name {
            fn null() -> sea_orm::Value {
                sea_orm::Value::String(None)
            }
        }
        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }
        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                $struct_name::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}
new_user_type!(
    Location,
    "Invalid location: {0}",
    InvalidLocation,
    pub fn sanitize(value: &str) -> String {
        value.trim().to_owned()
    },
    pub fn validate(value: &str) -> Result<(), InvalidLocation> {
        tzdb::raw_tz_by_name(value)
            .ok_or(InvalidLocation("Invalid Time Zone."))
            .map(|_| ())
    }
);

impl Default for Location {
    fn default() -> Self {
        Self("Etc/UTC".into())
    }
}

impl<'s> ToSchema<'s> for Location {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Location",
            ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::String)
                .description(Some("A valid IANA Time Zone."))
                .example(Some("America/New_York".into()))
                .into(),
        )
    }
}

new_user_type!(
    Username,
    "Invalid username: {0}",
    InvalidUsername,
    pub fn sanitize(value: &str) -> String {
        value.trim().to_owned()
    },
    pub fn validate(value: &str) -> Result<(), InvalidUsername> {
        if value.len() < 3 {
            return Err(InvalidUsername(
                "Username must be at least 3 characters long.",
            ));
        }
        if value.len() > 16 {
            return Err(InvalidUsername(
                "Username must be at most 16 characters long.",
            ));
        }
        if !value.chars().any(|c| !c.is_numeric()) {
            return Err(InvalidUsername(
                "Username must contain at least one non-numeric character.",
            ));
        }
        if value.chars().any(|c| !c.is_alphanumeric()) {
            return Err(InvalidUsername(
                "Username must only contain alphanumeric characters.",
            ));
        }
        Ok(())
    }
);
impl<'s> ToSchema<'s> for Username {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Username",
            ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::String)
                .min_length(Some(3))
                .max_length(Some(16))
                .format(Some(utoipa::openapi::SchemaFormat::Custom(
                    "An Alphanumeric String. At least 3 characters in length and no more than 16 characters.".to_owned(),
                )))
                .description(Some("A username. Will be unique."))
                .example(Some("KingTux".into()))
                .into(),
        )
    }
}
new_user_type!(
    Email,
    "Invalid email: {0}",
    InvalidEmail,
    pub fn sanitize(value: &str) -> String {
        value.trim().to_owned()
    },
    pub fn validate(value: &str) -> Result<(), InvalidEmail> {
        if value.len() < 3 {
            return Err(InvalidEmail("Email must be at least 3 characters long."));
        }
        if value.len() > 320 {
            return Err(InvalidEmail("Email must be at most 320 characters long."));
        }
        if !value.contains('@') {
            return Err(InvalidEmail("Email must contain an @."));
        }
        Ok(())
    }
);
impl<'s> ToSchema<'s> for Email {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Email",
            ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::String)
                .min_length(Some(3))
                .max_length(Some(320))
                .description(Some("A valid RFC 5322 email address."))
                .example(Some("example@gmail.com".into()))
                .into(),
        )
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn location_tests() {
        use super::Location;
        assert!(Location::new("America/New_York").is_ok());
        assert!(Location::new("America/Not_A_Real_Time_Zone").is_err());
        assert!(Location::new("Etc/UTC").is_ok());
    }
}
