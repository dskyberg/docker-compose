use serde::Deserialize;
use std::str::FromStr;
use void::Void;

pub type Args = std::collections::HashMap<String, String>;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Build {
    pub context: String,
    pub dockerfile: Option<String>,
    #[serde(default)]
    pub args: Args,
}

impl FromStr for Build {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Build {
            context: s.to_string(),
            dockerfile: None,
            args: Args::new(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::build::Build;

    #[test]
    fn test_without_build() {
        let s = r#"---
        context: busybox
        "#;

        let yaml: Build = serde_yaml::from_str(s).unwrap();
        dbg!(&yaml);
    }
}
