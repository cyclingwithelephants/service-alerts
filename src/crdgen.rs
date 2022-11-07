use kube::CustomResourceExt;
use controller::kinds::service_alerts::ServiceAlerter;
fn main() {
    print!("{}", serde_yaml::to_string(&ServiceAlerter::crd()).unwrap())
}
