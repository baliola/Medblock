mod registry;

#[ic_cdk::query]
fn dummy() {}

ic_cdk::export_candid!();
