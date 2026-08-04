//! Queue primitive benchmarks.
//!
//! Times `armature-queue` job/config construction, priority comparison and the
//! JSON payload round-trip that job creation depends on. No broker, worker or
//! storage backend is involved - these are the in-process data structures only.
//!
//! ```bash
//! cargo bench -p armature-queue --bench queue
//! ```

use armature_queue::{Job as QueueJob, JobPriority, QueueConfig};
use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::json;
use std::hint::black_box;

fn bench_queue_job_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_job");

    group.bench_function("job_new", |b| {
        b.iter(|| {
            QueueJob::new(
                black_box("default"),
                black_box("send_email"),
                black_box(json!({"to": "user@example.com"})),
            )
        })
    });

    group.bench_function("uuid_generation", |b| {
        b.iter(|| {
            let _id = uuid::Uuid::new_v4();
        })
    });

    group.bench_function("json_serialization", |b| {
        let data = json!({"type": "test", "payload": {"key": "value"}});
        b.iter(|| serde_json::to_string(black_box(&data)).unwrap())
    });

    group.bench_function("json_parsing", |b| {
        let json_str = r#"{"type":"test","payload":{"key":"value"}}"#;
        b.iter(|| serde_json::from_str::<serde_json::Value>(black_box(json_str)).unwrap())
    });

    group.finish();
}

fn bench_queue_config(c: &mut Criterion) {
    c.bench_function("queue_config_new", |b| {
        b.iter(|| QueueConfig::new(black_box("redis://localhost:6379"), black_box("default")))
    });
}

fn bench_job_priority(c: &mut Criterion) {
    c.bench_function("enum_comparison", |b| {
        b.iter(|| {
            let p1 = JobPriority::Critical;
            let p2 = JobPriority::High;
            black_box(p1 == p2);
        })
    });
}

criterion_group!(
    queue_benches,
    bench_queue_job_creation,
    bench_queue_config,
    bench_job_priority,
);

criterion_main!(queue_benches);
