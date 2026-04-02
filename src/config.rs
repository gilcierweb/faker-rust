//! Faker configuration - locale and random number generator settings

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::cell::RefCell;
use std::rc::Rc;

/// Thread-local configuration for Faker
thread_local! {
    static FAKER_CONFIG: Rc<RefCell<FakerConfig>> = Rc::new(RefCell::new(FakerConfig::default()));
}

/// Faker configuration
#[derive(Clone, Debug)]
pub struct FakerConfig {
    /// Current locale (default: "en")
    pub locale: String,
    /// Random number generator
    pub rng: Rc<RefCell<StdRng>>,
}

impl Default for FakerConfig {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
            rng: Rc::new(RefCell::new(StdRng::from_entropy())),
        }
    }
}

impl FakerConfig {
    /// Get the current Faker configuration
    pub fn current() -> Self {
        FAKER_CONFIG.with(|config| config.borrow().clone())
    }

    /// Set the locale
    pub fn set_locale(locale: &str) {
        FAKER_CONFIG.with(|config| {
            config.borrow_mut().locale = locale.to_string();
        });
    }

    /// Get the current locale
    pub fn locale() -> String {
        FAKER_CONFIG.with(|config| config.borrow().locale.clone())
    }

    /// Set the random seed for deterministic output
    pub fn set_seed(seed: u64) {
        FAKER_CONFIG.with(|config| {
            config.borrow_mut().rng = Rc::new(RefCell::new(StdRng::seed_from_u64(seed)));
        });
    }

    /// Generate a random number using the configured RNG
    pub fn rand_u32(&self, max: u32) -> u32 {
        use rand::Rng;
        self.rng.borrow_mut().gen_range(0..max)
    }

    /// Generate a random usize
    pub fn rand_usize(&self, max: usize) -> usize {
        use rand::Rng;
        self.rng.borrow_mut().gen_range(0..max)
    }

    /// Generate a random number between min and max (inclusive)
    pub fn rand_range(&self, min: u32, max: u32) -> u32 {
        if max <= min {
            return min;
        }
        // Use exclusive range to avoid off-by-one error
        let range = min..max;
        use rand::Rng;
        let mut rng = self.rng.borrow_mut();
        rng.gen_range(range)
    }

    /// Generate a random f64 between 0.0 and 1.0
    pub fn rand_f64(&self) -> f64 {
        use rand::Rng;
        let mut rng = self.rng.borrow_mut();
        let a: u32 = rng.gen_range(0..1000000);
        (a as f64) / 1000000.0
    }

    /// Generate a random bool
    pub fn rand_bool(&self) -> bool {
        use rand::Rng;
        let mut rng = self.rng.borrow_mut();
        rng.gen_range(0..2) == 1
    }

    /// Generate a random char
    pub fn rand_char(&self, chars: &[char]) -> char {
        if chars.is_empty() {
            return 'a'; // fallback
        }
        let idx = self.rand_range(0, chars.len() as u32) as usize;
        chars[idx]
    }

    /// Sample a random element from a slice
    pub fn sample<T: Clone>(&self, items: &[T]) -> T {
        if items.is_empty() {
            return items
                .first()
                .cloned()
                .unwrap_or_else(|| panic!("Cannot sample from empty slice"));
        }
        let idx = self.rand_range(0, items.len() as u32) as usize;
        items[idx].clone()
    }

    /// Shuffle a vector in place
    pub fn shuffle<T>(&self, vec: &mut Vec<T>) {
        use rand::seq::SliceRandom;
        vec.shuffle(&mut *self.rng.borrow_mut());
    }
}
