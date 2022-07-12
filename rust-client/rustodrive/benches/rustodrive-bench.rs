use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(feature = "mock-socket")]
fn bench_can_proxy(c: &mut Criterion) {
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        time::Instant,
    };

    use rustodrive::{
        canproxy::CANProxy,
        commands::{ODriveCommand, Write, Read},
        messages::CANRequest,
    };

    let mut can_proxy = CANProxy::new("fakecan");

    c.bench_function("send request", |b| {
        b.iter_custom(|iters| {
            const num_messages: usize = 1000;
            let is_done = Arc::new(AtomicBool::new(false));
            let is_done_clone = is_done.clone();

            can_proxy.register_rw("thread 1", move |can_read_write| {
                let frames = (0..num_messages).into_iter().map(|axis| black_box(CANRequest {
                    axis: axis as u32,
                    cmd: ODriveCommand::Read(Read::EncoderError),
                    data: [0;8]
                })).collect();
                can_read_write.request_many(frames);
                is_done_clone.store(true, Ordering::SeqCst);
            });

            let start = Instant::now();
            while !is_done.load(Ordering::SeqCst) {
                can_proxy.process_messages();
            }

            let elapsed = start.elapsed().div_f32(num_messages as f32);
            can_proxy.unregister("thread 1");
            return elapsed;
        });
    });
}

#[cfg(not(feature = "mock-socket"))]
fn bench_can_proxy() {}

criterion_group!(benches, bench_can_proxy);
criterion_main!(benches);
