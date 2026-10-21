use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

struct Sandbox(PathBuf);
impl Sandbox {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "linear-directory-{}-{}",
            std::process::id(),
            rand::random::<u64>()
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn run(&self, cwd: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_linear"))
            .args(args)
            .current_dir(cwd)
            .env("HOME", &self.0)
            .env("XDG_CONFIG_HOME", &self.0)
            .env_remove("LINEAR_CLI_PROFILE")
            .env_remove("LINEAR_API_KEY")
            .env_remove("LINEAR_CLI_TEAM")
            .env("LINEAR_CLI_OUTPUT", "json")
            .output()
            .unwrap()
    }
    fn ok(&self, cwd: &Path, args: &[&str]) -> String {
        let out = self.run(cwd, args);
        assert!(
            out.status.success(),
            "{:?}: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    }
    fn setup(&self) {
        // Login consumes stdin; these fixture keys are not real credentials.
        for profile in ["personal", "work", "client"] {
            self.login(
                &self.0,
                &["--profile", profile, "auth", "login"],
                &format!("fake-{profile}-credential\n"),
            );
        }
    }
    fn login(&self, cwd: &Path, args: &[&str], key: &str) {
        use std::io::Write;
        let mut child = Command::new(env!("CARGO_BIN_EXE_linear"))
            .args(args)
            .env("HOME", &self.0)
            .env("XDG_CONFIG_HOME", &self.0)
            .env_remove("LINEAR_API_KEY")
            .env_remove("LINEAR_CLI_PROFILE")
            .current_dir(cwd)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(key.as_bytes())
            .unwrap();
        assert!(child.wait().unwrap().success());
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn directory_profiles_inherit_override_unbind_and_report_source() {
    let s = Sandbox::new();
    s.setup();
    let parent = s.0.join("work");
    let child = parent.join("repo/src");
    fs::create_dir_all(&child).unwrap();
    s.ok(&parent, &["config", "bind", "work"]);
    s.ok(&parent, &["config", "set", "default-team", "RCM"]);
    assert_eq!(
        s.ok(&child, &["config", "get", "default-team"]).trim(),
        "RCM"
    );
    assert_eq!(
        s.ok(&child, &["config", "get", "api-key", "--raw"]).trim(),
        "fake-work-credential"
    );
    assert_eq!(s.ok(&child, &["config", "get", "profile"]).trim(), "work");
    let status = s.ok(&child, &["auth", "status", "--output", "json"]);
    let status: serde_json::Value = serde_json::from_str(&status).unwrap();
    assert_eq!(status["profile"], "work");
    assert!(status["profile_source"].as_str().unwrap().contains("work"));
    assert_eq!(
        s.ok(
            &child,
            &["--profile", "personal", "config", "get", "profile"]
        )
        .trim(),
        "personal"
    );
    s.ok(&child, &["config", "bind", "client"]);
    assert_eq!(s.ok(&child, &["config", "get", "profile"]).trim(), "client");
    assert!(s.ok(&child, &["config", "bindings"]).contains("client"));
    s.ok(&child, &["config", "unbind"]);
    assert_eq!(s.ok(&child, &["config", "get", "profile"]).trim(), "work");
    s.login(&child, &["auth", "login"], "updated-work-credential\n");
    assert_eq!(
        s.ok(&child, &["config", "get", "api-key", "--raw"]).trim(),
        "updated-work-credential"
    );
    assert_eq!(
        s.ok(
            &child,
            &["--profile", "personal", "config", "get", "api-key", "--raw"]
        )
        .trim(),
        "fake-personal-credential"
    );
    let sibling = s.0.join("work-other");
    fs::create_dir(&sibling).unwrap();
    assert_eq!(
        s.ok(&sibling, &["config", "get", "profile"]).trim(),
        "personal"
    );
}

#[test]
fn invalid_binding_never_falls_back_to_global_profile() {
    let s = Sandbox::new();
    s.setup();
    assert!(!s.run(&s.0, &["config", "bind", "missing"]).status.success());
    s.ok(&s.0, &["config", "bind", "work"]);
    s.ok(&s.0, &["config", "workspace-remove", "work"]);
    for args in [
        &["config", "get", "profile"][..],
        &["auth", "status"][..],
        &["config", "set", "default-team", "WRONG"][..],
    ] {
        let result = s.run(&s.0, args);
        assert!(!result.status.success(), "must not fall back: {:?}", args);
        assert!(String::from_utf8_lossy(&result.stderr).contains("work"));
    }
    // Recovery remains possible even when the selected profile has been removed.
    s.ok(&s.0, &["config", "unbind"]);
    // Recreate a profile without usable credentials, then bind it.
    s.login(&s.0, &["--profile", "empty", "auth", "login"], "fixture\n");
    // Save an empty API key directly in the isolated fixture config (not real auth).
    let cfg = if cfg!(target_os = "macos") {
        s.0.join("Library/Application Support/linear-cli/config.toml")
    } else {
        s.0.join("linear-cli/config.toml")
    };
    let text = fs::read_to_string(&cfg)
        .unwrap()
        .replace("api_key = \"fixture\"", "api_key = \"\"");
    fs::write(cfg, text).unwrap();
    s.ok(&s.0, &["config", "bind", "empty"]);
    let out = s.run(&s.0, &["config", "get", "api-key", "--raw"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("empty"));
}

#[test]
#[cfg(unix)]
fn symlink_paths_select_the_same_directory_binding() {
    let s = Sandbox::new();
    s.setup();
    let actual = s.0.join("actual");
    fs::create_dir(&actual).unwrap();
    let link = s.0.join("link");
    std::os::unix::fs::symlink(&actual, &link).unwrap();
    s.ok(&s.0, &["config", "bind", "work", link.to_str().unwrap()]);
    assert_eq!(s.ok(&actual, &["config", "get", "profile"]).trim(), "work");
    assert_eq!(s.ok(&link, &["config", "get", "profile"]).trim(), "work");
}

#[test]
fn malformed_private_config_errors_instead_of_using_a_default_account() {
    let s = Sandbox::new();
    s.setup();
    let cfg = if cfg!(target_os = "macos") {
        s.0.join("Library/Application Support/linear-cli/config.toml")
    } else {
        s.0.join("linear-cli/config.toml")
    };
    fs::write(cfg, "current = [ malformed").unwrap();
    assert!(!s.run(&s.0, &["config", "get", "profile"]).status.success());
    assert!(!s.run(&s.0, &["auth", "status"]).status.success());
}

#[test]
fn explicit_api_key_overrides_profile_oauth_in_auth_status() {
    let s = Sandbox::new();
    s.setup();
    let cfg = if cfg!(target_os = "macos") {
        s.0.join("Library/Application Support/linear-cli/config.toml")
    } else {
        s.0.join("linear-cli/config.toml")
    };
    let mut content = fs::read_to_string(&cfg).unwrap();
    content.push_str("\n[workspaces.personal.oauth]\nclient_id = 'fixture'\naccess_token = 'fake-oauth-token'\ntoken_type = 'Bearer'\n");
    fs::write(cfg, content).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_linear"))
        .args(["auth", "status", "--output", "json"])
        .current_dir(&s.0)
        .env("HOME", &s.0)
        .env("XDG_CONFIG_HOME", &s.0)
        .env_remove("LINEAR_CLI_PROFILE")
        .env("LINEAR_API_KEY", "explicit-fixture-key")
        .output()
        .unwrap();
    assert!(output.status.success());
    let status: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(status["env_api_key"], true);
    assert_eq!(status["auth_type"], "api_key");
    assert!(!String::from_utf8_lossy(&output.stdout).contains("explicit-fixture-key"));
}

#[test]
fn worktree_inherits_original_checkout_binding_unless_locally_bound() {
    let s = Sandbox::new();
    s.setup();
    let repo = s.0.join("company/repo");
    fs::create_dir_all(&repo).unwrap();
    let git = |args: &[&str]| {
        assert!(Command::new("git")
            .current_dir(&repo)
            .args(args)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .output()
            .unwrap()
            .status
            .success());
    };
    git(&["init"]);
    let tree = s.0.join("outside/tree");
    git(&[
        "worktree",
        "add",
        "--orphan",
        "-b",
        "fixture",
        tree.to_str().unwrap(),
    ]);
    s.ok(repo.parent().unwrap(), &["config", "bind", "work"]);
    assert_eq!(s.ok(&tree, &["config", "get", "profile"]).trim(), "work");
    s.ok(&tree, &["config", "bind", "client"]);
    assert_eq!(s.ok(&tree, &["config", "get", "profile"]).trim(), "client");
}
