extern crate git_info;

fn main() {
    let info = git_info::get();

    println!("User Name: {}", info.user_name.unwrap());
    println!("User Email: {}", info.user_email.unwrap());
    println!("Current Branch: {}", info.current_branch.unwrap());
    println!("Config: {:#?}", info.config.unwrap());
    println!("Branches: {:#?}", info.branches.unwrap());
}
