use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use log::{info};
/*
Box :   定义的一个指向堆上的指针
dyn:    关键字区分：到底是是一个特征，还是一个具体的类型,如当我们返回值为一个特征时，可以用这个关键字进行区分
        参考: https://www.insp.top/article/lets-understand-new-keywords-those-impl-and-dyn-of-rust-2018-edition
Send:   该类型表示可以安全的发送到多个线程，该类型是一种移动的类型(非Send类型是指针类型)
‘static: 生命周期，标明该程序存在整个程序的生命周期内
*/
// todo lifetime?
type Job = Box<dyn FnOnce() + Send + 'static>;


// 定义枚举变量
enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /*
        arc : 适用于多线程的引用计数指针
        Mutex: 线程间互斥锁
        Receiver: 消息通道的接收者(多生产者，单消费者)
    */
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        job()
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });
        info!("threadPool worker thread is created: number id {}",id);
        Worker {
            id,
            thread: Some(handle),
        }
    }
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


impl ThreadPool {
    pub fn new(nums: usize) -> Self {
        let (sender, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        let mut workers: Vec<Worker> = Vec::with_capacity(nums);
        for index in 0..nums {
            workers.push(Worker::new(index, Arc::clone(&receiver)));
        };
        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        let result = self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

