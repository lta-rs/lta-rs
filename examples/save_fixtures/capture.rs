use std::future::Future;
use std::io::ErrorKind::NotFound;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{fs, io};

use satay_reqwest::Error::Reqwest;
use tokio::time;

use crate::http::retry_after;

/// Records per full `DataMall` page; a shorter page means the `$skip` walk is done.
pub const PAGE_SIZE: usize = 500;
const MAX_ATTEMPTS: u32 = 5;

/// A single successful fetch: the decoded record count plus the untouched wire body.
pub struct Captured {
    pub records: usize,
    pub body: Vec<u8>,
}

pub enum FailureKind {
    Transport(satay_reqwest::Error),
    Unexpected(http::StatusCode),
}

pub struct AttemptFailure {
    pub kind: FailureKind,
    pub retry_after: Option<Duration>,
}

impl From<satay_reqwest::Error> for AttemptFailure {
    fn from(err: satay_reqwest::Error) -> Self {
        Self {
            kind: FailureKind::Transport(err),
            retry_after: None,
        }
    }
}

impl AttemptFailure {
    pub fn unexpected(status: http::StatusCode, headers: &http::HeaderMap) -> Self {
        Self {
            kind: FailureKind::Unexpected(status),
            retry_after: retry_after(headers),
        }
    }

    /// satay decode errors are schema drift, not transient; reqwest transport errors are.
    fn retryable(&self) -> bool {
        match &self.kind {
            FailureKind::Transport(err) => matches!(err, Reqwest(_)),
            FailureKind::Unexpected(status) => {
                *status == http::StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
            }
        }
    }

    fn describe(&self) -> String {
        match &self.kind {
            FailureKind::Transport(err) => format!("transport error: {err}"),
            FailureKind::Unexpected(status) => format!("unexpected status {status}"),
        }
    }
}

/// Sleeps before every request except the first one issued by the process.
pub struct Pacer {
    delay: Duration,
    armed: AtomicBool,
}

impl Pacer {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            armed: AtomicBool::new(false),
        }
    }

    pub async fn wait(&self) {
        if self.armed.swap(true, Ordering::Relaxed) {
            time::sleep(self.delay).await;
        }
    }
}

pub struct Ctx {
    pub client: reqwest::Client,
    pub account_key: String,
    pub fixtures_root: PathBuf,
    pub all_pages: bool,
    pub max_pages: u32,
    pub pacer: Pacer,
}

pub type FetchFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Captured, AttemptFailure>> + Send + 'a>>;

pub type FetchFn = for<'a> fn(&'a Ctx, u32) -> FetchFuture<'a>;

pub struct Capture {
    pub id: &'static str,
    /// Fixture directory under tests/fixtures.
    pub dir: &'static str,
    /// Fixtures are written as `<stem>_<N>.json`.
    pub stem: &'static str,
    /// Whether the endpoint supports `$skip` pagination.
    pub paginated: bool,
    pub fetch: FetchFn,
}

pub struct Run {
    pub id: &'static str,
    pub files: Vec<PathBuf>,
    pub records: usize,
}

pub async fn run(ctx: &Ctx, capture: &Capture) -> Result<Run, String> {
    let dir = ctx.fixtures_root.join(capture.dir);
    let mut number =
        next_number(&dir, capture.stem).map_err(|error| format!("{}: {error}", capture.id))?;

    let mut skip = 0u32;
    let mut pages = 0u32;
    let mut records = 0usize;
    let mut files = vec![];

    loop {
        let captured = fetch_with_retry(ctx, capture, skip).await?;
        let file_name = format!("{}_{}.json", capture.stem, number);
        let path = dir.join(&file_name);

        fs::write(&path, &captured.body).map_err(|error| {
            format!(
                "{}: failed to write {}: {error}",
                capture.id,
                path.display()
            )
        })?;

        println!(
            "✔ {} #{} → tests/fixtures/{}/{} ({} bytes, {} records)",
            capture.id,
            number,
            capture.dir,
            file_name,
            captured.body.len(),
            captured.records
        );

        files.push(path);
        records += captured.records;
        number += 1;
        pages += 1;

        if !capture.paginated
            || !ctx.all_pages
            || captured.records < PAGE_SIZE
            || pages >= ctx.max_pages
        {
            break;
        }

        skip += u32::try_from(captured.records).map_err(|s| s.to_string())?;
    }

    Ok(Run {
        id: capture.id,
        files,
        records,
    })
}

async fn fetch_with_retry(ctx: &Ctx, capture: &Capture, skip: u32) -> Result<Captured, String> {
    let mut attempt = 1u32;
    loop {
        ctx.pacer.wait().await;
        match (capture.fetch)(ctx, skip).await {
            Ok(captured) => return Ok(captured),
            Err(failure) => {
                let describe = failure.describe();

                if failure.retryable() && attempt < MAX_ATTEMPTS {
                    let wait = failure
                        .retry_after
                        .unwrap_or_else(|| Duration::from_secs(1 << (attempt - 1).min(4)));

                    eprintln!(
                        "  {} attempt {attempt}/{MAX_ATTEMPTS} failed ({describe}); retrying in {:.0}s",
                        capture.id,
                        wait.as_secs_f64()
                    );

                    time::sleep(wait).await;
                    attempt += 1;
                } else {
                    return Err(format!("{}: {describe}", capture.id));
                }
            }
        }
    }
}

/// Next capture number for a stem: one past the highest existing `<stem>_<N>.json`, starting
/// at 1 so the legacy `_0` imports are never touched.
fn next_number(dir: &Path, stem: &str) -> io::Result<u32> {
    let prefix = format!("{stem}_");
    let mut max = 0u32;

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == NotFound => return Ok(max + 1),
        Err(error) => return Err(error),
    };

    for entry in entries {
        let name = entry?.file_name();
        let Some(name) = name.to_str() else { continue };
        let Some(rest) = name.strip_prefix(&prefix) else {
            continue;
        };
        let Some(digits) = rest.strip_suffix(".json") else {
            continue;
        };
        if let Ok(n) = digits.parse::<u32>() {
            max = max.max(n);
        }
    }

    Ok(max + 1)
}
