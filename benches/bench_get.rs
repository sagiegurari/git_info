#![feature(test)]
extern crate test;

use git_info;
use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let info = git_info::get();

        assert!(info.current_branch.is_some());
    });
}
