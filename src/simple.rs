use futures::future::Future;
use futures::task::Context;
use futures::task::Poll;

use std::pin::Pin;

pub struct SimpleFuture {
    n: i32,
}

impl Future for SimpleFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(self.n)
    }
}

pub async fn simpleexec(p: i32) -> i32 {
    p + 10
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use std::pin::Pin;

    use futures::channel::oneshot;
    use futures::future::FutureExt;

    use lib::simple::{simpleexec, SimpleFuture};

    #[test]
    fn test_future_return_simpleexec_a_value() {
        let f = simpleexec(32);
        let (ch_s, ch_r) = oneshot::channel();
        let _ = block_on(f.map(move |n| ch_s.send(n + 5)));
        let result = block_on(ch_r).unwrap();
        assert_eq!(result, 47);
        // Pin::new(&mut f).poll(cx)
    }

    #[test]
    fn test_future_return_a_value() {
        let f = SimpleFuture { n: 32 };
        let (ch_s, ch_r) = oneshot::channel();
        let _ = block_on(f.map(move |n| ch_s.send(n + 5)));
        let result = block_on(ch_r).unwrap();
        assert_eq!(result, 37);
        // Pin::new(&mut f).poll(cx)
    }

    #[test]
    fn test_async_send() {
        let (ch_s, ch_r) = oneshot::channel();
        block_on(async move {
            let v = simpleexec(10).await;
            ch_s.send(v)
        })
        .unwrap();

        let fin = block_on(async move {
            let res = ch_r.await.unwrap();
            res + 5
        });
        assert_eq!(fin, 25);
    }
}
