use std::collections::HashSet;
use zed_extension_api::{self as zed, *};

const PACKAGE_NAME: &str = "@actions/languageserver";

struct GitHubActionsExtension {
	installed: HashSet<String>,
}

impl GitHubActionsExtension {
	fn binary_path() -> String {
		// Use the official @actions/languageserver binary from node_modules
		// The package provides its own bin/actions-languageserver wrapper
		// npm packages are installed in the current working directory (Zed's work dir)
		std::env::current_dir()
			.unwrap()
			.join("node_modules")
			.join("@actions")
			.join("languageserver")
			.join("bin")
			.join("actions-languageserver")
			.to_string_lossy()
			.to_string()
	}

	fn detect_repository(&self, worktree: &Worktree) -> Vec<serde_json::Value> {
		let root_path = worktree.root_path();
		println!(
			"[gh-actions] Attempting to detect repository at: {}",
			root_path
		);

		// Strategy 1: Use git CLI to get remote URL
		println!(
			"[gh-actions] Executing: git -C {} remote get-url origin",
			root_path
		);
		let mut git_command = zed::Command::new("git")
			.arg("-C")
			.arg(&root_path)
			.arg("remote")
			.arg("get-url")
			.arg("origin");

		// Add environment variables from worktree
		for (key, value) in worktree.shell_env() {
			git_command = git_command.env(key, value);
		}

		match git_command.output() {
			Ok(output) => {
				if let Some(0) = output.status {
					let remote_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
					println!("[gh-actions] Got remote URL from git: {}", remote_url);

					if let Some((owner, name)) = Self::parse_github_url(&remote_url) {
						println!("[gh-actions] Parsed owner: {}, name: {}", owner, name);

						// Try to get additional info from gh CLI if available
						if worktree.which("gh").is_some() {
							println!("[gh-actions] gh CLI available, fetching repo info");
							let repo_slug = format!("{}/{}", owner, name);

							let mut gh_command = zed::Command::new("gh")
								.arg("repo")
								.arg("view")
								.arg(&repo_slug)
								.arg("--json")
								.arg("id,owner");

							for (key, value) in worktree.shell_env() {
								gh_command = gh_command.env(key, value);
							}

							if let Ok(gh_output) = gh_command.output() {
								if let Some(0) = gh_output.status {
									let json_str = String::from_utf8_lossy(&gh_output.stdout);
									if let Ok(repo_info) =
										serde_json::from_str::<serde_json::Value>(&json_str)
									{
										let id = repo_info["id"].as_i64().unwrap_or(0);
										let organization_owned = repo_info["owner"]["type"]
											.as_str() == Some("Organization");

										println!("[gh-actions] Got full repo info from gh CLI");
										return vec![serde_json::json!({
											"id": id,
											"owner": owner,
											"name": name,
											"organizationOwned": organization_owned,
											"workspaceUri": format!("file://{}", root_path)
										})];
									}
								}
							}
						}

						// Return basic info if gh CLI not available or failed
						return vec![serde_json::json!({
							"id": 0,
							"owner": owner,
							"name": name,
							"organizationOwned": false,
							"workspaceUri": format!("file://{}", root_path)
						})];
					}
				} else {
					let stderr = String::from_utf8_lossy(&output.stderr);
					println!("[gh-actions] git command failed: {}", stderr);
				}
			}
			Err(e) => {
				println!("[gh-actions] Failed to execute git command: {}", e);
			}
		}

		// Strategy 2: Check GITHUB_REPOSITORY environment variable
		let shell_env = worktree.shell_env();
		if let Some((_, repo_var)) = shell_env.iter().find(|(key, _)| key == "GITHUB_REPOSITORY") {
			println!("[gh-actions] Found GITHUB_REPOSITORY env var: {}", repo_var);
			if let Some((owner, name)) = repo_var.split_once('/') {
				println!("[gh-actions] Using repository from env: {}/{}", owner, name);
				return vec![serde_json::json!({
					"id": 0,
					"owner": owner,
					"name": name,
					"organizationOwned": false,
					"workspaceUri": format!("file://{}", root_path)
				})];
			}
		}

		println!("[gh-actions] Could not detect repository information");
		vec![]
	}

	fn parse_github_url(url: &str) -> Option<(String, String)> {
		// Handle SSH format: git@github.com:owner/repo.git
		if url.starts_with("git@github.com:") {
			let path = url.trim_start_matches("git@github.com:");
			return Self::parse_github_path(path);
		}

		// Handle HTTPS format: https://github.com/owner/repo.git
		if url.starts_with("https://github.com/") || url.starts_with("http://github.com/") {
			let path = url
				.trim_start_matches("https://github.com/")
				.trim_start_matches("http://github.com/");
			return Self::parse_github_path(path);
		}

		None
	}

	fn parse_github_path(path: &str) -> Option<(String, String)> {
		let parts: Vec<&str> = path.trim_end_matches(".git").split('/').collect();
		if parts.len() >= 2 {
			Some((parts[0].to_string(), parts[1].to_string()))
		} else {
			None
		}
	}

	fn install_package_if_needed(
		&mut self,
		id: &LanguageServerId,
		package_name: &str,
	) -> Result<()> {
		use LanguageServerInstallationStatus::*;
		let installed_version = npm_package_installed_version(package_name)?;

		// If package is already installed in this session, then we won't reinstall it
		if installed_version.is_some() && self.installed.contains(package_name) {
			return Ok(());
		}

		set_language_server_installation_status(id, &CheckingForUpdate);

		let latest_version = npm_package_latest_version(package_name)?;

		if installed_version.as_ref() != Some(&latest_version) {
			println!("Installing {package_name}@{latest_version}...");

			set_language_server_installation_status(id, &Downloading);

			if let Err(error) = npm_install_package(package_name, &latest_version) {
				// If installation failed, but we don't want to error but rather reuse existing version
				if installed_version.is_none() {
					Err(error)?;
				}
			}
		} else {
			println!("Found {package_name}@{latest_version} installed");
		}

		self.installed.insert(package_name.into());
		Ok(())
	}
}

impl Extension for GitHubActionsExtension {
	fn new() -> Self {
		Self {
			installed: HashSet::new(),
		}
	}

	fn language_server_command(
		&mut self,
		language_server_id: &LanguageServerId,
		_worktree: &Worktree,
	) -> Result<Command> {
		self.install_package_if_needed(language_server_id, PACKAGE_NAME)?;

		Ok(Command {
			command: node_binary_path()?,
			args: vec![Self::binary_path(), "--stdio".to_string()],
			env: Default::default(),
		})
	}

	fn language_server_initialization_options(
		&mut self,
		_language_server_id: &LanguageServerId,
		worktree: &Worktree,
	) -> Result<Option<serde_json::Value>> {
		let shell_env = worktree.shell_env();
		let session_token = shell_env
			.iter()
			.find(|(key, _)| key == "GITHUB_TOKEN" || key == "GH_TOKEN")
			.map(|(_, value)| value.clone())
			.unwrap_or_default();

		// Attempt to detect repository information
		let repos = self.detect_repository(worktree);

		let options = serde_json::json!({
			"sessionToken": session_token,
			"experimentalFeatures": {},
			"repos": repos
		});

		println!(
			"[gh-actions] Initialization options: {}",
			serde_json::to_string_pretty(&options).unwrap_or_default()
		);

		Ok(Some(options))
	}
}

register_extension!(GitHubActionsExtension);
