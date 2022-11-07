use std::time::Duration;

// #[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
// #[serde(rename_all = "camelCase")]
// #[kube(
// group = "cactuar.rs",
// version = "v1alpha1",
// kind = "ServiceAlerter",
// namespaced
// )]
pub struct ControllerConfigSpec {
    reconciliation: Reconciliation,
}

pub struct Reconciliation {
    interval: Duration,
    backend: AlertingBackend,
}

pub enum AlertingBackend {
    PrometheusInKubernetesWithAlertManager,
    PrometheusInKubernetesWithPrometheusAlerts,
}
