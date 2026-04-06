use std::marker::PhantomData;

#[derive(Debug)]
pub struct Sender<T> {
    // TODO: implement
    _delete_me: PhantomData<T>,
}

#[derive(Debug)]
pub struct Receiver<T> {
    // TODO: implement
    _delete_me: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TryRecvError {
    Empty,
    Disconnected,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    unimplemented!()
}

impl<T> Sender<T> {
    pub fn send(&self, val: T) -> Result<(), T> {
        unimplemented!("phase 1")
    }
}

impl<T> Receiver<T> {
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        unimplemented!("phase 1")
    }

    pub fn recv(&self) -> Option<T> {
        unimplemented!("phase 3")
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        unimplemented!("phase 2")
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        unimplemented!("phase 2")
    }
}
