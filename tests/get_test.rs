extern crate git_info;

fn main() {
    let info = git_info::get();

    assert!(info.user_name.is_some());
    assert!(info.user_email.is_some());
    assert!(info.branch.is_some());
}
