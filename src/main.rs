use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;

fn main() {
	env_logger::init();
	let mut frequency = HashMap::new();
	let dataset = "datasets/census.csv";
	log::info!("Reading dataset from {}", dataset);
	let file = fs::File::open(dataset).expect("Cannot read dataset");
	let mut reader = csv::Reader::from_reader(file);

	log::info!("Parsing CSV records");
	for record in reader.records() {
		let record = record.expect("Invalid record");

		if let Some(digit) = get_first_digit(&record) {
			log::trace!("Found digit '{}' in {:?}", digit, record);

			let count = frequency.entry(digit).or_insert(0);
			*count += 1;
		} else {
			log::warn!("No valid digit found in {:?}", record);
		}
	}
	log::debug!("Frequency: {:?}", frequency);

	let total: usize = frequency.values().sum();
	let percentage: BTreeMap<char, f32> = frequency
		.into_iter()
		.map(|(digit, count)| (digit, count as f32 / total as f32))
		.collect();

	log::info!("Percentage: {:#.2?}", percentage);
}

fn get_first_digit(record: &csv::StringRecord) -> Option<char> {
	log::trace!("Parsing record: {:?}", record);
	match record.get(1) {
		Some(population) => population
			.chars()
			.next()
			.filter(|c| c.is_ascii_digit() && *c != '0'),
		None => None,
	}
}
