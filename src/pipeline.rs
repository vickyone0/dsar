// use std::sync::mpsc::sync_channel;
// use std::{result, thread};


// pub fn pipe_line(){

//     let (tx1, rx1) = sync_channel(4);

//     let (tx2,rx2) = sync_channel(4);


//     thread::spawn(move|| {
//         for i in 0..10 {
//             tx1.send(i).unwrap();
//         }
//     });

//     thread::spawn(move || {
//         for n in rx1 {
//             tx2.send(n*2).unwrap();
//         }
//     });

//     for result in rx2 {
//         println!("Result: {}", result);
//     }

//     println!("Pipeline complete!");


// }
use std::sync::mpsc::sync_channel;
use std::thread;

pub fn pipe_line(){

    let (tx1,rx1) = sync_channel(4);
    let (tx2,rx2) = sync_channel(4);


    thread::spawn(move ||{
        tx1.send("djdojo".to_string()).unwrap();
    });

    thread::spawn(move||{
        if let Ok(msg) = rx1.recv() {
        tx2.send(msg + "fjjd").unwrap();
    }
    });

    if let Ok(result) = rx2.recv(){
        println!("Result: {}", result);
    }

}