use super::*;

#[test]
fn git_info_new() {
    let git_info = GitInfo::new();

    assert!(git_info.user_name.is_none());
    assert!(git_info.user_email.is_none());
    assert!(git_info.branch.is_none());
}
