use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use gitguardian_api::{ApiCall, ApiConfig, Paginated, models::page::Page};
use http::Response;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Client {
    http: ::reqwest::Client,
    config: ApiConfig,
}

impl Client {
    pub fn new(config: ApiConfig) -> Self {
        Self::with_http(config, ::reqwest::Client::new())
    }

    pub fn with_http(config: ApiConfig, http: ::reqwest::Client) -> Self {
        Self { http, config }
    }

    pub fn config(&self) -> &ApiConfig {
        &self.config
    }

    pub async fn send<C: ApiCall>(&self, call: &C) -> Result<C::Output, Error> {
        let request = call.build(&self.config)?;
        let response = self
            .http
            .execute(::reqwest::Request::try_from(request)?)
            .await?;

        let status = response.status();
        let headers = response.headers().clone();
        let mut out = Response::new(response.bytes().await?);
        *out.status_mut() = status;
        *out.headers_mut() = headers;

        Ok(call.parse(out)?)
    }

    pub fn paginate<T, C>(&self, call: C) -> Pages<'_, T, C>
    where
        C: Paginated<Output = Page<T>> + Send + Sync + Unpin + 'static,
        T: Send + 'static,
    {
        Pages {
            client: self,
            state: State::Ready(call),
        }
    }
}

type Pending<'a, T, C> = Pin<Box<dyn Future<Output = (Result<Page<T>, Error>, C)> + Send + 'a>>;

enum State<'a, T, C> {
    Ready(C),
    InFlight(Pending<'a, T, C>),
    Done,
}

pub struct Pages<'a, T, C> {
    client: &'a Client,
    state: State<'a, T, C>,
}

impl<T, C> Stream for Pages<'_, T, C>
where
    C: Paginated<Output = Page<T>> + Send + Sync + Unpin + 'static,
    T: Send + 'static,
{
    type Item = Result<Page<T>, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let client = this.client;

        let mut pending: Pending<'_, T, C> = match std::mem::replace(&mut this.state, State::Done) {
            State::Ready(call) => Box::pin(async move { (client.send(&call).await, call) }),
            State::InFlight(pending) => pending,
            State::Done => return Poll::Ready(None),
        };

        match pending.as_mut().poll(cx) {
            Poll::Pending => {
                this.state = State::InFlight(pending);
                Poll::Pending
            }
            Poll::Ready((Ok(page), mut call)) => {
                if let Some(cursor) = page.next.clone() {
                    call.set_cursor(cursor);
                    this.state = State::Ready(call);
                }
                Poll::Ready(Some(Ok(page)))
            }
            Poll::Ready((Err(error), _)) => Poll::Ready(Some(Err(error))),
        }
    }
}
