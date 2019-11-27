extern crate git_info;

fn main() {
    let info = git_info::get();

    println!("User Name: {}", info.user_name.unwrap());
    println!("User Email: {}", info.user_email.unwrap());
    println!("Branch: {}", info.branch.unwrap());
}
