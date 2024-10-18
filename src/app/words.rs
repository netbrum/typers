use rand::seq::IteratorRandom;

const WORDS: &str = include_str!("../../words/en1000");

pub struct Words;

impl Words {
    pub fn generate(n: usize) -> Vec<&'static str> {
        WORDS.lines().choose_multiple(&mut rand::thread_rng(), n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_exact_words() {
        const LENGTH: usize = 50;
        let words = Words::generate(LENGTH);
        assert_eq!(words.len(), LENGTH);
    }
}
