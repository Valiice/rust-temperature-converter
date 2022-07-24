use std::{
    fs::File,
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let conversion = get_conversion_type();

    let input = get_number_input();

    let mut file = File::create("output-borrow.txt").expect("Could not create file!");
    let mut vec_times: Vec<Duration> = Vec::new();

    for _ in 0..1000 {
        let now = Instant::now();

        let converted: f64 = convert(&conversion, &input);

        clearscreen::clear().expect("failed to clear screen");

        match conversion {
            'c' => println!("Fahrenheit: {input}°F to Celsius: {converted:.2}°C"),
            'f' => println!("Celsius: {input}°C to Fahrenheit: {converted:.2}°F"),
            _ => panic!("Problem printing result"),
        }

        let elapsed = now.elapsed();
        vec_times.push(elapsed);
        // println!("Elapsed: {:.2?}", elapsed);
    }

    // let now = Instant::now();

    // let converted: f64 = convert(&conversion, &input);

    // clearscreen::clear().expect("failed to clear screen");

    // match conversion {
    //     'c' => println!("Fahrenheit: {input}°F to Celsius: {converted:.2}°C"),
    //     'f' => println!("Celsius: {input}°C to Fahrenheit: {converted:.2}°F"),
    //     _ => panic!("Problem printing result"),
    // }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    for time in vec_times {
        writeln!(&mut file, "{:.2?}", time).expect("Cannot write to file, sorry.");
    }
    thread::sleep(Duration::from_secs(5));
}

fn convert(conversion: &char, input: &i64) -> f64 {
    let mut converted: f64 = 0.0;
    let input = *input as f64;

    match conversion {
        'c' => converted = to_celsius(&input),
        'f' => converted = to_fahrenheit(&input),
        _ => println!("Not a valid conversion type"),
    }
    converted
}

fn to_celsius(input: &f64) -> f64 {
    (input - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(input: &f64) -> f64 {
    (input * 9.0 / 5.0) + 32.0
}

fn get_conversion_type() -> char {
    println!("Please input what you want to convert to (C)elsius / (F)ahrenheit.");
    let conversion = input_str();

    let conversion: char = match conversion.trim().to_lowercase().parse() {
        Ok(num) => num,
        Err(error) => panic!("Problem converting the conversion: {}", error),
    };
    conversion
}

fn get_number_input() -> i64 {
    println!("Please input degrees.");

    let input: i64 = match input_str().trim().parse() {
        Ok(num) => num,
        Err(error) => panic!("Problem converting the input: {}", error),
    };
    input
}

fn input_str() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
