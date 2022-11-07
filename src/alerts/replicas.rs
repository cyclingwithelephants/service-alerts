use alert::newAlert;
use kinds::service_alerts::{Alert, Operation};

const SUMMARY: &str = "A brief summary";
const DESCRIPTION: &str = "A more detailed description";
const ALERT_FIRING_DURATION: &str = "2m"; // seconds
const METRIC: String = "up\u{007B}{{serviceSelector}}\u{007D}"; // looks like up{service="my-service"}

fn newReplicaAlerts(alert_config: Alert) -> String {
    let initial_string = String::from("");

    newAlert(alert_config, ALERT_FIRING_DURATION, SUMMARY, DESCRIPTION)
}
