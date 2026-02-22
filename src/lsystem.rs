use std::collections::HashMap;

pub struct LSystem {
    pub axiom: String,
    pub rules: HashMap<char, String>,
}

impl LSystem {
    pub fn new(axiom: &str) -> Self {
        Self {
            axiom: axiom.to_string(),
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, from: char, to: &str) {
        self.rules.insert(from, to.to_string());
    }

    pub fn expand(&self, iterations: usize) -> String {
        let mut current = self.axiom.clone();
        for _ in 0..iterations {
            current = current
                .chars()
                .map(|c| self.rules.get(&c).cloned().unwrap_or_else(|| c.to_string()))
                .collect();
        }
        current
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
    fn test_single_iteration() {
        let mut sys = LSystem::new("F");
        sys.add_rule('F', "F+F");
        assert_eq!(sys.expand(1), "F+F");
    }

    #[test]
    fn test_two_iterations() {
        let mut sys = LSystem::new("F");
        sys.add_rule('F', "F+F");
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
        // Iteration 1: X → F[+X][-X], F unchanged (no F in axiom)
        assert_eq!(sys.expand(1), "F[+X][-X]");
        // Iteration 2: F → FF, X → F[+X][-X]
        assert_eq!(sys.expand(2), "FF[+F[+X][-X]][-F[+X][-X]]");
    }
}
