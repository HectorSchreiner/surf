use std::pin::Pin;
use std::task::{Context, Poll};

use ::async_trait::async_trait;
use ::tokio::task::JoinHandle;
use ::tokio_stream::wrappers::BroadcastStream;

use crate::domains::vulnerabilities::{VulnerabilityEvent, VulnerabilityFeed};

pub struct Github {
    handle: JoinHandle<()>,
}

impl Github {
    pub fn new() -> anyhow::Result<Self> {
        let handle = tokio::spawn(async move {});

        Ok(Self { handle })
    }
}

// #[async_trait]
// impl VulnerabilityFeed for Github {
//     async fn listen(&self) -> Result<impl VulnerabilityQueue, ()> {
//         let (_, rx) = tokio::sync::broadcast::channel::<VulnerabilityEvent>(1024);
//         Ok(BroadcastStream::new(rx))
//     }
// }
