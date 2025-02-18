use anyhow::Error;
use chrono::Local;
use rand::Rng;
use rand_chacha::{ChaCha20Rng, ChaCha8Rng};
use rand_core::{RngCore, SeedableRng};
use rand_distr::{Distribution, Normal};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

static MENTAL_CONCEPT_ARCHETYPES: OnceLock<ThoughtConceptList> = OnceLock::new();
pub const MENTAL_CONCEPT_ARCHETYPE_FILE: &str =
    "/home/kagemnikarimu/Projects/RustProjects/nyota/src/lex/eff_large_wordlist.txt";
const DEFAULT_TIMES_THOUGHT: u32 = 0;
type ThoughtConceptList = Mutex<Vec<ThoughtConcept>>;

pub fn test_mental_concepts() {
    load_mental_concepts(MENTAL_CONCEPT_ARCHETYPE_FILE).unwrap();
    let length = MENTAL_CONCEPT_ARCHETYPES
        .get()
        .unwrap() // get from OnceCell
        .lock() // get lock on Mutex
        .unwrap() // unwrap Mutex result
        .len(); // get Vec length
    println!("{:#?}", length);
}

pub fn load_mental_concepts(from_file_path: &str) -> Result<(), Error> {
    let file = File::open(from_file_path)?;
    let buffer_file = BufReader::new(file);

    let parsed_concepts = parse_eff_large_wordlist(buffer_file)?;

    // Initialize the OnceCell with an empty Vec in a Mutex
    let mut archetypes = MENTAL_CONCEPT_ARCHETYPES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap();

    for concept in parsed_concepts {
        let thought_concept = ThoughtConcept {
            name: concept,
            times_thought: DEFAULT_TIMES_THOUGHT,
        };

        archetypes.push(thought_concept);
    }
    Ok(())
}

fn parse_eff_large_wordlist(buffered_file: BufReader<File>) -> Result<Vec<String>, Error> {
    let mut list: Vec<String> = Vec::new();
    for line in buffered_file.lines() {
        match line {
            Ok(item) => list.push(item.split_once("\t").unwrap_or(("", "")).1.to_string()),
            Err(e) => {
                return Err(Error::msg(format!(
                    "Error reading line from eff_large_wordlist.txt: {}",
                    e
                )));
            }
        }
    }
    Ok(list)
}

const THOUGHT_FREQUENCY: f64 = 5.0;
const THOUGHT_FREQUENCY_STANDARD_DEVIATION: f64 = 1.0;
const THOUGHT_FREQUENCY_VARIANCE_PERCENT: f64 = 0.15;
const FIRST_THOUGHTS: Vec<String> = Vec::new();

const THOUGHT_OUTLIER_TIME_MULTIPLIER_MIN: f64 = 0.2; // 5x faster than normal
const THOUGHT_OUTLIER_TIME_MULTIPLIER_MAX: f64 = 10.0; // 10x slower than normal
const THOUGHT_OUTLIER_TIME_MULTIPLIER_RANGE: f64 =
    THOUGHT_OUTLIER_TIME_MULTIPLIER_MAX - THOUGHT_OUTLIER_TIME_MULTIPLIER_MIN;

pub struct StreamOfThought<'a> {
    seed_of_consciousness: ChaCha8Rng,
    thought_frequency: Normal<f64>,
    concepts: &'a OnceLock<ThoughtConceptList>,
    pub last_thought_stream: Vec<String>,
    pub last_thought_time: chrono::DateTime<Local>,
}

#[derive(Debug)]
struct ThoughtConcept {
    name: String,
    times_thought: u32,
}

impl<'a> StreamOfThought<'a> {
    pub fn new(path_to_seed_file: &str) -> Self {
        load_mental_concepts(path_to_seed_file).unwrap();
        Self {
            seed_of_consciousness: Self::awaken(),
            thought_frequency: Self::set_thought_frequency(),
            concepts: &MENTAL_CONCEPT_ARCHETYPES,
            last_thought_stream: FIRST_THOUGHTS,
            last_thought_time: Local::now(),
        }
    }

    fn awaken() -> ChaCha8Rng {
        ChaCha8Rng::from_rng(&mut rand::rng())
    }

    fn set_thought_frequency() -> Normal<f64> {
        Normal::new(THOUGHT_FREQUENCY, THOUGHT_FREQUENCY_STANDARD_DEVIATION).unwrap()
    }

    fn get_frequent_thoughts(&self, limit: usize) -> Vec<String> {
        let concepts = self.concepts.get().unwrap().lock().unwrap();
        let mut concept_vec: Vec<&ThoughtConcept> = concepts.iter().collect();
        concept_vec.sort_by(|a, b| b.times_thought.cmp(&a.times_thought));
        concept_vec
            .iter()
            .take(limit)
            .map(|c| c.name.clone())
            .collect()
    }

    fn learn_new_concept(&mut self, concept: String) -> Result<(), Error> {
        let mut concepts = self.concepts.get().unwrap().lock().unwrap();
        let new_concept = ThoughtConcept {
            name: concept,
            times_thought: 1,
        };
        concepts.push(new_concept);
        Ok(())
    }

    pub fn think(&mut self) {
        let time_until_next_thought = self.get_time_until_next_thought();
        std::thread::sleep(time_until_next_thought);
        self.last_thought_time = Local::now();
        self.last_thought_stream = self.generate_thought_stream();
    }

    fn generate_thought_stream(&mut self) -> Vec<String> {
        // Get access to our concepts list
        let mut concepts = self.concepts.get().unwrap().lock().unwrap();

        // Create vector to hold our stream of thoughts - now with capacity for 8 words
        let mut thought_stream = Vec::with_capacity(8);

        // Generate a random hash
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.seed_of_consciousness.next_u64().to_le_bytes());
        let hash = hasher.finalize();

        // Use all 32 bytes, giving us 8 chunks of 4 bytes each
        for chunk in hash.chunks(4) {
            let num = u32::from_le_bytes(chunk.try_into().unwrap_or([0; 4]));
            let index = (num % concepts.len() as u32) as usize;

            // Get the concept and increment its thought count
            let concept = &mut concepts[index];
            concept.times_thought += 1;

            thought_stream.push(concept.name.clone());
        }

        thought_stream
    }

    fn get_time_until_next_thought(&mut self) -> Duration {
        // Base interval (normal distribution around 5 seconds)
        let normal_thought_interval = self
            .thought_frequency
            .sample(&mut self.seed_of_consciousness)
            * 1000.0;

        // Occasionally generate outlier intervals
        let time_until_next_thought =
            if self.seed_of_consciousness.random::<f64>() < THOUGHT_FREQUENCY_VARIANCE_PERCENT {
                normal_thought_interval
                    * (self.seed_of_consciousness.random::<f64>()
                        * THOUGHT_OUTLIER_TIME_MULTIPLIER_RANGE
                        + THOUGHT_OUTLIER_TIME_MULTIPLIER_MIN)
            } else {
                normal_thought_interval
            };

        Duration::from_millis(time_until_next_thought as u64)
    }
}
