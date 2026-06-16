use std::time::Duration;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "obscura-check-update",
    version = env!("OBSCURA_BUILD_VERSION"),
    about = "Check GitHub Releases for a newer Obscura release"
)]
struct Args {
    /// Emit machine-readable JSON instead of text.
    #[arg(long)]
    json: bool,

    /// Repository to check, in owner/name form. Defaults to OBSCURA_UPDATE_REPO or h4ckf0r0day/obscura.
    #[arg(long)]
    repo: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run_check_update(args.json, args.repo).await
}

async fn run_check_update(json_output: bool, repo_arg: Option<String>) -> anyhow::Result<()> {
    let current = env!("OBSCURA_BUILD_VERSION");
    let repo = repo_arg
        .or_else(|| std::env::var("OBSCURA_UPDATE_REPO").ok())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "h4ckf0r0day/obscura".to_string());
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);

    let client = reqwest::Client::builder()
        .user_agent(format!("obscura/{} update-check", current))
        .timeout(Duration::from_secs(10))
        .build()?;

    let resp = client.get(&url).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("GitHub update check failed: HTTP {}", resp.status());
    }

    let body: serde_json::Value = resp.json().await?;
    let latest = body
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches("obscura-v")
        .trim_start_matches('v')
        .to_string();
    let html_url = body
        .get("html_url")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let update_available = !latest.is_empty() && latest != current;

    if json_output {
        println!(
            "{}",
            serde_json::json!({
                "current": current,
                "latest": latest,
                "updateAvailable": update_available,
                "releaseUrl": html_url,
                "repository": repo,
            })
        );
    } else if update_available {
        println!("Update available: {} -> {}", current, latest);
        if !html_url.is_empty() {
            println!("Release: {}", html_url);
        }
        println!("Download and verify the release manually; this command never auto-replaces binaries.");
    } else {
        println!("Obscura is up to date ({})", current);
    }

    Ok(())
}
