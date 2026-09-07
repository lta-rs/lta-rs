use std::future::Future;
use std::mem;
use std::time::Duration;

use http::header::RETRY_AFTER;
use satay_reqwest::reqwest::Request;
use satay_reqwest::satay_runtime;
use satay_runtime::Action;

/// The untouched wire response kept next to the decoded one, so fixture files contain
/// exactly the bytes the API returned.
pub struct RawResponse {
    pub headers: http::HeaderMap,
    pub body: Vec<u8>,
}

/// [`satay_reqwest::ReqwestActionExt::send_with`] with the raw wire response preserved:
/// the same satay Action pipeline (`request()` → reqwest conversion → `decode()`), returning
/// the decoded response alongside the untouched body for fixture capture.
pub trait CaptureActionExt: Action + Sized + Send {
    fn capture_with(
        self,
        client: &reqwest::Client,
    ) -> impl Future<Output = Result<(Self::Response, RawResponse), satay_reqwest::Error>> + Send
    {
        async move {
            let http_req = self.request()?;
            let reqwest_req: Request = http_req.try_into()?;
            let mut res = client.execute(reqwest_req).await?;

            let status = res.status();
            let headers = mem::take(res.headers_mut());
            let body = res.bytes().await?;
            let response = Self::decode(satay_runtime::ResponseParts {
                status,
                headers: headers.clone(),
                body: body.clone(),
            })?;

            Ok((
                response,
                RawResponse {
                    headers,
                    body: body.to_vec(),
                },
            ))
        }
    }
}

impl<T: Action + Send> CaptureActionExt for T {}

/// `Retry-After` expressed in seconds; the HTTP-date form is ignored.
pub fn retry_after(headers: &http::HeaderMap) -> Option<Duration> {
    let value = headers.get(RETRY_AFTER)?.to_str().ok()?;
    Some(Duration::from_secs(value.trim().parse().ok()?))
}
