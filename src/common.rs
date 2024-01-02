use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Permissions {
    Base(BasePermission),
    Explicit(ExplicitPermissions),
}

impl Default for Permissions {
    fn default() -> Self {
        Self::Base(BasePermission::Default)
    }
}

#[derive(Deserialize, Default, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BasePermission {
    /// Whatever default permissions come from the workflow's `GITHUB_TOKEN`.
    #[default]
    Default,
    ReadAll,
    WriteAll,
}

#[derive(Deserialize, Default, Debug, PartialEq)]
#[serde(rename_all = "kebab-case", default)]
pub struct ExplicitPermissions {
    pub actions: Permission,
    pub checks: Permission,
    pub contents: Permission,
    pub deployments: Permission,
    pub id_token: Permission,
    pub issues: Permission,
    pub discussions: Permission,
    pub packages: Permission,
    pub pages: Permission,
    pub pull_requests: Permission,
    pub repository_projects: Permission,
    pub security_events: Permission,
    pub statuses: Permission,
}

#[derive(Deserialize, Default, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    Read,
    Write,
    #[default]
    None,
}

pub type Env = HashMap<String, EnvValue>;

/// Environment variable values are always strings, but GitHub Actions
/// allows users to configure them as various native YAML types before
/// internal stringification.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum EnvValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl ToString for EnvValue {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Boolean(b) => b.to_string(),
        }
    }
}

/// A "literal or expr" type, for places in GitHub Actions where a
/// key can either have a literal value (array, object, etc.) or an
/// expression string.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoE<T> {
    Literal(T),
    Expr(String),
}

impl<T> Default for LoE<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Literal(T::default())
    }
}

pub type BoE = LoE<bool>;

/// A "scalar or vector" type, for places in GitHub Actions where a
/// key can have either a scalar value or an array of values.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SoV<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> From<Vec<T>> for SoV<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Many(value)
    }
}

impl<T> From<T> for SoV<T> {
    fn from(value: T) -> Self {
        Self::One(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{BasePermission, ExplicitPermissions};

    use super::Permissions;

    #[test]
    fn test_permissions() {
        assert_eq!(
            serde_yaml::from_str::<Permissions>("read-all").unwrap(),
            Permissions::Base(BasePermission::ReadAll)
        );

        let perm = "security-events: write";
        assert!(matches!(
            serde_yaml::from_str::<ExplicitPermissions>(perm),
            Ok(_)
        ));
    }
}
