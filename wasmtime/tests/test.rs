use anyhow::Result;
use test_programs_artifacts::TEST;

#[tokio::test]
async fn initialize_test_component() -> Result<()> {
    let component = std::fs::read(TEST)?;

    // TODO: execute `component`. it should trap.

    let out = component_init_wasmtime::initialize(&component).await?;

    // TODO: execute `out`. it should not trap.

    Ok(())
}
