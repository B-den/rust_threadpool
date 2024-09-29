// mod threadpool;
mod executor;
mod tree;

use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::tree::Node;

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

fn exec_test() {
    let task_duration_ms = 1;
    let tasks_nums: Vec<i32> = vec![10, 50, 100, 500];
    let th_cnts = vec![1, 5, 10, 50];

    for th_cnt in th_cnts {
        let mut exer = executor::ThPool::new(th_cnt);

        for tasks_num in tasks_nums.clone() {
            let now = Instant::now();
            for _ in 0..tasks_num {
                let f = move || -> () {
                    thread::sleep(Duration::from_millis(task_duration_ms));
                };

                if let Err(e) = exer.enqueue(f) {
                    println!("{e}");
                }
            }

            exer.finish();

            let elapsed_time = now.elapsed().as_millis();
            println!("{tasks_num}, {th_cnt}, {elapsed_time}");
            // println!("tasks: {tasks_num}, threads: {th_cnt}, time: {elapsed_time}ms");
        }
    }

    println!("done");
}

fn main() {
    let mut exer = executor::ThPool::new(2);

    // let tree_orig = Arc::new(Mutex::new(tree::Node::<i32>::new(0)));
    // {
    //     let mut tree = tree_orig.lock().unwrap();
    //     for i in 1..5 {
    //         tree.push(i);
    //     }
    
    //     tree.iter().for_each(|n| print!("{} ", n.val));
    //     println!();
    
    //     let mut i = 11;
    //     let mut j = 111;
    //     for child in tree.iter_mut() {
    //         child.push(i);
    //         for ch in child.iter_mut() {
    //             ch.push(j);
    //             j += 1;
    //             ch.push(j);
    //             j += 1;
    //         }
    //         i += 1;
    //     }

    // }

    // let tree_orig = Arc::new(Mutex::new(tree::Node::<i32>::new(0)));
    let mut tree = tree::Node::<i32>::new(0);
    for i in 1..5 {
        tree.push(i);
    }

    tree.iter().for_each(|n| print!("{} ", n.val));
    println!();

    let mut i = 11;
    let mut j = 111;
    for child in tree.iter_mut() {
        child.push(i);
        for ch in child.iter_mut() {
            ch.push(j);
            j += 1;
            ch.push(j);
            j += 1;
        }
        i += 1;
    }


    fn node_print<S: Into<String> + Clone>(prefix: S) -> impl Fn(&Node<i32>) {
        move |node: &Node<i32>| {
            println!("{}{}", prefix.clone().into(), node.val);
            node.iter().for_each(node_print(prefix.clone().into() + "\t"));                
        }
    }

    // let tree = tree_orig.clone();
    // let _ = exer.enqueue(move || {
    //     let locked_tree = tree.lock().unwrap();
    //     locked_tree.iter().for_each(node_print("".to_string()));
    // });

    let tree_cp = Arc::new(tree);
    let _ = exer.enqueue(move || {
        tree_cp.iter().for_each(node_print(""));
    });

    exer.finish();
    
}
