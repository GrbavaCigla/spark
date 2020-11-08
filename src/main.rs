use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    numbers: String,

    #[structopt(long)]
    min: Option<i32>,

    #[structopt(long)]
    max: Option<i32>,

    #[structopt(short, long)]
    delimiter: Option<char>
}

fn get_bar(bars: &[char], min: f32, max: f32, value: f32) -> Option<char> {
    if value < min || value > max {
        return None;
    }

    let index =(value - min) * 7_f32 / (max - min);
    Some(bars[index.round() as usize])
}

fn main() {
    let bars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let args = Cli::from_args();

    let delim = match args.delimiter {
        Some(delim) => delim,
        None => ','
    };

    let numbers: Vec<i32> = args.numbers.split(delim)
    .map(|s| s.parse()
    .unwrap_or_else(|_| {
        eprintln!("Expected a number!");
        std::process::exit(1)
    })).collect();


    let min = match args.min{
        Some(min) => min,
        None => match numbers.iter().min(){
            Some(min) => *min,
            None => panic!("Could not find min value. Report a bug at https://github.com/GrbavaCigla/spark")
        }
    };

    let max = match args.max{
        Some(max) => max,
        None => match numbers.iter().max(){
            Some(max) => *max,
            None => {
                eprintln!("Could not find max value. Report a bug at https://github.com/GrbavaCigla/spark");
                std::process::exit(1);
            }
        }
    };

    for i in numbers.iter(){
        let bar = get_bar(&bars, min as f32, max as f32, *i as f32);
        let bar = match bar {
            Some(bar) => bar,
            None => panic!("Could value is smaller than min value or bigger than max value. Report a bug at https://github.com/GrbavaCigla/spark")
        };
        print!("{}", bar);
    }
}
