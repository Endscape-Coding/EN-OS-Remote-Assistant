use std::process::Command;

pub async fn check_prog(name: &str) -> bool {
    let cmd = Command::new("which")
    .arg(name)
    .output();

    if cmd.expect("Error..?").status.success() {
        log::info!("{} has been installed!", name);
        return true;
    } else {
        log::error!("{} not installed!", name);
        return false;
    }
}
