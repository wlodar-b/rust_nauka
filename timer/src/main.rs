use std::thread;
use std::time::Duration;

fn main() {
    let mut minutes = 0;
    'counting_up: loop {
        let mut seconds = 0;

        loop {            
        println!("Actual time {minutes}:{seconds}");
        thread::sleep(Duration::from_secs(1));
            if seconds == 60 {
                break;
            }
            if minutes == 3 {
                break 'counting_up;
            }
            seconds += 1;
        }

        minutes += 1;
    }
    println!("End count = {minutes}");
}
