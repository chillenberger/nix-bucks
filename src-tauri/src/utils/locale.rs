use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Debug, Clone, Copy)]
pub enum Locale {
    English,
    Spanish,
}

impl Serialize for Locale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(String::from(*self).as_str())
    }
}

impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.into())
    }
}

impl From<Locale> for String {
    fn from(value: Locale) -> Self {
        match value {
            Locale::English => String::from("en"),
            Locale::Spanish => String::from("es"),
        }
    }
}

impl Into<Locale> for String {
    fn into(self) -> Locale {
        match self.as_str() {
            "en" => Locale::English,
            "es" => Locale::Spanish,
            _ => Locale::English,
        }
    }
}
