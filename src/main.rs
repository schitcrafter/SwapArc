#![feature(strict_provenance_atomic_ptr)]
#![feature(strict_provenance)]
#![feature(core_intrinsics)]
// only for testing!
#![allow(soft_unstable)]
#![feature(test)]
#![feature(bench_black_box)]

mod swap_arc_intermediate;
mod swap_arc_tls;
mod swap_arc;
mod swap_arc_tls_less_fence;
mod swap_arc_tls_optimistic;

use std::{mem, thread};
use std::hint::{black_box, spin_loop};
use std::mem::transmute;
use std::ops::Range;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::swap_arc_tls_optimistic::SwapArcIntermediateTLS;

extern crate test;
use test::Bencher;
use arc_swap::ArcSwap;

fn main() {
    /*for _ in 0..10 {
        test_us_single();
    }*/
    /*for _ in 0..10 {
        test_us_multi();
    }*/
    // bad_bench_us_multi();
    /*
    let arc = Arc::new(4);
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(arc);

    let mut threads = vec![];
    tmp.update(Arc::new(31));
    println!("{}", tmp.load());
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    for _ in 0..20/*5*//*1*/ {
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            for x in 0..2000/*200*/ {
                let l1 = tmp.load();
                let l2 = tmp.load();
                let l3 = tmp.load();
                let l4 = tmp.load();
                let l5 = tmp.load();
                println!("{}{}{}{}{}", l1, l2, l3, l4, l5);
                if x % 5 == 0 {
                    println!("completed load: {x}");
                }
                // thread::sleep(Duration::from_millis(1000));
            }
        }));
    }
    for _ in 0..20/*5*//*1*/ {
        // let send = send.clone();
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            // let send = send.clone();
            for x in 0..2000/*200*/ {
                /*
                thread::sleep(Duration::from_millis(500));
                println!("{:?}", list.remove_head());
                thread::sleep(Duration::from_millis(500));*/
                // send.send(list.remove_head()).unwrap();
                tmp.update(Arc::new(rand::random()));

                if x % 5 == 0 {
                    println!("completed removals: {x}");
                }
            }
        }));
    }
    threads.into_iter().for_each(|thread| thread.join().unwrap());
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("test took: {}ms", time - start);

    // loop {}
    println!("{:?}", tmp.load().as_ref());
    tmp.update(Arc::new(6));
    println!("{:?}", tmp.load().as_ref());
    println!("test! 0");*/
    /*let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(0));
    let mut threads = vec![];
    for _ in 0..20/*5*//*1*/ {
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..2000/*200*/ {
                let l1 = tmp.load();
                black_box(l1);
            }
        }));
    }
    for _ in 0..20/*5*//*1*/ {
        // let send = send.clone();
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            // let send = send.clone();
            for _ in 0..2000/*200*/ {
                tmp.update(Arc::new(rand::random()));
            }
        }));
    }
    threads.into_iter().for_each(|thread| thread.join().unwrap());*/
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(3));
    let mut threads = vec![];
    for _ in 0..20/*5*//*1*/ {
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..20000/*200*/ {
                let l1 = tmp.load();
                let l2 = tmp.load();
                let l3 = tmp.load();
                let l4 = tmp.load();
                let l5 = tmp.load();
                black_box(l1);
                black_box(l2);
                black_box(l3);
                black_box(l4);
                black_box(l5);
            }
        }));
    }
    threads.into_iter().for_each(|thread| thread.join().unwrap());
}

fn bad_bench_us_multi() {
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(0));
    let mut many_threads = Arc::new(Mutex::new(vec![]));
    for _ in 0..10 {
        let started = Arc::new(AtomicBool::new(false));
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            let started = started.clone();
            threads.push(thread::spawn(move || {
                while !started.load(Ordering::Acquire) {
                    spin_loop();
                }

                for _ in 0..2000/*200*/ {
                    let l1 = tmp.load();
                    let l2 = tmp.load();
                    let l3 = tmp.load();
                    let l4 = tmp.load();
                    let l5 = tmp.load();
                    black_box(l1);
                    black_box(l2);
                    black_box(l3);
                    black_box(l4);
                    black_box(l5);
                }
            }));
        }
        for _ in 0..20/*5*//*1*/ {
            // let send = send.clone();
            let tmp = tmp.clone();
            let started = started.clone();
            threads.push(thread::spawn(move || {
                while !started.load(Ordering::Acquire) {
                    spin_loop();
                }

                for _ in 0..2000/*200*/ {
                    tmp.update(Arc::new(rand::random()));
                }
            }));
        }
        many_threads.lock().unwrap().push((threads, started));
    }
    for _ in 0..10 {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let (threads, started) = many_threads.clone().lock().unwrap().remove(0);
        started.store(true, Ordering::Release);
        threads.into_iter().for_each(|thread| thread.join().unwrap());
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("test took: {}ms", time - start);
    }
}

/*
#[bench]
fn bench_us_multi(bencher: &mut Bencher) {
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(0));
    let mut many_threads = Arc::new(Mutex::new(vec![]));
    for _ in 0..100 {
        let started = Arc::new(AtomicBool::new(false));
        let mut threads = vec![];
        for _ in 0..5/*5*//*1*/ {
            let tmp = tmp.clone();
            let started = started.clone();
            threads.push(thread::spawn(move || {
                while !started.load(Ordering::Acquire) {
                    spin_loop();
                }

                for _ in 0..2000/*200*/ {
                    let l1 = tmp.load();
                    let l2 = tmp.load();
                    let l3 = tmp.load();
                    let l4 = tmp.load();
                    let l5 = tmp.load();
                    black_box(l1);
                    black_box(l2);
                    black_box(l3);
                    black_box(l4);
                    black_box(l5);
                }
            }));
        }
        for _ in 0..5/*5*//*1*/ {
            // let send = send.clone();
            let tmp = tmp.clone();
            let started = started.clone();
            threads.push(thread::spawn(move || {
                while !started.load(Ordering::Acquire) {
                    spin_loop();
                }

                for _ in 0..2000/*200*/ {
                    tmp.update(Arc::new(rand::random()));
                }
            }));
        }
        many_threads.lock().unwrap().push((threads, started));
    }
    bencher.iter(|| {
        let (threads, started) = many_threads.clone().lock().unwrap().remove(0);
        started.store(true, Ordering::Release);
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

#[bench]
fn bench_other_multi(bencher: &mut Bencher) {
    let tmp = Arc::new(ArcSwap::new(Arc::new(0)));

    let mut many_threads = Arc::new(Mutex::new(vec![]));
    for _ in 0..100 {
        let started = Arc::new(AtomicBool::new(false));
        let mut threads = vec![];
        for _ in 0..5/*5*//*1*/ {
            let tmp = tmp.clone();
            let started = started.clone();
            threads.push(thread::spawn(move || {
                while !started.load(Ordering::Acquire) {
                    spin_loop();
                }

                for _ in 0..2000/*200*/ {
                    let l1 = tmp.load();
                    let l2 = tmp.load();
                    let l3 = tmp.load();
                    let l4 = tmp.load();
                    let l5 = tmp.load();
                    black_box(l1);
                    black_box(l2);
                    black_box(l3);
                    black_box(l4);
                    black_box(l5);
                }
            }));
        }
        for _ in 0..5/*5*//*1*/ {
            // let send = send.clone();
            let tmp = tmp.clone();
            let started = started.clone();
            threads.push(thread::spawn(move || {
                while !started.load(Ordering::Acquire) {
                    spin_loop();
                }

                for _ in 0..2000/*200*/ {
                    tmp.store(Arc::new(rand::random()));
                }
            }));
        }
        many_threads.lock().unwrap().push((threads, started));
    }
    bencher.iter(|| {
        let (threads, started) = many_threads.clone().lock().unwrap().remove(0);
        started.store(true, Ordering::Release);
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}*/

#[bench]
fn bench_us_multi(bencher: &mut Bencher) {
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(0));
    bencher.iter(|| {
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..20000/*200*/ {
                    let l1 = tmp.load();
                    let l2 = tmp.load();
                    let l3 = tmp.load();
                    let l4 = tmp.load();
                    let l5 = tmp.load();
                    black_box(l1);
                    black_box(l2);
                    black_box(l3);
                    black_box(l4);
                    black_box(l5);
                }
            }));
        }
        for _ in 0..20/*5*//*1*/ {
            // let send = send.clone();
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                // let send = send.clone();
                for _ in 0..20000/*200*/ {
                    tmp.update(Arc::new(rand::random()));
                }
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

#[bench]
fn bench_other_multi(bencher: &mut Bencher) {
    let tmp = Arc::new(ArcSwap::new(Arc::new(0)));

    bencher.iter(|| {
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..20000/*200*/ {
                    let l1 = tmp.load();
                    let l2 = tmp.load();
                    let l3 = tmp.load();
                    let l4 = tmp.load();
                    let l5 = tmp.load();
                    black_box(l1);
                    black_box(l2);
                    black_box(l3);
                    black_box(l4);
                    black_box(l5);
                }
            }));
        }
        for _ in 0..20/*5*//*1*/ {
            // let send = send.clone();
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                // let send = send.clone();
                for _ in 0..20000/*200*/ {
                    tmp.store(Arc::new(rand::random()));
                }
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

#[bench]
fn bench_us_single(bencher: &mut Bencher) {
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(0));
    bencher.iter(|| {
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..20000/*200*/ {
                    let l1 = tmp.load();
                    black_box(l1);
                }
            }));
        }
        for _ in 0..20/*5*//*1*/ {
            // let send = send.clone();
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                // let send = send.clone();
                for _ in 0..20000/*200*/ {
                    tmp.update(Arc::new(rand::random()));
                }
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

#[bench]
fn bench_other_single(bencher: &mut Bencher) {
    let tmp = Arc::new(ArcSwap::new(Arc::new(0)));

    bencher.iter(|| {
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..20000/*200*/ {
                    let l1 = tmp.load();
                    black_box(l1);
                }
            }));
        }
        for _ in 0..20/*5*//*1*/ {
            // let send = send.clone();
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                // let send = send.clone();
                for _ in 0..20000/*200*/ {
                    tmp.store(Arc::new(rand::random()));
                }
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

#[bench]
fn bench_us_read_heavy_single(bencher: &mut Bencher) {
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(3));
    bencher.iter(|| {
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..200000/*200*/ {
                    let l1 = tmp.load();
                    black_box(l1);
                }
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

#[bench]
fn bench_other_read_heavy_single(bencher: &mut Bencher) {
    let tmp = Arc::new(ArcSwap::new(Arc::new(3)));

    bencher.iter(|| {
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..200000/*200*/ {
                    let l1 = tmp.load();
                    black_box(l1);
                }
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

#[bench]
fn bench_us_read_heavy_multi(bencher: &mut Bencher) {
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(3));
    bencher.iter(|| {
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..200000/*200*/ {
                    let l1 = tmp.load();
                    let l2 = tmp.load();
                    let l3 = tmp.load();
                    let l4 = tmp.load();
                    let l5 = tmp.load();
                    black_box(l1);
                    black_box(l2);
                    black_box(l3);
                    black_box(l4);
                    black_box(l5);
                }
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

#[bench]
fn bench_other_read_heavy_multi(bencher: &mut Bencher) {
    let tmp = Arc::new(ArcSwap::new(Arc::new(3)));

    bencher.iter(|| {
        let mut threads = vec![];
        for _ in 0..20/*5*//*1*/ {
            let tmp = tmp.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..200000/*200*/ {
                    let l1 = tmp.load();
                    let l2 = tmp.load();
                    let l3 = tmp.load();
                    let l4 = tmp.load();
                    let l5 = tmp.load();
                    black_box(l1);
                    black_box(l2);
                    black_box(l3);
                    black_box(l4);
                    black_box(l5);
                }
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
    });
}

fn test_us_multi() {
    let arc = Arc::new(4);
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(arc);
    tmp.update(Arc::new(31));

    let mut threads = vec![];
    for _ in 0..20/*5*//*1*/ {
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..20000/*200*/ {
                let l1 = tmp.load();
                let l2 = tmp.load();
                let l3 = tmp.load();
                let l4 = tmp.load();
                let l5 = tmp.load();
                black_box(l1);
                black_box(l2);
                black_box(l3);
                black_box(l4);
                black_box(l5);
            }
        }));
    }
    for _ in 0..20/*5*//*1*/ {
        // let send = send.clone();
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            // let send = send.clone();
            for _ in 0..20000/*200*/ {
                tmp.update(Arc::new(rand::random()));
            }
        }));
    }
    threads.into_iter().for_each(|thread| thread.join().unwrap());
}

fn test_us_single() {
    let tmp: Arc<SwapArcIntermediateTLS<i32, Arc<i32>, 0>> = SwapArcIntermediateTLS::new(Arc::new(3));
    let mut threads = vec![];
    for _ in 0..20/*5*//*1*/ {
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..2000/*200*/ {
                let l1 = tmp.load();
                black_box(l1);
            }
        }));
    }
    for _ in 0..20/*5*//*1*/ {
        // let send = send.clone();
        let tmp = tmp.clone();
        threads.push(thread::spawn(move || {
            // let send = send.clone();
            for _ in 0..2000/*200*/ {
                tmp.update(Arc::new(rand::random()));
            }
        }));
    }
    threads.into_iter().for_each(|thread| thread.join().unwrap());
}

/*
fn test_leak_arc(arc: &Arc<i32>) {

}

fn leak_arc<'a, T: 'a>(val: Arc<T>) -> &'a Arc<T> {
    let ptr = addr_of!(val);
    mem::forget(val);
    unsafe { ptr.as_ref() }.unwrap()
}
*/