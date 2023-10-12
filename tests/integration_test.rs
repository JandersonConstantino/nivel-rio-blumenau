use std::{process::Command, time::Duration};
use tokio::time::timeout;

#[tokio::test]
async fn should_return_only_first_item() {
    let mut cmd = Command::new("nivel-rio-blumenau")
        .arg("-u")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    let timeout_duration = Duration::from_secs(5);

    let result = timeout(timeout_duration, async {
        let status = cmd.wait().expect("Timeout exceeded");
        assert!(status.success());
    })
    .await;

    assert!(result.is_ok(), "Timeout exceeded");

    let output = cmd.wait_with_output().expect("Failed to read output");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(stdout.lines().count(), 1);
    assert_eq!(stderr, "");
}

#[tokio::test]
async fn should_return_all_data() {
    let mut cmd = Command::new("nivel-rio-blumenau")
        .args(["-u", "-r"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    let timeout_duration = Duration::from_secs(5);

    let result = timeout(timeout_duration, async {
        let status = cmd.wait().expect("Timeout exceeded");
        assert!(status.success());
    })
    .await;

    assert!(result.is_ok(), "Timeout exceeded");

    let output = cmd.wait_with_output().expect("Failed to read output");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stdout.lines().count() > 20);
    assert_eq!(stderr, "");
}
