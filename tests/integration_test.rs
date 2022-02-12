use assert_cmd::Command;

#[test]
fn help_flag() {
    let mut cmd = Command::cargo_bin("pushover_cli").unwrap();
    let output = r#"pushover_cli 0.1.0
Send notifications using Pushover

USAGE:
    pushover_cli --user <USER> --token <TOKEN> --message <MESSAGE>

OPTIONS:
    -h, --help                 Print help information
    -m, --message <MESSAGE>    Message to send
    -t, --token <TOKEN>        Pushover token
    -u, --user <USER>          Pushover user
    -V, --version              Print version information
"#;

    let assert = cmd.arg("--help").assert();
    assert.success().code(0).stdout(output);

    let assert = cmd.arg("-h").assert();
    assert.success().code(0).stdout(output);
}

#[test]
fn version_flag() {
    let mut cmd = Command::cargo_bin("pushover_cli").unwrap();
    let output = "pushover_cli 0.1.0\n";

    let assert = cmd.arg("--version").assert();
    assert.success().code(0).stdout(output);

    let assert = cmd.arg("-V").assert();
    assert.success().code(0).stdout(output);
}
