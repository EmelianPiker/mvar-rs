extern crate mvar;

use mvar::MVar;
use std::sync::Arc;

#[test]
fn test_single_thread() {
    let mvar: MVar<usize> = MVar::new_empty();
    mvar.put(1);
    println!("{:?}", mvar.take());
}

#[test]
fn test_spsc() {
    let mvar: Arc<MVar<usize>> = Arc::new(MVar::new_empty());
    let mut threads = vec![];

    let mvar1 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        println!("{:?}", mvar1.take());
    }));

    let mvar2 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        mvar2.put(2);
    }));

    for t in threads { t.join().unwrap(); }
}

#[test]
fn test_spmc() {
    let mvar: Arc<MVar<usize>> = Arc::new(MVar::new_empty());
    let mut threads = vec![];

    let mvar1 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        println!("{:?}", mvar1.take());
    }));

    let mvar1 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        println!("{:?}", mvar1.take());
    }));

    let mvar2 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        mvar2.put(3);
        mvar2.put(4);
    }));

    for t in threads { t.join().unwrap(); }
}

#[test]
fn test_mpsc() {
    let mvar: Arc<MVar<usize>> = Arc::new(MVar::new_empty());
    let mut threads = vec![];

    let mvar1 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        println!("{:?}", mvar1.take());
        println!("{:?}", mvar1.take());
    }));

    let mvar2 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        mvar2.put(5);
    }));

    let mvar2 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        mvar2.put(6);
    }));

    for t in threads { t.join().unwrap(); }
}

#[test]
fn test_mpmc() {
    let mvar: Arc<MVar<usize>> = Arc::new(MVar::new_empty());
    let mut threads = vec![];

    let mvar1 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        println!("{:?}", mvar1.take());
    }));

    let mvar1 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        println!("{:?}", mvar1.take());
    }));

    let mvar2 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        mvar2.put(7);
    }));

    let mvar2 = mvar.clone();
    threads.push(::std::thread::spawn(move|| {
        mvar2.put(8);
    }));

    for t in threads { t.join().unwrap(); }
}
