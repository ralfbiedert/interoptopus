use backend_csharp_ng::Pipeline;
use interoptopus::inventory::Inventory;

#[test]
fn can_run_pipeline() {
    let inventory = Inventory::new();

    Pipeline::new().execute(&inventory);
}
