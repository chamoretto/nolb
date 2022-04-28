// #[cfg(tests)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_version() {
        let mut cmd = Command::cargo_bin("nolb-cli").unwrap();
        let assert = cmd.arg("--version").assert();
        assert.success().code(0).stdout("nolb 0.1.0\n");
    }

    // TODO: сделать вариант когда предлагает верно, и когда не хватает очков для верного предложения (отдельно)
    // #[test]
    // fn test_version_with_typo() {
    //     let mut cmd = Command::cargo_bin("nolb-cli").unwrap();
    //     let assert = cmd.arg("--vsion").assert();
    //     assert.success().code(0).stdout("nolb 0.1.0\n");
    // }
}
