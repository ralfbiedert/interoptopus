use backend_csharp_ng::ForwardPipeline;
use interoptopus::inventory::Inventory;

#[test]
fn can_run_pipeline() {
    let inventory = Inventory::new();
    let output = ForwardPipeline::new(inventory).process();
}
