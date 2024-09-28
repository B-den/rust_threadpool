use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};

struct Thread<F>
where F: Fn() -> () + 'static + std::marker::Send {
    handler: thread::JoinHandle<()>,
    input: mpsc::Sender<F>,
    // out_chan: mpsc::Sender<Result<(), ()>>,
}

pub struct ThPool<F>
where F: Fn() -> () + 'static + std::marker::Send {
    manager_threads: Vec<Option<thread::JoinHandle<()>>>,
    task_chan: Option<Sender<F>>,
    // working_threads: Vec<Option<Thread<F>>>,
    // task_queue: Arc<Mutex<VecDeque<F>>>,
    // is_running: Arc<Mutex<bool>>,
    tasks_finished: Arc<(Mutex<bool>, Condvar)>,
}

impl<F> ThPool<F>
where F: Fn() -> () + 'static + std::marker::Send {
    pub fn new(executors_num: usize) -> Self {
        let (task_sender, task_receiver) = mpsc::channel();

        let mut this = Self{
            manager_threads: vec![],
            task_chan: Some(task_sender),
            // working_threads: vec![],
            // task_queue: Arc::new(Mutex::new(VecDeque::new())),
            // is_running: Arc::new(Mutex::new(true)),
            tasks_finished: Arc::new((Mutex::new(false), Condvar::new())),
        };

        this.start_manager(task_receiver, executors_num);

        return this;
    }

    pub fn enqueue(&mut self, task: F) -> Result<(), mpsc::SendError<F>> {
        // let mut queue = self.task_queue.lock().unwrap();
        // queue.push_back(task);
        self.task_chan.as_ref().unwrap().send(task)
    }

    fn start_manager(&mut self, task_chan: Receiver<F>, executors_num: usize) {
        // let mut task_queue: Arc<Mutex<VecDeque<F>>> = self.task_queue.clone();
        // let is_running = self.is_running.clone();
        let tasks_finished = self.tasks_finished.clone();

        let managing_func = move || {
            let mut executors = Vec::<Option<Thread<F>>>::new();
            executors.resize_with(executors_num, || None);

            let (signal_sender, signal_receiver) = mpsc::channel();
            
            for i in 0..executors_num {
                let ready_sender = signal_sender.clone();
                let (task_sender, task_receiver) = mpsc::channel::<F>();
                let exec_func = move || {
                    let id = i;
                    let _ = ready_sender.send(id);
                    
                    while let Ok(task) = task_receiver.recv() {
                        task();
                        let _ = ready_sender.send(id);
                    }
                };

                executors[i] = Some(Thread {
                    handler: thread::spawn(exec_func),
                    input: task_sender,
                });
            }


            let send_task = |id: usize, task: F| {
                let exec_thread = executors[id].as_ref();
                let _ = exec_thread.unwrap().input.send(task);
            };

            // let is_stop = || -> bool {
            //     let continue_execution = is_running.lock().unwrap();
            //     return !*continue_execution;
            // };

            loop {
                // if is_stop() {
                //     break;
                // }

                if let Ok(task) = Self::next_task(&task_chan) {
                    let exec_id = Self::next_executor(&signal_receiver);
                    send_task(exec_id, task);
                } else {
                    break;
                }

                // thread::yield_now();
            }

            while executors.len() > 0 {
                let th = executors.remove(0).unwrap();

                drop(th.input);
                th.handler.join().unwrap();
            }

            let (lock, cvar) = &*tasks_finished;
            let mut done = lock.lock().unwrap();
            *done = true;
            cvar.notify_one();
        };

        self.manager_threads.push(Some(thread::spawn(managing_func)));
    }

    pub fn finish(&mut self) {
        // {
        //     let mut is_running = self.is_running.lock().unwrap();
        //     *is_running = false;
        // }
        self.task_chan = None;

        let (lock, cvar) = &*self.tasks_finished;
        let mut done = lock.lock().unwrap();
        while !*done {
            done = cvar.wait(done).unwrap();
        }

        while self.manager_threads.len() > 0 {
            let handle = self.manager_threads.remove(0);
            if let Some(handle) = handle {
                handle.join().unwrap();
            }
        }
    }

    fn next_task(task_chan: &Receiver<F>) -> Result<F, mpsc::RecvError> {
        task_chan.recv()
    }

    fn next_executor(chan: &Receiver<usize>) -> usize {
        if let Ok(id) = chan.recv() {
            return id;
        }

        return 0;
    }
}


impl<F> Drop for ThPool<F>
where F: Fn() -> () + 'static + std::marker::Send {
    fn drop(&mut self) {
        while self.manager_threads.len() > 0 {
            let handle = self.manager_threads.remove(0);
            if let Some(handle) = handle {
                handle.join().unwrap();
            }
        }

    }
}