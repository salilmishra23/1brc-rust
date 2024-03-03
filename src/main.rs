use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Measurements {
    sum: f64,
    count: u64,
    min: f32,
    max: f32,
}

impl Measurements {
    fn new(item: f32) -> Measurements {
        Measurements {
            sum: item as f64,
            count: 1,
            min: item,
            max: item,
        }
    }

    fn update(&mut self, item: f32) {
        self.sum += item as f64;
        self.count += 1;
        self.min = item.min(self.min);
        self.max = item.max(self.max);
    }

    fn mean(&self) -> f64 {
        self.sum / (self.count as f64)
    }
}

impl fmt::Display for Measurements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1};{:.1},{:.1}", self.min, self.mean(), self.max)
    }
}

fn main() {
    let f = File::open("./measurements.txt").unwrap();
    let reader = BufReader::new(f);

    let mut map: BTreeMap<String, Measurements> = BTreeMap::new();

    for line in reader.lines() {
        if let Some((city, temp)) = line.unwrap().split_once(';') {
            let temp: f32 = temp.parse().unwrap();
            map.entry(city.to_string())
                .and_modify(|item| item.update(temp))
                .or_insert(Measurements::new(temp));
        }
    }
    for (key, value) in map.iter() {
        println!("{};{}", key, value);
    }
}
