use std::thread;
use std::time::Duration;

fn main() {
    let mut seconds = 0;
    let mut minutes = 2;

    while minutes != -1 {
        thread::sleep(Duration::from_secs(1));
        println!("Time to liftoff: {minutes}:{seconds}!");
        seconds -= 1;

        if seconds == -1 {
            seconds += 60;
            minutes -= 1;
        }

        if minutes == -1 {
            break;
        }
    }
    println!("LIFTOFF!!!");
}
