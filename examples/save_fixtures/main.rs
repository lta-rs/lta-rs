mod capture;
mod endpoints;
mod http;

use std::process::ExitCode;
use std::time::Duration;

use argh::FromArgs;

use crate::capture::{Capture, Ctx, Pacer};

/// Collect fresh LTA DataMall fixtures from the live API into tests/fixtures/.
///
/// Every run writes NEW numbered files (`<name>_<N>.json`); existing fixtures are never
/// overwritten, so captures collect up over time.
#[derive(FromArgs)]
struct Args {
    /// print the capture ids and exit
    #[argh(switch)]
    list: bool,
    /// comma-separated capture ids to run (default: all)
    #[argh(option, short = 'o')]
    only: Option<String>,
    /// walk $skip pagination to the end and collect every page
    #[argh(switch)]
    all_pages: bool,
    /// maximum pages collected per endpoint with --all-pages
    #[argh(option, default = "200")]
    max_pages: u32,
    /// delay between consecutive requests, in milliseconds
    #[argh(option, default = "500")]
    delay_ms: u64,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Args = argh::from_env();
    let captures = endpoints::all();

    if args.list {
        for capture in &captures {
            println!("{}", capture.id);
        }
        return ExitCode::SUCCESS;
    }

    let selected = match select(&captures, args.only.as_deref()) {
        Ok(selected) => selected,
        Err(unknown) => {
            eprintln!("unknown capture id: {unknown}");
            eprintln!(
                "valid ids: {}",
                captures
                    .iter()
                    .map(|capture| capture.id)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            return ExitCode::from(2);
        }
    };

    let Ok(account_key) = std::env::var("LTA_ACCOUNT_KEY") else {
        eprintln!("LTA_ACCOUNT_KEY is not set.");
        eprintln!("store it once with `secretspec set LTA_ACCOUNT_KEY`, then run via:");
        eprintln!("  secretspec run -- cargo run --example save_fixtures");
        return ExitCode::from(2);
    };

    let ctx = Ctx {
        client: satay_reqwest::reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("build reqwest client"),
        account_key,
        fixtures_root: std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"),
        all_pages: args.all_pages,
        max_pages: args.max_pages,
        pacer: Pacer::new(Duration::from_millis(args.delay_ms)),
    };

    let mut failed = 0usize;
    for capture in &selected {
        match capture::run(&ctx, capture).await {
            Ok(run) => println!(
                "✓ {}: {} file(s), {} records",
                run.id,
                run.files.len(),
                run.records
            ),
            Err(message) => {
                eprintln!("✗ {message}");
                failed += 1;
            }
        }
    }

    println!();
    println!(
        "Done: {} endpoint(s) ok, {failed} failed.",
        selected.len() - failed
    );
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn select<'c>(captures: &'c [Capture], only: Option<&str>) -> Result<Vec<&'c Capture>, String> {
    let Some(only) = only else {
        return Ok(captures.iter().collect());
    };
    only.split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(|id| {
            captures
                .iter()
                .find(|capture| capture.id == id)
                .ok_or_else(|| id.to_owned())
        })
        .collect()
}
