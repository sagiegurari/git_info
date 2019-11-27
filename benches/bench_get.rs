#![feature(test)]
extern crate git_info;
extern crate test;

use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let info = git_info::get();

        assert!(info.current_branch.is_some());
    });
}
