pub(crate) fn invoke(name_only: bool) -> anyhow::Result<()> {
    anyhow::ensure!(name_only,"name --name-only is only supported on users.");
    Ok(())
}