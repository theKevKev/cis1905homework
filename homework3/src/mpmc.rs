use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug)]
pub struct Sender<T> {
    inner: Arc<ChannelInner<T>>,
}

#[derive(Debug)]
pub struct Receiver<T> {
    inner: Arc<ChannelInner<T>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TryRecvError {
    Empty,
    Disconnected,
}

#[derive(Debug)]
struct ChannelInner<T> {
    num_senders: AtomicUsize,
    num_receivers: AtomicUsize,
    buf: Mutex<VecDeque<T>>,
    can_read: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(ChannelInner {
        num_senders: AtomicUsize::new(1),
        num_receivers: AtomicUsize::new(1),
        buf: Mutex::new(VecDeque::new()),
        can_read: Condvar::new(),
    });
    let sender = Sender {
        inner: Arc::clone(&inner),
    };
    let receiver = Receiver { inner };
    (sender, receiver)
}

impl<T> Sender<T> {
    pub fn send(&self, val: T) -> Result<(), T> {
        if self.inner.is_receiver_closed() {
            return Err(val);
        }
        let mut buffer = self.inner.buf.lock().unwrap();
        let should_notify = buffer.is_empty();
        buffer.push_back(val);
        drop(buffer);
        if should_notify {
            self.inner.can_read.notify_all();
        }
        Ok(())
    }
}

impl<T> Receiver<T> {
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        if self.inner.is_sender_closed() {
            return Err(TryRecvError::Disconnected);
        }
        let value = self.inner.buf.lock().unwrap().pop_front();
        match value {
            None => Err(TryRecvError::Empty),
            Some(val) => Ok(val),
        }
    }

    pub fn recv(&self) -> Option<T> {
        let result = self.try_recv();
        match result {
            Err(TryRecvError::Disconnected) => None,
            Ok(val) => Some(val),
            Err(TryRecvError::Empty) => {
                let mut buf = self
                    .inner
                    .can_read
                    .wait_while(self.inner.buf.lock().unwrap(), |buf| {
                        buf.is_empty() && self.inner.num_senders.load(Ordering::SeqCst) > 0
                    })
                    .unwrap();
                buf.pop_front()
            }
        }
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.inner.num_senders.fetch_add(1, Ordering::SeqCst);
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        self.inner.num_receivers.fetch_add(1, Ordering::SeqCst);
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.inner.num_senders.fetch_sub(1, Ordering::SeqCst);
        self.inner.can_read.notify_all();
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.inner.num_receivers.fetch_sub(1, Ordering::SeqCst);
    }
}

impl<T> ChannelInner<T> {
    fn is_sender_closed(&self) -> bool {
        self.num_senders.load(Ordering::SeqCst) == 0 && self.buf.lock().unwrap().is_empty()
    }

    fn is_receiver_closed(&self) -> bool {
        self.num_receivers.load(Ordering::SeqCst) == 0
    }
}
