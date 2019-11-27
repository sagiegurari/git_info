#![feature(test)]
extern crate git_info;
extern crate test;

use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let info = git_info::get();

        assert!(info.user_name.is_some());
        assert!(info.user_email.is_some());
        assert!(info.branch.is_some());
    });
}
