use std::collections::HashSet;

fn main() {
    // TODO get these as config values.
    let mut guess_set = HashSet::new();
    guess_set.insert(String::from("aaaaa"));
    guess_set.insert(String::from("bbbbb"));
    guess_set.insert(String::from("ccccc"));
    let mut solution_set = HashSet::new();
    solution_set.insert(String::from("aaaaa"));
    solution_set.insert(String::from("bbbbb"));
    let solve_tree = generate_solve_tree(&guess_set, &solution_set);
    // TODO display solve stats.
    // TODO let user type in values for the solution to handle.
}

struct SolveTree {
    head: GuessNode,
}

struct GuessNode {
    guess: String, // TODO lifetimes.
    expected_rounds_left: f64,
    possible_clues: Vec<ClueEdge>,
}

struct ClueEdge {
    clue: Vec<Clue>,
    optimal_guess: GuessNode,
}

enum Clue {
    Grey,
    Yellow,
    Green,
}

fn generate_solve_tree(guess_set: &HashSet<String>, solution_set: &HashSet<String>) -> SolveTree {
    SolveTree {
        head: optimal_guess_tree(guess_set),
    }
}

fn optimal_guess_tree(guess_set: &HashSet<String>) -> GuessNode {
    // Try all guesses and minimize on expected number of rounds left.
    guess_set
        .iter()
        .map(|guess| {
            // Calculate expected value over all possible clues received.
            // TODO call optimal_guess_tree recurisvely for each possible clue and wrap in a clue edge struct.
            optimal_guess_tree(guess_set)
        })
        .min_by(|n1, n2| {
            n1.expected_rounds_left
                .partial_cmp(&n2.expected_rounds_left)
                .unwrap()
        })
        .expect("Set of guesses is empty")
}
