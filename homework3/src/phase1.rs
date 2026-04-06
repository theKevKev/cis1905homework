use crate::mpmc::{self, TryRecvError};

#[test]
fn test_channel_function() {
    let (_, _) = mpmc::channel::<i32>();
}

#[test]
fn test_send() {
    let (tx, _rx) = mpmc::channel();

    // sending should succeed
    assert!(tx.send("hello, world!").is_ok());
}

#[test]
fn test_send_many() {
    let (tx, _rx) = mpmc::channel();

    // should be able to send multiple items
    for i in 0..10 {
        assert!(tx.send(i).is_ok());
    }
}

#[test]
fn test_send_try_recv() {
    let (tx, rx) = mpmc::channel();

    // receive a value after sending
    assert!(tx.send(42).is_ok());
    assert_eq!(rx.try_recv(), Ok(42));
}

#[test]
fn test_try_recv_nothing() {
    let (_tx, rx) = mpmc::channel::<u32>();

    // receiving on an empty channel should be an error
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
}

#[test]
fn test_send_multi_try_recv() {
    let (tx, rx) = mpmc::channel();

    // try sending and receiving multiple values
    assert!(tx.send(1).is_ok());
    assert!(tx.send(2).is_ok());
    assert!(tx.send(3).is_ok());
    assert_eq!(rx.try_recv(), Ok(1));
    assert_eq!(rx.try_recv(), Ok(2));
    assert_eq!(rx.try_recv(), Ok(3));

    // make sure the channel is cleared out
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
}

#[test]
fn test_buffering() {
    let (tx, rx) = mpmc::channel();

    // send and receive lots of data (gets buffered inside the channel)
    for i in 0..1000 {
        assert!(tx.send(i).is_ok());
    }
    for i in 0..1000 {
        assert_eq!(rx.try_recv(), Ok(i));
    }
}

#[test]
fn test_try_recv_after_empty() {
    let (tx, rx) = mpmc::channel::<&'static str>();

    // make sure the channel still works after returning empty
    assert!(tx.send("fst").is_ok());
    assert!(tx.send("snd").is_ok());
    assert_eq!(rx.try_recv(), Ok("fst"));
    assert_eq!(rx.try_recv(), Ok("snd"));
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
    assert!(tx.send("thd").is_ok());
    assert_eq!(rx.try_recv(), Ok("thd"));
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
}
