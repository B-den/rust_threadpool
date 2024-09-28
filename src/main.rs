// mod threadpool;
mod executor;

use std::thread;
use std::time::Duration;
use std::time::Instant;

// fn run_tasks(tasks_num: usize, thread_count: usize) {
//     let mut pool = threadpool::Pool::new(thread_count);
    
//     pool.exec();
//     for _ in 0..tasks_num {
//         let f = move || -> () {
//             thread::sleep(Duration::from_millis(10));
//             // println!("{i}, {}", thread::current().name().unwrap());
//         };
//         pool.enqueue(f);
        
//         // thread::sleep(Duration::from_millis(10));
//     }
    
//     pool.finish();
// }

// fn firstv() {
//     let tasks_num = 50;
//     let th_cnts = vec![1, 2, 5, 10, 20];
//     // let th_cnts = vec![3];
    
//     for th_cnt in th_cnts {
//         let now = Instant::now();
//         run_tasks(tasks_num, th_cnt);
//         let elapsed_time = now.elapsed().as_millis();
    
//         println!("tasks: {tasks_num}, threads: {th_cnt}, time: {elapsed_time}");
//     }
    
//     println!("done");
// }

fn main() {
    
    let tasks_num = 100;
    let th_cnts = vec![1, 2, 5, 10, 20, 50];
    
    for th_cnt in th_cnts {
        let now = Instant::now();
        let mut exer = executor::ThPool::new(th_cnt);
        
        for i in 0..tasks_num {
            let f = move || -> () {
                thread::sleep(Duration::from_millis(10));
            };

            if let Err(e) = exer.enqueue(f) {
                println!("{e}");
            }
        }

        exer.finish();

        let elapsed_time = now.elapsed().as_millis();
        println!("tasks: {tasks_num}, threads: {th_cnt}, time: {elapsed_time}");
    }
    
    println!("done");
}
