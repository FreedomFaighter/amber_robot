pub mod canproxy;
pub(crate) mod cansocket;
pub mod commands;
pub(crate) mod macros;
pub mod messages;
pub mod odrivegroup;
pub mod threads;
pub mod axis;
pub mod response;
pub mod utils;

pub mod test_ui;

#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        messages::{ODriveMessage},
        threads::ReadWriteCANThread, response::ODriveResponse,
    };
    use std::sync::{
        atomic::AtomicBool,
        mpsc::{channel, Receiver, Sender},
        Arc,
    };

    pub(crate) struct ThreadStub {
        pub thread_id: &'static str,
        pub proxy_receiver: Receiver<ODriveMessage>,
        pub proxy_sender: Sender<ODriveResponse>,
        pub rw_communicator: ReadWriteCANThread,
    }

    impl ThreadStub {
        pub fn new(thread_name: &'static str, threads_alive: Arc<AtomicBool>) -> Self {
            let (thread_requester, proxy_receiver) = channel::<ODriveMessage>();
            let (proxy_sender, thread_receiver) = channel::<ODriveResponse>();

            Self {
                thread_id: thread_name,
                proxy_receiver,
                proxy_sender,
                rw_communicator: ReadWriteCANThread::new(
                    thread_name,
                    thread_requester,
                    thread_receiver,
                    threads_alive,
                ),
            }
        }
    }

    pub fn wait_for_msgs<T>(receiver: Receiver<T>) -> T {
        loop {
            match receiver.try_recv() {
                Ok(res) => {
                    return res;
                }
                Err(_) => continue,
            }
        }
    }
}
