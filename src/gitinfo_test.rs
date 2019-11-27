use super::*;

#[test]
fn get_existing() {
    let git_info = get();

    assert!(git_info.branch.is_some());
}
