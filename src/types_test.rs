use super::*;

#[test]
fn git_info_new() {
    let git_info = GitInfo::new();

    assert!(git_info.user_name.is_none());
    assert!(git_info.user_email.is_none());
    assert!(git_info.dirty.is_none());
    assert!(git_info.config.is_none());
    assert!(git_info.current_branch.is_none());
    assert!(git_info.branches.is_none());
}
