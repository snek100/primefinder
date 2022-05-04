use std::io;
use std::thread;
use num_integer::Roots;
use num_cpus::get;
use indicatif::ProgressBar;
use ringbuf::RingBuffer;
use std::time::Duration;
fn main() {
    let user_given_number = asks_user_for_number();
    let divisor = finds_if_number_is_prime(user_given_number);
        
    if divisor != 1 {
    if divisor != 0 {
            println!("{} is not prime, it is divisible by {}", user_given_number, divisor)
    }
    else {
            println!("1 is not prime, because it's the zero (which is niether positive, nor negative) of multiplication, and therefore is niether prime nor composite. (niether is a building block, nor is built)")
    }
    }
    else {
        println!("{} is prime.", user_given_number);
    }
}


fn finds_if_number_is_prime(number_to_check:u128) -> u128 {
    if number_to_check == 1 {
        return 0;
    }
    else {
        let number_of_threads:u128 = ((get()) as u128) * 2;
        println!("spinning up {} threads", number_of_threads);
        let progress_bar = ProgressBar::new(number_to_check.sqrt().try_into().unwrap());
        let mut count2:u128 = 1;
        let mut thread_group_ring_buffers_divisor = vec![];
        let mut thread_group_ring_buffers_stop = vec![];
        let mut thread_group_ring_buffers_work = vec![];
        let mut threads_group = vec![];
        for thread_number in 0..number_of_threads {
           // let progress_bar_clone = progress_bar.clone();
            let this_thread_ring_buffer_divisor = RingBuffer::<u128>::new(1);
            let this_thread_ring_buffer_stop = RingBuffer::<bool>::new(1);
            let this_thread_ring_buffer_work = RingBuffer::<u128>::new(1024);
            let (mut this_thread_ring_buffer_divisor_write, this_thread_ring_buffer_divisor_read) = this_thread_ring_buffer_divisor.split();
            let (this_thread_ring_buffer_stop_write, this_thread_ring_buffer_stop_read) = this_thread_ring_buffer_stop.split();
            let (mut this_thread_ring_buffer_work_write, this_thread_ring_buffer_work_read) = this_thread_ring_buffer_work.split();
            thread_group_ring_buffers_divisor.push(this_thread_ring_buffer_divisor_read);
            thread_group_ring_buffers_stop.push(this_thread_ring_buffer_stop_write);
            thread_group_ring_buffers_work.push(this_thread_ring_buffer_work_read);
            threads_group.push(thread::spawn(move || {
                 let root:u128 = number_to_check.sqrt().try_into().unwrap();
                 let mut count = 3 + (thread_number * 2); 
                 if 2 > root {
                     this_thread_ring_buffer_divisor_write.push(1).unwrap();
                     return (true, 1);
                 }
                 if number_to_check % 2 == 0 {
                     this_thread_ring_buffer_divisor_write.push(2).unwrap();
                     return (false, 2);
                 }

                 loop {

                    if count > root {
                        this_thread_ring_buffer_work_write.push(count2 * 2).unwrap();
                        thread::sleep(Duration::from_millis(100));
                        this_thread_ring_buffer_divisor_write.push(1).unwrap();
                        return (true, 1);
                     }       
                 
                     if number_to_check % count == 0 {
                        this_thread_ring_buffer_divisor_write.push(count).unwrap();
                        return (false, count);
                     }

                     if count2 != 0 {
                     }
                     else {
                        if this_thread_ring_buffer_stop_read.is_empty() {
                        }
                        else {
                        println!("recieved stop command, thread {} stopping",thread_number);
                        return (false, 0);
                        }
                        this_thread_ring_buffer_work_write.push(2097152).unwrap();
                        //progress_bar_clone.inc(4194304);
                    }

                     count = count + (number_of_threads * 2);
                     count2 = (count2 + 1) & 1048575;
                 }

            }));
        }
       // let mut andy:bool = true;
        let mut divisor:u128 = 0;
        let mut received = 0;
        let mut done_threads = vec![];
        println!("threads started");
        loop {
          //  thread::sleep(Duration::from_millis(10));
           // let mut done_threads = vec![];
            let mut all_done = false;
            for this_thread_ring_buffer_divisor_read in &mut thread_group_ring_buffers_divisor {
                if this_thread_ring_buffer_divisor_read.is_empty() {
                   // thread::sleep(Duration::from_millis(10))
                }
                else {
                    received = this_thread_ring_buffer_divisor_read.pop().unwrap();
                    if received > 1 {
                        break
                    }
                    if received == 1 {
                        done_threads.push(1);
                       // progress_bar.tick();
                       // thread::sleep(Duration::from_millis(10));
                        let mut count = 0;
                        for &mut thread in &mut done_threads {
                            count = count + thread
                        }
                        if count == number_of_threads {
                            all_done = true;
                            break 
                        }
                    }
                }
            }
            if received > 1 {
                break
            }
                for this_thread_ring_buffer_work_read in &mut thread_group_ring_buffers_work {
                    if this_thread_ring_buffer_work_read.is_empty() {
                      //  thread::sleep(Duration::from_millis(10));
                       // progress_bar.tick();
                    }
                    else {
                        let work_progress = this_thread_ring_buffer_work_read.pop().unwrap();
                        progress_bar.inc(work_progress.try_into().unwrap())
                    }
                }
            if all_done {
                progress_bar.finish();
                break
            }
        }
            if received != 0 {
            for this_thread_ring_buffer_stop_write in &mut thread_group_ring_buffers_stop {
                this_thread_ring_buffer_stop_write.push(true).unwrap();
            }
            divisor = received;
            //andy = false;
            }
            else {
            println!("there was an issue recieving the divisor from a thread");
            }
        //}
    return divisor
    }
}

fn asks_user_for_number() -> u128 {
    loop {
        println!("Enter any positive integer less than 2^128");
        let mut string_to_recieve_input = String::new();
            io::stdin()
                .read_line(&mut string_to_recieve_input)
                .expect("Failed to read input");
            let user_input:u128 = match string_to_recieve_input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
        if user_input > 0 {
            return user_input
        }
        else {
            println!("Not a positive integer less than 2^128, try again.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
       // let mut primeiness = false;
        let big_prime_chart:Vec<bool> = vec![false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false];
        for checking in 1..big_prime_chart.len() {
        let divisor = finds_if_number_is_prime(checking.try_into().unwrap());
        let mut primeiness = false;
        if divisor != 1 {
            primeiness = false;
        }
        else {
            primeiness = true;
        }
        println!("{}", checking);
        assert_eq!(big_prime_chart[(checking - 1)], primeiness);
        }
    }
}

