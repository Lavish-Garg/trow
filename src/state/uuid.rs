pub fn scratch_path(uuid: &str) -> String {
    warn!("Deprecated, please use the recommended function");
    format!("data/scratch/{}", uuid)
}