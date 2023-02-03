use std::any::type_name;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use importunate::{inner::Inner, *};

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_new_index::<u128, 34>(c, 0);
    bench_new_index::<u128, 34>(c, 33);

    bench_old_index::<u128, 34>(c, 0);
    bench_old_index::<u128, 34>(c, 33);

    bench_combine::<u8, 5>(c);
    bench_combine::<u128, 5>(c);
    bench_combine::<u16, 8>(c);
    bench_combine::<u32, 12>(c);
    bench_combine::<u64, 15>(c);
    bench_combine::<u64, 20>(c);
    bench_combine::<u128, 34>(c);

    bench_calculate::<u8, 5>(c);
    bench_calculate::<u128, 5>(c);
    bench_calculate::<u16, 8>(c);
    bench_calculate::<u32, 12>(c);
    bench_calculate::<u64, 15>(c);
    bench_calculate::<u64, 20>(c);
    bench_calculate::<u128, 34>(c);

    bench_apply::<u8, 5>(c);
    bench_apply::<u128, 5>(c);
    bench_apply::<u16, 8>(c);
    bench_apply::<u32, 12>(c);
    bench_apply::<u64, 15>(c);
    bench_apply::<u64, 20>(c);
    bench_apply::<u128, 34>(c);

}

fn bench_old_index<I: Inner, const SIZE: usize>(c: &mut Criterion, index: usize) {
    c.bench_function(format!("old index {SIZE} {index}").as_str(), |b| {
        let perm = Permutation::<I, SIZE>::get_max();

        b.iter(|| old_index(black_box(perm), index))
    });
}

fn bench_new_index<I: Inner, const SIZE: usize>(c: &mut Criterion, index: usize) {
    c.bench_function(format!("new index {SIZE} {index}").as_str(), |b| {
        let perm = Permutation::<I, SIZE>::get_max();

        b.iter(|| new_index(black_box(perm), index))
    });
}

fn bench_apply<I: Inner, const SIZE: usize>(c: &mut Criterion) {
    c.bench_function(format!("apply {} {SIZE}", type_name::<I>()).as_str(), |b| {
        let arr = Permutation::<I, SIZE>::get_max().get_array();
        let perm = Permutation::<I, SIZE>::try_calculate(arr, |&x| x).unwrap();
        let test_arr = arr;
        b.iter(|| apply(black_box(test_arr), perm))
    });
}
fn bench_calculate<I: Inner, const SIZE: usize>(c: &mut Criterion) {
    c.bench_function(
        format!("calculate {} {SIZE}", type_name::<I>()).as_str(),
        |b| {
            let mut arr: [usize; SIZE] = Permutation::<I, SIZE>::default().get_array();
            arr.reverse();
            let test_arr = arr;
            b.iter(|| calculate::<I, SIZE>(black_box(test_arr)))
        },
    );
}

fn bench_combine<I: Inner, const SIZE: usize>(c: &mut Criterion) {
    c.bench_function(
        format!("combine {} {SIZE}", type_name::<I>()).as_str(),
        |b| {
            let lhs = Permutation::<I, SIZE>::get_max();
            let rhs = Permutation::<I, SIZE>::get_max();
            b.iter(|| combine::<I, SIZE>(black_box(lhs), black_box(&rhs)))
        },
    );
}

fn calculate<I: Inner, const SIZE: usize>(arr: [usize; SIZE]) -> Permutation<u64, SIZE> {
    Permutation::calculate_unchecked(arr, |&x| x)
}

fn apply<I: Inner, const SIZE: usize>(
    mut arr: [usize; SIZE],
    permutation: Permutation<I, SIZE>,
) -> [usize; SIZE] {
    permutation.apply(&mut arr);
    arr
}

fn new_index<I: Inner, const SIZE: usize>(
    permutation: Permutation<I, SIZE>,
    index: usize,
) -> usize {
    permutation.index_of(&index, |&x| x)
}

fn old_index<I: Inner, const SIZE: usize>(
    permutation: Permutation<I, SIZE>,
    index: usize,
) -> usize {
    permutation.element_at_index(index, |x| x)
}

fn combine<I: Inner, const SIZE: usize>(
    lhs: Permutation<I, SIZE>,
    rhs: &Permutation<I, SIZE>,
) -> Permutation<I, SIZE> {
    lhs.combine(rhs)
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
