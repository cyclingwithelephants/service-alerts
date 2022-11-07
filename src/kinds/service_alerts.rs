use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use kube::CustomResource;


/// Identifier that is recorded by the Kubernetes API for the purpose of
/// identifying the application responsible for the given Kubernetes resource.
const MANAGER_STRING: &str = "cactuar";

pub const API_GROUP: &str = "cactuar.rs";
pub const API_VERSION: &str = "v1alpha1";
pub const KIND: &str = "ServiceAlerter";
pub const FINALIZER_NAME: &str = "servicealerter.cactuar.rs";

/// CustomResourceDefinition name for the ServiceAlerter type, the FQDN (Fully
/// Qualified Domain Name) serves as a way to namespace custom resources in
/// Kubernetes.
const CUSTOM_RESOURCE_NAME: &str = "servicealerters.cactuar.rs";

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[kube(
    group = "cactuar.rs",
    version = "v1alpha1",
    kind = "ServiceAlerter",
    namespaced,
    status = "ServiceAlerterStatus",
    shortname = "servalert"
)]
pub struct ServiceAlerterSpec {
    pub common_labels: HashMap<String, String>,
    pub service_selector: ServiceSelector,
    pub alerts: Vec<Alert>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSelector {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Alert {
    Replicas(Operation),
    Errors(Operation),
    Traffic(Operation),
    Latency(HistogramPercentiles),
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    MoreThan(HashMap<Severity, i32>),
    LessThan(HashMap<Severity, i32>),
    EqualTo(HashMap<Severity, i32>),
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum HistogramPercentiles {
    // I'm lazy, need to generate P1-P99
    P50(Operation),
    P90(Operation),
    P95(Operation),
    P99(Operation),
}
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    Warning,
    Critical,
}


/// The status object of `StatusAlerter`
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAlerterStatus {
    // pub reconciled: bool,
    pub last_reconciled_at: Option<String>,
    pub reconciliation_expires_at: Option<String>,
}
