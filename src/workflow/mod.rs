use std::collections::HashMap;

use serde::Deserialize;

use crate::common::{BoE, Env};

pub mod event;
pub mod job;

/// A single GitHub Actions workflow.
///
/// See: <https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions>
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Workflow {
    pub name: Option<String>,
    pub run_name: Option<String>,
    pub on: Trigger,
    #[serde(default)]
    pub permissions: Permissions,
    #[serde(default)]
    pub env: Env,
    pub defaults: Option<Defaults>,
    pub concurrency: Option<Concurrency>,
    pub jobs: HashMap<String, Job>,
}

/// The triggering condition or conditions for a workflow.
///
/// Workflow triggers take three forms:
///
/// 1. A single webhook event name:
///
///     ```yaml
///     on: push
///     ```
/// 2. A list of webhook event names:
///
///     ```yaml
///     on: [push, fork]
///     ```
///
/// 3. A mapping of event names with (optional) configurations:
///
///     ```yaml
///     on:
///       push:
///         branches: [main]
///       pull_request:
///     ```
#[derive(Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Trigger {
    BareEvent(event::BareEvent),
    BareEvents(Vec<event::BareEvent>),
    Events(event::Events),
}

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

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Defaults {
    pub run: Option<RunDefaults>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunDefaults {
    pub shell: Option<String>,
    pub working_directory: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Concurrency {
    pub group: String,
    #[serde(default)]
    pub cancel_in_progress: BoE,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Job {
    NormalJob(job::NormalJob),
    ReusableWorkflowCallJob(job::ReusableWorkflowCallJob),
}

#[cfg(test)]
mod tests {
    use crate::workflow::ExplicitPermissions;

    use super::Permissions;

    #[test]
    fn test_permissions() {
        assert_eq!(
            serde_yaml::from_str::<Permissions>("read-all").unwrap(),
            Permissions::Base(crate::workflow::BasePermission::ReadAll)
        );

        let perm = "security-events: write";
        assert!(matches!(
            serde_yaml::from_str::<ExplicitPermissions>(perm),
            Ok(_)
        ));
    }
}
