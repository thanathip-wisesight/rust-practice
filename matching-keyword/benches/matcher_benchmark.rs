use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::collections::HashMap;
use std::io;

#[path = "../src/matcher/mod.rs"]
mod matcher;

struct MockData {
    short_text: String,
    text: String,
    long_text: String,
    pattern: Vec<String>,
    pattern_nested: Vec<Vec<String>>,
}

impl MockData {
    pub fn new() -> MockData {
        MockData {
            // 30 words after -> But I must...
            short_text: "สวัสดีวันนี้อากาศร้อน123456789+*-)(~`~)@{.,}??<>$$##&|!/✆⍟🎉😆🇹🇭🇺🇸🧪🪐👩‍🚀❤️🔒 #me My home But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system".to_string(),
            // 300 words after -> But I must...
            text: "สวัสดีวันนี้อากาศร้อน123456789+*-)(~`~)@{.,}??<>$$##&|!/✆⍟🎉😆🇹🇭🇺🇸🧪🪐👩‍🚀❤️🔒 #me My home But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that".to_string(),
            // 1000 words after -> But I must...
            long_text: "สวัสดีวันนี้อากาศร้อน123456789+*-)(~`~)@{.,}??<>$$##&|!/✆⍟🎉😆🇹🇭🇺🇸🧪🪐👩‍🚀❤️🔒 #me My home But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains. But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection:".to_string(),
            pattern: vec![
                "test".to_string(),
                "home".to_string(),
                "word".to_string(),
                "consectetuer".to_string(),
                "adipiscing".to_string(),
                "commodo".to_string(),
                "dolor".to_string(),
                "penatibus".to_string(),
                "consequat".to_string(),
                "ipsum".to_string(),
                "mistaken".to_string(),
                "pleasure".to_string(),
                "complete".to_string(),
                "account".to_string(),
                "welcomed".to_string(),
                "สวัสดี".to_string(),
                "วันนี้".to_string(),
                "explain".to_string(),
                "mistaken".to_string(),
                "denouncing".to_string(),
                "happiness".to_string(),
                "expound".to_string(),
                "me".to_string(),
                "born".to_string(),
                "complete".to_string(),
                "occasionally".to_string(),
                "anyone".to_string(),
                "frequently".to_string(),
                "789".to_string(),
                "dislike".to_string(),
            ],
            pattern_nested: vec![
                vec![
                    "test".to_string(),
                    "home".to_string(),
                    "word".to_string()
                ],
                vec![
                    "consectetuer".to_string(),
                    "adipiscing".to_string(),
                    "commodo".to_string(),
                    "dolor".to_string()
                ],
                vec![
                    "penatibus".to_string(),
                    "consequat".to_string(),
                    "ipsum".to_string(),
                    "mistaken".to_string()
                ],
                vec![
                    "pleasure".to_string(),
                    "complete".to_string(),
                    "account".to_string(),
                    "welcomed".to_string(),
                ],
                vec![
                    "สวัสดี".to_string(),
                    "วันนี้".to_string(),
                    "explain".to_string(),
                    "mistaken".to_string(),
                    "denouncing".to_string(),
                ],
                vec![
                    "happiness".to_string(),
                    "expound".to_string(),
                    "me".to_string(),
                ],
                vec![
                    "born".to_string(),
                    "complete".to_string(),
                    "occasionally".to_string(),
                    "anyone".to_string(),
                    "frequently".to_string(),
                    "789".to_string(),
                    "dislike".to_string(),
                ]
            ]
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let pattern = MockData::new().pattern;
    let mut data_tests = HashMap::new();
    data_tests.insert("short_text", MockData::new().short_text);
    data_tests.insert("normal_text", MockData::new().text);
    data_tests.insert("long_text", MockData::new().long_text);

    let mut group = c.benchmark_group("Matcher");

    for (key, value) in data_tests.iter() {
        // group.throughput(Throughput::Bytes(value.as_bytes()));
        group.bench_function(format!("is_match aho_corasick with {}", &key), |b| {
            b.iter(|| matcher::is_match(matcher::generator_aho_match(pattern.clone()), &value))
        });

        group.bench_function(format!("is_match_contains with {}", &key), |b| {
            b.iter(|| matcher::is_match_contains(pattern.clone(), &value))
        });

        group.bench_function(format!("is_match_contains_with_rayon with {}", &key), |b| {
            b.iter(|| matcher::is_match_contains_with_rayon(pattern.clone(), &value))
        });

        group.bench_function(
            format!("is_match_with_bytes aho_corasick with {}", &key),
            |b| {
                b.iter(|| {
                    matcher::is_match_with_bytes(
                        matcher::generator_aho_match(pattern.clone()),
                        io::BufReader::with_capacity(1, value.as_bytes()),
                    )
                })
            },
        );

        group.bench_function(format!("is_match_regex with {}", &key), |b| {
            b.iter(|| {
                matcher::is_match_regex(
                    &matcher::generator_regex(&matcher::generator_regex_with_condition(
                        pattern.clone(),
                    )),
                    &value,
                )
            })
        });
    }

    group.finish();

    // c.bench_function("run_match_multiple_condition", |b| {
    //     b.iter(|| {
    //         matcher::run_match_multiple_condition(
    //             pattern_nested,
    //             &MockData::new().text,
    //         )
    //     })
    // });

    // c.bench_function("run_match_multiple_condition_with_rayon", |b| {
    //     b.iter(|| {
    //         matcher::run_match_multiple_condition_with_rayon(
    //             pattern_nested,
    //             &MockData::new().text,
    //         )
    //     })
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
