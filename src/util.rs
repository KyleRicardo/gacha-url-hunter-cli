pub fn pause() {
    use std::process::Command;
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}