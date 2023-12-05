fn main() {
    println!("Day 5, Problem 1");
    println!("{}", solve(include_str!("input.txt")));
}

fn parse_seeds(input: &str) -> (Vec<u64>, &str) {
    let (line, input) = input.split_once('\n').unwrap();
    let line = line.strip_prefix("seeds: ").unwrap();
    let seeds = line
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let input = input.strip_prefix('\n').unwrap();
    (seeds, input)
}

#[derive(Debug)]
enum MapType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct Instruction {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    from: MapType,
    #[allow(dead_code)]
    to: MapType,
    instructions: Vec<Instruction>,
}

impl Map {
    fn find(&self, value: u64) -> u64 {
        for instruction in &self.instructions {
            if instruction.source_range_start <= value
                && value < instruction.source_range_start + instruction.range_length
            {
                return instruction.destination_range_start
                    + (value - instruction.source_range_start);
            }
        }
        // map to itself
        value
    }
}

#[derive(Debug)]
struct Dataset {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Dataset {
    fn find(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.find(seed);
        let fertilizer = self.soil_to_fertilizer.find(soil);
        let water = self.fertilizer_to_water.find(fertilizer);
        let light = self.water_to_light.find(water);
        let temperature = self.light_to_temperature.find(light);
        let humidity = self.temperature_to_humidity.find(temperature);
        self.humidity_to_location.find(humidity)
    }
}

impl MapType {
    fn parse_tuple(input: &str) -> (Self, Self) {
        let mut i = input.split('-');
        match (i.next(), i.next(), i.next()) {
            (Some(from), Some("to"), Some(to)) => (Self::from(from), Self::from(to)),
            other => panic!("failed to parse map tuple: {:?}", other),
        }
    }
}

impl From<&str> for MapType {
    fn from(value: &str) -> Self {
        match value {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            other => panic!("unknown map type {other}"),
        }
    }
}

fn parse_map(input: &str) -> (Map, &str) {
    let (header, mut rest) = input.split_once('\n').unwrap();
    let (name, _) = header.split_once(' ').unwrap();
    let (from_type, to_type) = MapType::parse_tuple(name);
    let mut instructions = vec![];

    loop {
        let splitted = rest.split_once('\n');
        if splitted.is_none() {
            break;
        }
        let splitted = splitted.unwrap();
        let line = splitted.0;
        rest = splitted.1;
        if line.is_empty() {
            break;
        }

        let (n1, line) = line.split_once(' ').unwrap();
        let (n2, line) = line.split_once(' ').unwrap();
        let n3 = line;

        let destination_range_start = n1.parse::<u64>().unwrap();
        let source_range_start = n2.parse::<u64>().unwrap();
        let range_length = n3.parse::<u64>().unwrap();

        instructions.push(Instruction {
            destination_range_start,
            source_range_start,
            range_length,
        });
    }

    (
        Map {
            from: from_type,
            to: to_type,
            instructions,
        },
        rest,
    )
}

fn solve(input: &str) -> u64 {
    let (seeds, input) = parse_seeds(input);
    let (seed_to_soil, input) = parse_map(input);
    let (soil_to_fertilizer, input) = parse_map(input);
    let (fertilizer_to_water, input) = parse_map(input);
    let (water_to_light, input) = parse_map(input);
    let (light_to_temperature, input) = parse_map(input);
    let (temperature_to_humidity, input) = parse_map(input);
    let (humidity_to_location, _input) = parse_map(input);

    let dataset = Dataset {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };

    // dbg!(&dataset);

    seeds.iter().map(|seed| dataset.find(*seed)).min().unwrap()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem1-input-small.txt")), 35);
}
