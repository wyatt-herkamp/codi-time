use std::borrow::Cow;

use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LanguageError {
    #[error("Invalid JSON")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Invalid Name")]
    InvalidName,
    #[error("At least one file name or extension must be provided")]
    InvalidFileNames,
    #[error("Missing File {0}")]
    MissingFile(Cow<'static, str>),
}
#[derive(Debug, RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/languages"]
pub struct LanguageFiles;

impl LanguageFiles {
    pub fn get_language_files() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
        Self::iter().filter(|f| f.ends_with("languages.json"))
    }
    pub fn get_category_files() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
        Self::iter().filter(|f| f.ends_with("categories.json"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LanguageCategory {
    /// Name of the Category
    pub name: String,
    /// Category
    pub description: Option<String>,
}
/// Default Languages are located in the `languages` folder
/// These language definitions are used to help determine the language in use. And provide colors and default mappings for the language.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LanguageDef {
    /// Name of the Language
    pub name: String,
    /// Default Color for the Language
    pub default_color: Option<String>,
    /// Categories of the Language
    #[serde(with = "vec_serializer_deserializer", default)]
    pub categories: Vec<String>,
    /// Alias Names for the Language
    #[serde(with = "vec_serializer_deserializer", default)]
    pub aliases: Vec<String>,
    /// File Extensions for the Language
    #[serde(with = "vec_serializer_deserializer", default)]
    pub extensions: Vec<String>,
    /// File Names for the Language
    #[serde(with = "vec_serializer_deserializer", default)]
    pub file_names: Vec<String>,
}

impl LanguageDef {
    /// Loads the language definitions from the `languages` folder
    pub fn load_languages() -> Result<Vec<LanguageDef>, LanguageError> {
        let mut language_files = LanguageFiles::get_language_files();
        // Load the first language file
        let first_file = language_files
            .next()
            .ok_or_else(|| LanguageError::MissingFile("languages.json".into()))?;
        let file = LanguageFiles::get(&first_file)
            .ok_or_else(|| LanguageError::MissingFile(first_file))?;
        // Parse the first language file
        let mut language_defs: Vec<LanguageDef> = serde_json::from_slice(file.data.as_ref())?;
        // Loop through the rest of the language files
        for file_name in language_files {
            let file = LanguageFiles::get(&file_name)
                .ok_or_else(|| LanguageError::MissingFile(file_name.clone()))?;
            let languages: Vec<LanguageDef> = serde_json::from_slice(&file.data)?;
            for language in &languages {
                language.is_valid()?;
            }
            language_defs.extend(languages);
        }
        Ok(language_defs)
    }

    /// Validates the language definition
    ///
    /// Checks that the name is not empty
    /// Checks that at least one file name or extension is provided
    pub fn is_valid(&self) -> Result<(), LanguageError> {
        if self.name.is_empty() {
            return Err(LanguageError::InvalidName);
        }
        if self.file_names.is_empty() && self.extensions.is_empty() {
            return Err(LanguageError::InvalidFileNames);
        }
        Ok(())
    }
}
/// Accepts a List or a single element into a Vec
mod vec_serializer_deserializer {

    use serde::{ser::SerializeSeq, Deserializer, Serializer};

    pub fn serialize<S>(vec: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if vec.is_empty() {
            return serializer.serialize_none();
        } else if vec.len() == 1 {
            return serializer.serialize_str(&vec[0]);
        }
        let mut seq = serializer.serialize_seq(Some(vec.len()))?;
        for e in vec {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
    struct VecOrElementVisitor;
    impl<'de> serde::de::Visitor<'de> for VecOrElementVisitor {
        type Value = Vec<String>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence or a single element")
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(1));
            while let Some(e) = seq.next_element()? {
                vec.push(e);
            }
            Ok(vec)
        }
        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![s.to_owned()])
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![v])
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![v.to_owned()])
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![])
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(VecOrElementVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{LanguageDef, LanguageFiles};
    use crate::language::LanguageCategory;
    #[test]
    pub fn load_languages() {
        LanguageDef::load_languages().expect("Failed to load languages");
    }

    #[test]
    pub fn load_categories() {
        let mut categories = Vec::new();
        for file_name in LanguageFiles::get_category_files() {
            let file = LanguageFiles::get(&file_name).expect("Failed to get file");
            let languages: Vec<LanguageCategory> = serde_json::from_slice(&file.data)
                .expect(format!("Failed to parse {:?}", file_name).as_str());
            categories.extend(languages);
        }

        println!("Loaded {} Categories", categories.len());
    }
}
