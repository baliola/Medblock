mod registry;
mod memory;
#[ic_cdk::query]
fn dummy() {}

ic_cdk::export_candid!();
