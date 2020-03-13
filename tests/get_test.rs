use git_info;

fn main() {
    let info = git_info::get();

    assert!(info.config.is_some());
    assert!(info.dirty.is_some());
    assert!(info.current_branch.is_some());
    assert!(info.branches.is_some());

    let config = info.config.unwrap();

    if config.contains_key("user.name") {
        assert_eq!(
            config.get("user.name").unwrap().to_string(),
            info.user_name.unwrap()
        );
    }
    if config.contains_key("user.email") {
        assert_eq!(
            config.get("user.email").unwrap().to_string(),
            info.user_email.unwrap()
        );
    }

    let branches = info.branches.unwrap();
    assert!(branches.contains(&info.current_branch.unwrap()));
}
