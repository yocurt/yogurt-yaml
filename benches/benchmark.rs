#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use libcurt::{IdentRange, Indicators, YogurtYaml};

fn criterion_benchmark_curt_multi(c: &mut Criterion) {
    let mut indicators = Vec::new();
    let indicator_brackets = Indicators::new(&["ID", "ADD"], IdentRange::Brackets);
    let indicator_closures = Indicators::new(&["REF", "END"], IdentRange::Closures);
    indicators.push(indicator_brackets);
    indicators.push(indicator_closures);
    let mut curt = YogurtYaml::new(&indicators);
    let test_data = "other stuff ID[Test, \nTestContent: \"3\"] more\n REF{Test2, \nTestContent: [4]\n} stuADD[Test3, TestContent: [[a,7],[a,d]]]".repeat(5);
    c.bench_function("YogurtYaml.curt(multi)", |b| {
        b.iter(|| curt.curt(black_box(&test_data)))
    });
}

fn criterion_benchmark_curt_multi_tags(c: &mut Criterion) {
    let mut indicators = Vec::new();
    let indicator_brackets = Indicators::new(&["ID", "ADD"], IdentRange::Brackets);
    let indicator_closures = Indicators::new(&["REF", "END"], IdentRange::Closures);
    let indicator_tags = Indicators::new(&["#", "@"], IdentRange::Tags);
    indicators.push(indicator_brackets);
    indicators.push(indicator_closures);
    indicators.push(indicator_tags);
    let mut curt = YogurtYaml::new(&indicators);
    let test_data = "other @stuff ID[Test, \nTestContent: \"3\"] more\n REF{Test2, \nTestContent: [4]\n} #stu ADD[Test3, TestContent: [[a,7],[a,d]]]".repeat(5);
    c.bench_function("YogurtYaml.curt(tags)", |b| {
        b.iter(|| curt.curt(black_box(&test_data)))
    });
}

fn criterion_benchmark_curt_5(c: &mut Criterion) {
    let mut curt = YogurtYaml::new_from_str(&["ID", "ADD", "REF", "END"]);
    let test_data = "other stuff ID[Test, \nTestContent: \"3\"] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]".repeat(5);
    c.bench_function("YogurtYaml.curt()", |b| {
        b.iter(|| curt.curt(black_box(&test_data)))
    });
}

fn criterion_benchmark_curt_1(c: &mut Criterion) {
    let mut curt = YogurtYaml::new_from_str(&["ID", "ADD", "REF", "END"]);
    let test_data = "other stuff ID[Test, \nTestContent: \"3\"] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]".repeat(1);
    c.bench_function("YogurtYaml.curt(short)", |b| {
        b.iter(|| curt.curt(black_box(&test_data)))
    });
}

criterion_group!(
    benches,
    criterion_benchmark_curt_1,
    criterion_benchmark_curt_5,
    criterion_benchmark_curt_multi
);
criterion_main!(benches);
