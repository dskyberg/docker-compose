use serde::Deserialize;

use crate::build::Build;
use crate::serde_utils::opt_string_or_struct::opt_string_or_struct;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Service {
    pub image: Option<String>,
    #[serde(default, deserialize_with = "opt_string_or_struct")]
    pub build: Option<Build>,
}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::build::Build;

    #[test]
    fn test_without_build() {
        let s = r#"---
        image: busybox
        "#;

        let yaml: Service = serde_yaml::from_str(s).unwrap();
        dbg!(&yaml);
    }

    #[test]
    fn test_with_build_string() {
        let s = r#"---
        image: busybox
        build: build_ctx
        "#;

        let yaml: Service = serde_yaml::from_str(s).unwrap();
        dbg!(&yaml);
    }
    
    #[test]
    fn test_with_build_struct() {
        let s = r#"---
        image: busybox
        build: 
            context: build_ctx
        "#;

        let yaml: Service = serde_yaml::from_str(s).unwrap();
        dbg!(&yaml);
    }
}
