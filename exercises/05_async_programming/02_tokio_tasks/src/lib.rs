//! # Tokio Async Tasks
//!
//! In this exercise, you will use `tokio::spawn` to create concurrent asynchronous tasks.
//!
//! ## Concepts
//! - `tokio::spawn` creates asynchronous tasks
//! - `JoinHandle` waits for task completion
//! - Concurrent execution between asynchronous tasks

use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

/// Concurrently compute the square of each number in 0..n, collect results and return in order.
///
/// Hint: Create `tokio::spawn` task for each i, collect JoinHandle, await them sequentially.
pub async fn concurrent_squares(n: usize) -> Vec<usize> {
    // 先收集所有任务的 JoinHandle，此时任务已经在后台运行
    let mut handles: Vec<JoinHandle<usize>> = Vec::new();

    for i in 0..n {
        // async move 把 i 的所有权移进任务，避免借用外部变量
        handles.push(tokio::spawn(async move { i * i }));
    }

    // 再按创建顺序依次等待，结果顺序与 i 的顺序一致
    let mut results = Vec::new();
    for handle in handles {
        // JoinHandle 的 await 返回 Result；unwrap 取出成功的结果
        let value = handle.await.unwrap();
        results.push(value);
    }

    results
}

/// Concurrently execute multiple "time-consuming" tasks (simulated with sleep), return all results.
/// Each task sleeps `duration_ms` milliseconds and then returns its `task_id`.
///
/// Key: All tasks should execute concurrently, total duration should be close to single task duration, not sum of all tasks.
pub async fn parallel_sleep_tasks(n: usize, duration_ms: u64) -> Vec<usize> {
    let mut handles: Vec<JoinHandle<usize>> = Vec::new();

    for id in 0..n {
        // 每个任务各自 sleep 同样的时长；它们是并发执行的，不是排队执行
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(duration_ms)).await;
            id
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    // 任务完成顺序不确定，排序后返回
    results.sort();
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn test_squares_basic() {
        let result = concurrent_squares(5).await;
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }

    #[tokio::test]
    async fn test_squares_zero() {
        let result = concurrent_squares(0).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_squares_one() {
        let result = concurrent_squares(1).await;
        assert_eq!(result, vec![0]);
    }

    #[tokio::test]
    async fn test_parallel_sleep() {
        let start = Instant::now();
        let result = parallel_sleep_tasks(5, 100).await;
        let elapsed = start.elapsed();

        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        // Concurrent execution, total time should be much less than 5 * 100ms
        assert!(
            elapsed.as_millis() < 400,
            "Tasks should run concurrently, took {}ms",
            elapsed.as_millis()
        );
    }
}
