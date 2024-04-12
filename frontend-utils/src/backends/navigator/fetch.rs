use futures::AsyncReadExt;
use isahc::{prelude::*, AsyncBody, Response as IsahcResponse};
use ruffle_core::backend::navigator::{OwnedFuture, SuccessResponse};
use ruffle_core::loader::Error;
use std::sync::{Arc, Mutex};

pub enum DesktopResponseBody {
    /// The response's body comes from a file.
    File(Result<Vec<u8>, std::io::Error>),

    /// The response's body comes from the network.
    ///
    /// This has to be stored in shared ownership so that we can return
    /// owned futures. A synchronous lock is used here as we do not
    /// expect contention on this lock.
    Network(Arc<Mutex<IsahcResponse<AsyncBody>>>),
}

pub struct DesktopResponse {
    pub url: String,
    pub response_body: DesktopResponseBody,
    pub status: u16,
    pub redirected: bool,
}

impl SuccessResponse for DesktopResponse {
    fn url(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed(&self.url)
    }

    #[allow(clippy::await_holding_lock)]
    fn body(self: Box<Self>) -> OwnedFuture<Vec<u8>, Error> {
        match self.response_body {
            DesktopResponseBody::File(file) => {
                Box::pin(async move { file.map_err(|e| Error::FetchError(e.to_string())) })
            }
            DesktopResponseBody::Network(response) => Box::pin(async move {
                let mut body = vec![];
                response
                    .lock()
                    .expect("working lock during fetch body read")
                    .copy_to(&mut body)
                    .await
                    .map_err(|e| Error::FetchError(e.to_string()))?;

                Ok(body)
            }),
        }
    }

    fn status(&self) -> u16 {
        self.status
    }

    fn redirected(&self) -> bool {
        self.redirected
    }

    #[allow(clippy::await_holding_lock)]
    fn next_chunk(&mut self) -> OwnedFuture<Option<Vec<u8>>, Error> {
        match &mut self.response_body {
            DesktopResponseBody::File(file) => {
                let res = file
                    .as_mut()
                    .map(std::mem::take)
                    .map_err(|e| Error::FetchError(e.to_string()));

                Box::pin(async move {
                    match res {
                        Ok(bytes) if !bytes.is_empty() => Ok(Some(bytes)),
                        Ok(_) => Ok(None),
                        Err(e) => Err(e),
                    }
                })
            }
            DesktopResponseBody::Network(response) => {
                let response = response.clone();

                Box::pin(async move {
                    let mut buf = vec![0; 4096];
                    let lock = response.try_lock();
                    if matches!(lock, Err(std::sync::TryLockError::WouldBlock)) {
                        return Err(Error::FetchError(
                            "Concurrent read operations on the same stream are not supported."
                                .to_string(),
                        ));
                    }

                    let result = lock
                        .expect("desktop network lock")
                        .body_mut()
                        .read(&mut buf)
                        .await;

                    match result {
                        Ok(count) if count > 0 => {
                            buf.resize(count, 0);
                            Ok(Some(buf))
                        }
                        Ok(_) => Ok(None),
                        Err(e) => Err(Error::FetchError(e.to_string())),
                    }
                })
            }
        }
    }

    fn expected_length(&self) -> Result<Option<u64>, Error> {
        match &self.response_body {
            DesktopResponseBody::File(file) => Ok(file.as_ref().map(|file| file.len() as u64).ok()),
            DesktopResponseBody::Network(response) => {
                let response = response.lock().expect("no recursive locks");
                let content_length = response.headers().get("Content-Length");

                if let Some(len) = content_length {
                    Ok(Some(
                        len.to_str()
                            .map_err(|_| Error::InvalidHeaderValue)?
                            .parse::<u64>()?,
                    ))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
