use geometrid::tile_set::{TileSet128, TileSet64};
use iai_callgrind::{main, library_benchmark_group, library_benchmark};
use std::hint::black_box;


#[library_benchmark]
fn bench_tile_set_64_iter() -> u64 {
    let set = black_box(TileSet64::<8,8,64>::ALL);
    

    set.iter_true_tiles().map(|x|x.x() as u64 + x.y() as u64).sum()
}#[library_benchmark]
fn bench_tile_set_64_iter_back() -> u64 {
    let set = black_box(TileSet64::<8,8,64>::ALL);
    

    set.iter_true_tiles().rev().map(|x|x.x() as u64 + x.y() as u64).sum()
}

#[library_benchmark]
fn bench_tile_set_128_iter() -> u64 {
    let set = black_box(TileSet128::<8,8,64>::ALL);
    

    set.iter_true_tiles().map(|x|x.x() as u64 + x.y() as u64).sum()


}#[library_benchmark]
fn bench_tile_set_128_iter_back() -> u64 {
    let set = black_box(TileSet128::<8,8,64>::ALL);
    

    set.iter_true_tiles().rev().map(|x|x.x() as u64 + x.y() as u64).sum()
}

library_benchmark_group!(
    name = bench_tile_set;
    benchmarks = bench_tile_set_64_iter, bench_tile_set_128_iter, bench_tile_set_64_iter_back, bench_tile_set_128_iter_back, 
);

main!(library_benchmark_groups = bench_tile_set);