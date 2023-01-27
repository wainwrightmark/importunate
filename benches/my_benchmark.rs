// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use importunate::importunate;
// use rand::{seq::IteratorRandom, SeedableRng};

// pub fn criterion_benchmark(c: &mut Criterion) {
//     for value in [1, 2, 4, 20, 100, 1000, 10000] {
//         c.bench_function(format!("random_item({value})").as_str(), |b| {
//             let mut rng = get_rng(123);
//             b.iter(|| random_item(black_box(value), &mut rng))
//         });
//         c.bench_function(format!("choose({value})").as_str(), |b| {
//             let mut rng = get_rng(123);
//             b.iter(|| choose(black_box(value), &mut rng))
//         });

//         c.bench_function(format!("random_item_windowed({value})").as_str(), |b| {
//             let mut rng = get_rng(123);
//             b.iter(|| random_item_windowed(black_box(value), 100, &mut rng))
//         });
//         c.bench_function(format!("choose_windowed({value})").as_str(), |b| {
//             let mut rng = get_rng(123);
//             b.iter(|| choose_windowed(black_box(value), 100, &mut rng))
//         });
//     }
// }

// fn random_item(max: usize, rng: &mut rand::rngs::StdRng) -> usize {
//     let range = UnhintedIterator(0..max);
//     range.choose_item(rng).unwrap()
// }

// fn choose(max: usize, rng: &mut rand::rngs::StdRng) -> usize {
//     let range = UnhintedIterator(0..max);
//     range.choose(rng).unwrap()
// }

// fn random_item_windowed(max: usize, window: usize, rng: &mut rand::rngs::StdRng) -> usize {
//     let range = WindowHintedIterator(0..max, window);
//     range.choose_item(rng).unwrap()
// }
// fn choose_windowed(max: usize, window: usize, rng: &mut rand::rngs::StdRng) -> usize {
//     let range = WindowHintedIterator(0..max, window);
//     range.choose(rng).unwrap()
// }

// fn get_rng(seed: u64) -> rand::rngs::StdRng {
//     rand::rngs::StdRng::seed_from_u64(seed)
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);

// #[derive(Clone)]
// struct UnhintedIterator<I: Iterator + Clone>(I);
// impl<I: Iterator + Clone> Iterator for UnhintedIterator<I> {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next()
//     }
// }

// #[derive(Clone)]
// struct WindowHintedIterator<I: ExactSizeIterator + Iterator + Clone>(I, usize);

// impl<I: ExactSizeIterator + Iterator + Clone> Iterator for WindowHintedIterator<I> {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next()
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (core::cmp::min(self.0.len(), self.1), None)
//     }
// }
