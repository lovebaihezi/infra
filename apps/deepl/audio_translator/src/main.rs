use infra_utils::{anyhow, trace::Trace};

fn main() -> anyhow::Result<()> {
    let _trace = Trace::init();
    Ok(())
}
