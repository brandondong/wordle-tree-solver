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
    head: ClueNode,
}

struct GuessNode {
    possible_clues: Vec<ClueNode>,
}

struct ClueNode {
    optimal_guess: Option<GuessNode>,
}

fn generate_solve_tree(guess_set: &HashSet<String>, solution_set: &HashSet<String>) -> SolveTree {
    SolveTree {
        head: ClueNode {
            optimal_guess: None,
        },
    }
}
