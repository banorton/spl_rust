use rand::Rng;
use std::collections::HashMap;

pub struct LSystem {
    pub axiom: String,
    /// Each char maps to a list of (weight, expansion) pairs.
    pub rules: HashMap<char, Vec<(f32, String)>>,
}

impl LSystem {
    pub fn new(axiom: &str) -> Self {
        Self {
            axiom: axiom.to_string(),
            rules: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_rule(&mut self, from: char, to: &str) {
        self.add_weighted_rule(from, 1.0, to);
    }

    pub fn add_weighted_rule(&mut self, from: char, weight: f32, to: &str) {
        self.rules
            .entry(from)
            .or_default()
            .push((weight, to.to_string()));
    }

    pub fn expand(&self, iterations: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut current = self.axiom.clone();
        for _ in 0..iterations {
            current = current
                .chars()
                .map(|c| self.pick_rule(c, &mut rng))
                .collect();
        }
        current
    }

    fn pick_rule(&self, c: char, rng: &mut impl Rng) -> String {
        let Some(options) = self.rules.get(&c) else {
            return c.to_string();
        };
        if options.len() == 1 {
            return options[0].1.clone();
        }
        let total: f32 = options.iter().map(|(w, _)| w).sum();
        let mut roll = rng.gen_range(0.0..total);
        for (w, expansion) in options {
            roll -= w;
            if roll <= 0.0 {
                return expansion.clone();
            }
        }
        options.last().unwrap().1.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_iterations() {
        let sys = LSystem::new("F");
        assert_eq!(sys.expand(0), "F");
    }

    #[test]
    fn test_single_deterministic_rule() {
        let mut sys = LSystem::new("F");
        sys.add_rule('F', "F+F");
        assert_eq!(sys.expand(1), "F+F");
        assert_eq!(sys.expand(2), "F+F+F+F");
    }

    #[test]
    fn test_unmatched_chars_preserved() {
        let mut sys = LSystem::new("F+F");
        sys.add_rule('F', "FF");
        assert_eq!(sys.expand(1), "FF+FF");
    }

    #[test]
    fn test_branching_grammar() {
        let mut sys = LSystem::new("X");
        sys.add_rule('X', "F[+X][-X]");
        sys.add_rule('F', "FF");
        assert_eq!(sys.expand(1), "F[+X][-X]");
        assert_eq!(sys.expand(2), "FF[+F[+X][-X]][-F[+X][-X]]");
    }

    #[test]
    fn test_stochastic_produces_valid_output() {
        let mut sys = LSystem::new("F");
        sys.add_weighted_rule('F', 1.0, "F+F");
        sys.add_weighted_rule('F', 1.0, "F-F");
        for _ in 0..20 {
            let result = sys.expand(1);
            assert!(result == "F+F" || result == "F-F");
        }
    }

    #[test]
    fn test_stochastic_varies_output() {
        let mut sys = LSystem::new("F");
        sys.add_weighted_rule('F', 1.0, "F+F");
        sys.add_weighted_rule('F', 1.0, "F-F");
        let results: std::collections::HashSet<String> = (0..50).map(|_| sys.expand(1)).collect();
        assert!(results.len() > 1, "expected variation across 50 expansions");
    }
}
