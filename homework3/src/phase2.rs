use crate::mpmc::{self, TryRecvError};

#[test]
fn test_clone_doesnt_panic() {
    let (tx, rx) = mpmc::channel::<u8>();

    // make sure cloning doesn't panic
    let _tx_clone = tx.clone();
    let _rx_clone = rx.clone();
}

#[test]
fn test_clone_sender_with_data() {
    let (tx1, rx) = mpmc::channel();

    // create some clones of the sender
    let tx2 = tx1.clone();
    let tx3 = tx2.clone();

    // send data on each clone
    assert!(tx1.send("hello from tx1").is_ok());
    assert!(tx2.send("hello from tx2").is_ok());
    assert!(tx3.send("hello from tx3").is_ok());

    // receive on a single receiver
    assert_eq!(rx.try_recv(), Ok("hello from tx1"));
    assert_eq!(rx.try_recv(), Ok("hello from tx2"));
    assert_eq!(rx.try_recv(), Ok("hello from tx3"));
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
}

#[test]
fn test_clone_receiver_with_data() {
    let (tx, rx1) = mpmc::channel();

    // create some clones of the receiver
    let rx2 = rx1.clone();
    let rx3 = rx2.clone();

    // send data on a single sender
    assert!(tx.send('a').is_ok());
    assert!(tx.send('b').is_ok());
    assert!(tx.send('c').is_ok());

    // receive data on each receiver
    assert_eq!(rx1.try_recv(), Ok('a'));
    assert_eq!(rx2.try_recv(), Ok('b'));
    assert_eq!(rx3.try_recv(), Ok('c'));

    // make sure the channel is empty
    assert_eq!(rx1.try_recv(), Err(TryRecvError::Empty));
    assert_eq!(rx2.try_recv(), Err(TryRecvError::Empty));
    assert_eq!(rx3.try_recv(), Err(TryRecvError::Empty));
}

#[test]
fn test_clone_receiver_with_data_order_independence() {
    let (tx, rx1) = mpmc::channel();

    // create a clone of the receiver
    let rx2 = rx1.clone();

    // send data on a single sender
    assert!(tx.send('x').is_ok());
    assert!(tx.send('y').is_ok());

    // receive data on each receiver (in the opposite order they were created)
    assert_eq!(rx2.try_recv(), Ok('x'));
    assert_eq!(rx1.try_recv(), Ok('y'));

    // make sure the channel is empty
    assert_eq!(rx1.try_recv(), Err(TryRecvError::Empty));
    assert_eq!(rx2.try_recv(), Err(TryRecvError::Empty));
}

#[test]
fn test_drop_sender() {
    let (tx, rx) = mpmc::channel::<i16>();

    // drop the sender
    drop(tx);

    // channel is disconnected, so we should get an error on receive
    assert_eq!(rx.try_recv(), Err(TryRecvError::Disconnected));
}

#[test]
fn test_drop_receiver() {
    let (tx, rx) = mpmc::channel::<i16>();

    // drop the receiver
    drop(rx);

    // channel is disconnected, so we should get an error on send
    assert_eq!(tx.send(404), Err(404));
}

#[test]
fn test_disconnected_with_data() {
    let (tx, rx) = mpmc::channel::<i16>();

    // drop the sender _after_ sending some data
    assert!(tx.send(42).is_ok());
    drop(tx);

    // channel still has data, so we should get it (even though sender is dropped)
    assert_eq!(rx.try_recv(), Ok(42));

    // channel is disconnected, so we should get an error on receive
    assert_eq!(rx.try_recv(), Err(TryRecvError::Disconnected));
}
