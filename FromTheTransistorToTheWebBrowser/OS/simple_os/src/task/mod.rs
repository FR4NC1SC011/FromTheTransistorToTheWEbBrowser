use core::{future::Future, pin::Pin};
use alloc::boxed::Box;
use core::task::{Context, Poll};

pub mod simple_executor;
pub mod keyboard;


pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}


impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
