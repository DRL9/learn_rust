use std::thread;
use std::time::Duration;

fn main() {
    println!("hello world");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    println!("where is the room?");
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let closure = |intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    let mut cache = Cache::new(closure);

    if intensity < 25 {
        println!("Today, do {} pushups!", cache.value(intensity));
        println!("Next, do {} situps!", cache.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cache.value(intensity));
        };
    }
}

struct Cache<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cache<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let value = (self.calculation)(arg);
                self.value = Some(value);
                value
            }
        }
    }
}
