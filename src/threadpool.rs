use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::{thread, vec::Vec};
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum ThreadStatus {
    Ready,
    Busy,
}

pub struct Pool<F>
where F: FnOnce() -> () + std::marker::Send + Copy + 'static {
    threads: Vec::<thread::JoinHandle::<()>>,
    thread_count: usize,
    senders: Arc<Mutex<Vec::<mpsc::Sender<Option<F>>>>>,
    task_queue: Arc<Mutex<VecDeque<F>>>,
    statuses: Arc<Mutex<Vec<Arc<Mutex<ThreadStatus>>>>>,
    is_running: Arc<Mutex<bool>>,
}

impl<F> Pool<F>
where F: FnOnce() -> () + std::marker::Send + Copy + 'static {
    pub fn new(thread_count: usize) -> Self {
        Self{
            threads: vec![],
            thread_count: thread_count,
            senders:  Arc::new(Mutex::new(Vec::<mpsc::Sender<Option<F>>>::new())),
            statuses: Arc::new(Mutex::new(Vec::<Arc<Mutex<ThreadStatus>>>::new())),
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            is_running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn exec(&mut self) {
        // let mut statuses: Vec<Arc<Mutex<ThreadStatus>>>;
        // { statuses = self.statuses.lock().unwrap().clone(); }
        // statuses.reserve(self.thread_count);
        // let statuses_ptr = &mut statuses;
        {
        let mut statuses = self.statuses.lock().unwrap();
        for i in 0..self.thread_count {
            let (tx, rx) = mpsc::channel();
            {
                let mut senders = self.senders.lock().unwrap();
                senders.push(tx);
            }
            
            statuses.push(Arc::new(Mutex::new(ThreadStatus::Ready)));
            let status: Arc<Mutex<ThreadStatus>> = statuses.last().unwrap().clone();
            
            let f = move || -> () {
                let change_state = |new_state: ThreadStatus| {{
                    let mut state = status.lock().unwrap();
                    *state = new_state;
                }};

                while let Ok(recvd) = rx.recv() {
                    match recvd {
                        Some(func) => {
                            change_state(ThreadStatus::Busy);
                            func();
                            change_state(ThreadStatus::Ready);
                        },
                        None => return,
                    }
                }
            };


            let builder = thread::Builder::new().name(i.to_string());
            let thread_handle: Result<thread::JoinHandle<()>, std::io::Error> = builder.spawn(f);

            self.threads.push(thread_handle.unwrap());
        }

        }

        self.run_control_thread();
    }

    pub fn enqueue(&mut self, task: F) {
        let mut queue = self.task_queue.lock().unwrap();
        queue.push_back(task);
    }

    pub fn finish(&mut self) {
        loop {
            {
                let queue = self.task_queue.lock().unwrap();
                if queue.is_empty() {
                    break;
                }
            }

            thread::yield_now();
        }

        {
            let mut run = self.is_running.lock().unwrap();
            *run = false;
        }

        self.send_all(None);
        while self.threads.len() > 0 {
            let thread_handle = self.threads.remove(0);
            thread_handle.join().unwrap();
        }
    }

    fn send_all(&mut self, func: Option<F>) {
        let senders = self.senders.lock().unwrap();
        senders.iter().for_each(|tx: &mpsc::Sender<Option<F>>| tx.send(func).unwrap());
    }

    fn run_control_thread(&mut self) {
        // let (tx, rx) = mpsc::channel();
        // {
        //     let mut senders = self.senders.lock().unwrap();
        //     senders.push(tx);
        // }

        let is_running = self.is_running.clone();
        let th_senders: Arc<Mutex<Vec<mpsc::Sender<Option<F>>>>> = self.senders.clone();
        let task_queue = self.task_queue.clone();
        let statuses = self.statuses.clone();
        let f = move || -> () {
            let find_available = || -> usize {
                loop {
                    let s = statuses.lock().unwrap();
                    for i in 0..s.len() {{
                        let state = s[i].lock().unwrap();
                        if *state == ThreadStatus::Ready {
                            return i;
                        }
                    }}
                }
            };

            let fetch_task = || -> Option<F> {
                let mut queue = task_queue.lock().unwrap();
                if !queue.is_empty() {
                    return queue.pop_front();
                }

                return None;
            };

            let stop = || -> bool {
                let continue_execution = is_running.lock().unwrap();
                return !*continue_execution;
            };

            loop {
                if stop() {
                    break;
                }

                let sender_index = find_available();
                let res = fetch_task();

                if let Some(task) = res {{
                    let senders = th_senders.lock().unwrap();
                    let _ = senders[sender_index].send(Some(task));
                }} else {
                    thread::yield_now();
                }
            }
        };

        let builder = thread::Builder::new().name(self.thread_count.to_string());
        let thread_handle: Result<thread::JoinHandle<()>, std::io::Error> = builder.spawn(f);
        
        self.threads.push(thread_handle.unwrap());
    }
}