struct alert {
    pub alert: Alert,
    pub operation: Operation,
    pub severity_and_threshold: HashMap<Severity, i32>,
}

fn parse_alert(alert: Alert) -> alert {
    match alert {
        Alert::Replicas(Operation) | Alert::Errors(Operation) | Alert::Traffic(Operation) => {
            let severity_and_threshold = match_operation(Operation);
            alert {
                alert,
                operation,
                severity_and_threshold,
            }
        }
        Alert::Latency(HistogramPercentiles) => {
            let operation = match_histogram_percentile(HistogramPercentiles);
            let severity_and_threshold = match_operation(operation);
            alert {
                alert,
                operation,
                severity_and_threshold,
            }
        }
    }
}

fn match_histogram_percentile(histogram_percentile: HistogramPercentile) -> (Operation) {
    match histogram_percentile {
        HistogramPercentile::P50(Operation) => Operation,
        HistogramPercentile::P90(Operation) => Operation,
        HistogramPercentile::P95(Operation) => Operation,
        HistogramPercentile::P99(Operation) => Operation,
    }
}

fn match_operation(operation: Operation) -> (HashMap<Severity, i32>) {
    match operation {
        Operation::MoreThan(hash_map) => hash_map,
        Operation::LessThan(hash_map) => hash_map,
        Operation::EqualTo(hash_map) => hash_map,
    }
}

// TODO: common labels
// TODO: histogram percentiles
// TODO: multiple operations
pub fn newAlert(alert_config: alert, alertFiringDuration: &str, summary: &str, description: &str) -> String {
    let initial_string = String::from("");

    for (key, value) in alert_config.severity_and_threshold {
        let template = Template::new(
            "\
- alert:
    expr: {{metric}} {{operation}} {{threshold}}
    for: {{alertFiringDuration}}
    labels:
      severity: {severity}
    annotations:
      summary: {{summary}}
      description: {{description}}
",
        );
        let parameters = HashMap::from([
            ("metric", alert_config.alert),
            ("operation", alert_config.operation),
            ("threshold", value),
            ("alertFiringDuration", alertFiringDuration),
            ("severity", key),
            ("summary", summary),
            ("description", description),
        ]);
        let rendered_text = template.render(&parameters).unwrap();
        initial_string.push_str(rendered_text);
    }


    return string;
}
