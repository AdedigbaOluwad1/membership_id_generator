use super::print_dots;

pub fn generate_membership_id(country: &u32, state: &u32, vec: &Vec<String>) -> String {
    println!("Please kindly wait while we generate a unique Membership ID for you 🥳!");

    print_dots(3, "Generating".to_string());

    let mut new_membership_id = String::from(format!(
        "DL{}{}",
        format!("{:03}", &country),
        format!("{:02}", &state)
    ));

    let members_with_matching_ids: Vec<&String> = vec
        .into_iter()
        .filter(|&x| x[..new_membership_id.len()] == new_membership_id)
        .collect();

    if members_with_matching_ids.len() > 0 {
        println!(
            "Found {} member(s) with matching IDs \r\nPlease wait while we generate yours..",
            members_with_matching_ids.len()
        );
    } else {
        println!("Yohoo! You're the first of your kind, welcome 🥳🥳🥳\r\nKindly give us some time while we carefully craft your membership ID!");
    }

    print_dots(6, "Almost done".to_string());

    new_membership_id =
        new_membership_id + &format!("{:03}", &members_with_matching_ids.len() + 1).to_string();

    println!(
        "Congrats! Your newly generated Membership ID is {}",
        new_membership_id
    );

    new_membership_id
}
