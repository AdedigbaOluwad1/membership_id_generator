use std;

mod membership;
use membership::generate_membership_id;

mod utils;
use utils::{print_dots, prompt_user};

fn main() {
    // Membership ID Generator
    let mut vector = vec![
        "DL23402001".to_string(),
        "DL23340001".to_string(),
        "DL23340002".to_string(),
        "DL23340003".to_string(),
    ];

    prompt_user(&mut vector);
}
