use controller::kinds::service_alerts::ServiceAlerter;
use kube::CustomResourceExt;
fn main() {
    print!("{}", serde_yaml::to_string(&ServiceAlerter::crd()).unwrap())
}
