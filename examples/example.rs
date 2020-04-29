use git_info;

fn main() {
    let info = git_info::get();

    println!(
        "User Name: {}",
        info.user_name.unwrap_or("Unknown".to_string())
    );
    println!(
        "User Email: {}",
        info.user_email.unwrap_or("Unknown".to_string())
    );
    println!("Dirty: {}", info.dirty.unwrap_or(false));
    println!(
        "Current Branch: {}",
        info.current_branch.unwrap_or("Unknown".to_string())
    );

    println!(
        "Last Commit Hash: {}",
        info.head.last_commit_hash.unwrap_or("Unknown".to_string())
    );
    println!(
        "Last Commit Hash (short): {}",
        info.head
            .last_commit_hash_short
            .unwrap_or("Unknown".to_string())
    );

    println!("Config: {:#?}", info.config.unwrap());
    println!("Branches: {:#?}", info.branches.unwrap_or(vec![]));
}
