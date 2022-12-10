use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn boil_teapot() -> JoinHandle<()>{
    thread::spawn(|| {
        println!("Teapot put on the stove");
        thread::sleep(Duration::from_millis(50));
        println!("Pouring boiling water into a cup!");
    })
}

fn roast_toasts() -> JoinHandle<()>{
    thread::spawn(|| {
        println!("Put first toast");
        thread::sleep(Duration::from_millis(30));
        println!("Finished first toast");

        println!("Put second toast");
        thread::sleep(Duration::from_millis(20));
        println!("Finished second toast");

        println!("Putting roasted toasts on a plate");
    })
}

pub fn make_breakfast()
{
    let mut handles: Vec<_> = vec![];
    handles.push(boil_teapot());
    handles.push(roast_toasts());

    for handle in handles{
        handle.join().unwrap();
    }
}