//! # Select and Timeout
//!
//! In this exercise, you will use `tokio::select!` macro to implement race selection and timeout control.
//!
//! ## Concepts
//! - `tokio::select!` waits for multiple async operations simultaneously
//! - `tokio::time::timeout` timeout control
//! - The first completed branch is executed, others are cancelled

use std::future::Future;
use tokio::time::{sleep, Duration};

/// Async operation with timeout.
/// If `future` completes within `timeout_ms` milliseconds, returns Some(result).
/// Otherwise returns None.
///
/// Hint: Use `tokio::select!` or `tokio::time::timeout`.
pub async fn with_timeout<F, T>(future: F, timeout_ms: u64) -> Option<T>
where
    F: Future<Output = T>,
{
    // tokio::pin! 是宏：把 future 固定在栈上，得到 Pin<&mut F>。
    // select! 需要先 pin 住才能反复 poll 同一个 Future。
    tokio::pin!(future);

    // select! 是宏：同时等待多个分支，谁先完成就执行谁的分支，其余分支被取消
    tokio::select! {
        // future 先完成：拿到结果
        value = &mut future => Some(value),
        // 超时分支先完成：返回 None
        _ = sleep(Duration::from_millis(timeout_ms)) => None,
    }
}

/// Race two async tasks, return the result of whichever finishes first.
///
/// Hint: Use `tokio::select!` macro.
pub async fn race<F1, F2, T>(f1: F1, f2: F2) -> T
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    // 两个 Future 都要先 pin 住，select! 才能 poll 它们
    tokio::pin!(f1);
    tokio::pin!(f2);

    tokio::select! {
        // 两个分支返回类型相同（都是 T），谁先完成就用谁的结果
        value = &mut f1 => value,
        value = &mut f2 => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_success() {
        let result = with_timeout(async { 42 }, 100).await;
        assert_eq!(result, Some(42));
    }

    #[tokio::test]
    async fn test_timeout_expired() {
        let result = with_timeout(
            async {
                sleep(Duration::from_millis(200)).await;
                42
            },
            50,
        )
        .await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_race_first_wins() {
        let result = race(
            async {
                sleep(Duration::from_millis(10)).await;
                "fast"
            },
            async {
                sleep(Duration::from_millis(200)).await;
                "slow"
            },
        )
        .await;
        assert_eq!(result, "fast");
    }

    #[tokio::test]
    async fn test_race_second_wins() {
        let result = race(
            async {
                sleep(Duration::from_millis(200)).await;
                "slow"
            },
            async {
                sleep(Duration::from_millis(10)).await;
                "fast"
            },
        )
        .await;
        assert_eq!(result, "fast");
    }
}
