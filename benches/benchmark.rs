#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use libcurt::YogurtYaml;

fn criterion_benchmark_curt_5(c: &mut Criterion) {
    let mut curt = YogurtYaml::new(&["ID", "ADD", "REF", "END"]);
    let test_data = "other stuff ID[Test, \nTestContent: \"3\"] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]".repeat(5);
    c.bench_function("YogurtYaml.curt()", |b| {
        b.iter(|| curt.curt(black_box(&test_data)))
    });
}

fn criterion_benchmark_curt_1(c: &mut Criterion) {
    let mut curt = YogurtYaml::new(&["ID", "ADD", "REF", "END"]);
    let test_data = "other stuff ID[Test, \nTestContent: \"3\"] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]".repeat(1);
    c.bench_function("YogurtYaml.curt(short)", |b| {
        b.iter(|| curt.curt(black_box(&test_data)))
    });
}

criterion_group!(benches, criterion_benchmark_curt_1, criterion_benchmark_curt_5);
criterion_main!(benches);
