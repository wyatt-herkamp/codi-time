use serde::Serialize;
use thiserror::Error;
use utoipa::{
    openapi::{ObjectBuilder, RefOr, SchemaType},
    ToSchema,
};
#[derive(Clone, Debug, PartialEq, Error)]
#[error("Invalid Ordering. Expected 'a' | 'd' | \"ascending\" | \"descending\" | 0 | 1")]
pub struct InvalidOrdering;
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum QueryOrdering {
    Ascending,
    Descending,
}
#[cfg(feature = "sea-orm")]
impl Into<sea_orm::Order> for QueryOrdering {
    fn into(self) -> sea_orm::Order {
        match self {
            Self::Ascending => sea_orm::Order::Asc,
            Self::Descending => sea_orm::Order::Desc,
        }
    }
}
impl Default for QueryOrdering {
    fn default() -> Self {
        Self::Ascending
    }
}

impl<'a> ToSchema<'a> for QueryOrdering {
    fn schema() -> (&'a str, RefOr<utoipa::openapi::schema::Schema>) {
        (
            "Ordering",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .enum_values::<[&str; 4usize], &str>(Some(["ascending", "descending", "a", "d"]))
                .into(),
        )
    }
}
impl TryFrom<String> for QueryOrdering {
    type Error = InvalidOrdering;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "a" | "ascending" | "asc" => Ok(Self::Ascending),
            "d" | "desc" | "descending" => Ok(Self::Descending),
            _ => Err(InvalidOrdering),
        }
    }
}
impl TryFrom<char> for QueryOrdering {
    type Error = InvalidOrdering;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            'a' => Ok(Self::Ascending),
            'd' => Ok(Self::Descending),
            _ => Err(InvalidOrdering),
        }
    }
}
impl TryFrom<u8> for QueryOrdering {
    type Error = InvalidOrdering;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::Ascending),
            0 => Ok(Self::Descending),
            _ => Err(InvalidOrdering),
        }
    }
}
mod _serde {
    use serde::Deserialize;

    use super::QueryOrdering;
    struct OrderingVisitor;
    impl<'de> serde::de::Visitor<'de> for OrderingVisitor {
        type Value = QueryOrdering;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing either 'a' or 'd'")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            QueryOrdering::try_from(v.to_string())
                .map_err(|_| serde::de::Error::custom(format!("Invalid ordering: {}", v)))
        }
        fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            QueryOrdering::try_from(v)
                .map_err(|_| serde::de::Error::custom(format!("Invalid ordering: {}", v)))
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            QueryOrdering::try_from(v)
                .map_err(|_| serde::de::Error::custom(format!("Invalid ordering: {}", v)))
        }
    }

    impl<'de> Deserialize<'de> for QueryOrdering {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(OrderingVisitor)
        }
    }
}
