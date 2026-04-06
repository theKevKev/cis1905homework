use std::{thread, time::Duration};

use crate::mpmc;

#[test]
fn test_send_recv() {
    let (tx, rx) = mpmc::channel();

    // receive a value after sending
    assert!(tx.send(42).is_ok());
    assert_eq!(rx.recv(), Some(42));
}

#[test]
fn test_send_recv_nothing() {
    let (_tx, rx) = mpmc::channel::<u32>();

    // spawn a thread to receive a nonexistent value
    let t = thread::spawn(move || {
        let _ = rx.recv();
    });

    // receiving on an empty channel should block indefinitely
    thread::sleep(Duration::from_millis(100));
    assert!(!t.is_finished());
}

#[test]
fn test_send_recv_later() {
    let (tx, rx) = mpmc::channel::<u32>();

    // spawn a thread to receive a nonexistent value
    let t = thread::spawn(move || {
        assert_eq!(rx.recv(), Some(10));
    });

    // we shouldn't have received data yet
    thread::sleep(Duration::from_millis(10));
    assert!(!t.is_finished());

    // now send some data
    assert!(tx.send(10).is_ok());

    // at this point, the thread should be finished
    thread::sleep(Duration::from_millis(10));
    assert!(t.is_finished());
}

#[test]
fn test_recv_disconnected() {
    let (tx, rx) = mpmc::channel();

    // test sending some dummy value
    assert!(tx.send(()).is_ok());
    assert_eq!(rx.recv(), Some(()));

    // drop the sender
    drop(tx);

    // the receier should return none **immediately** (not block indefinitely)
    let t = thread::spawn(move || {
        assert_eq!(rx.recv(), None);
    });

    // detect blocking and fail the test (instead of hanging)
    thread::sleep(Duration::from_millis(10));
    assert!(t.is_finished());
}

#[test]
fn test_recv_disconnected_while_blocked() {
    let (tx, rx) = mpmc::channel::<char>();

    // the receier should initially block while waiting for a value
    let t = thread::spawn(move || {
        assert_eq!(rx.recv(), None);
    });

    // the thread should still be running as the receiver is blocked
    thread::sleep(Duration::from_millis(10));
    assert!(!t.is_finished());

    // drop the sender before sending any data
    drop(tx);

    // the receiver should have received none at this point (due to disconnect)
    thread::sleep(Duration::from_millis(10));
    assert!(t.is_finished());
}

#[test]
fn test_recv_disconnected_while_blocked_many() {
    let (tx, rx1) = mpmc::channel::<char>();

    // clone the receivers
    let rx2 = rx1.clone();
    let rx3 = rx1.clone();

    // the receiers should initially block while waiting for a value
    let t = thread::spawn(move || {
        assert_eq!(rx1.recv(), Some('!'));
        assert_eq!(rx2.recv(), None);
        assert_eq!(rx3.recv(), None);
    });

    // the thread should still be running as the receiver is blocked
    thread::sleep(Duration::from_millis(10));
    assert!(!t.is_finished());

    // drop the sender before after sending a single message
    assert!(tx.send('!').is_ok());
    drop(tx);

    // the remaining receivers should have received none at this point (due to disconnect)
    thread::sleep(Duration::from_millis(10));
    assert!(t.is_finished());
}

#[test]
fn test_recv_disconnected_with_data() {
    let (tx, rx) = mpmc::channel();

    // send some data and drop the sender
    assert!(tx.send("hello").is_ok());
    drop(tx);

    // receiver should still be able to receive data
    assert_eq!(rx.recv(), Some("hello"));

    // now receiver should indicate disconnected channel
    assert_eq!(rx.recv(), None);
}

#[test]
fn test_multi_producer_multi_consumer() {
    let (tx1, rx1) = mpmc::channel::<char>();

    // clone the senders and receivers
    let (tx2, rx2) = (tx1.clone(), rx1.clone());
    let (tx3, rx3) = (tx2.clone(), rx2.clone());

    // spawn some threads for the senders
    thread::spawn(move || {
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(1));
            assert!(tx1.send('1').is_ok());
        }
    });
    thread::spawn(move || {
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(1));
            assert!(tx2.send('2').is_ok());
        }
    });
    thread::spawn(move || {
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(1));
            assert!(tx3.send('3').is_ok());
        }
    });

    // spawn some threads for the receivers (slower than the senders)
    // `thread::scope` means that we wait for the threads to finish
    let mut results1 = Vec::new();
    let mut results2 = Vec::new();
    let mut results3 = Vec::new();
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..100 {
                thread::sleep(Duration::from_millis(2));
                results1.push(rx1.recv().expect("channel should be open"));
            }
        });
        s.spawn(|| {
            for _ in 0..100 {
                thread::sleep(Duration::from_millis(2));
                results2.push(rx2.recv().expect("channel should be open"));
            }
        });
        s.spawn(|| {
            for _ in 0..100 {
                thread::sleep(Duration::from_millis(2));
                results3.push(rx3.recv().expect("channel should be open"));
            }
        });
    });

    // collect the results into a single sorted vector
    let mut results = Vec::new();
    results.append(&mut results1);
    results.append(&mut results2);
    results.append(&mut results3);
    results.sort();

    // we should have gotten everything
    let mut results_iter = results.into_iter();
    for _ in 0..100 {
        assert_eq!(results_iter.next(), Some('1'));
    }
    for _ in 0..100 {
        assert_eq!(results_iter.next(), Some('2'));
    }
    for _ in 0..100 {
        assert_eq!(results_iter.next(), Some('3'));
    }
}
