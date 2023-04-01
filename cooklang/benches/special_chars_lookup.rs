//! This is a test to check what of the following methods is fater to check if
//! a character is special

use criterion::{criterion_group, criterion_main, Criterion};

const TEST: &str = r#"
>> servings: 6

Zero step is cook @corn beef{1%kg}. Put into a large pan and simmer for 2 hours.

The first step is to cook your @potatoes{3%medium} and @carrots{3%medium}. I used a steamer, but you can always go the traditional route and boil them. In either case, peel the carrots but not the potatoes.

Steam the potatoes for 30 minutes to start with, and then add the peeled carrots. Continue steaming for 10-15 more minutes, or until the potatoes and carrots are firm but tender when poked.

Meanwhile, cook your @frozen peas{1%cup} according to package directions. I use the kind that can be steamed in the package in the microwave. When they are done, set them aside to cool.

When the potatoes and carrots are done, allow them to cool to the point that you can handle them easily.

Peel the potatoes. Using your fingers or the back of a knife, gently scrape the thin layer of skin off of the potatoes. Dice them into 1cm cube-ish shapes and put them into a medium serving bowl.

Next, dice your carrots. I've heard it said that a Soviet housewife could be judged on her housekeeping skills by how finely she could dice vegetables for her soups and salads. I, however, won't judge you. In fact, if you chop your potatoes and carrots a little larger, I would probably even thank you. I happen to like chunky salads.
[- a block comment -]
Toss the carrots and a cup of steamed peas into the bowl with the -- potatoes.

Peel and dice your hardboiled @eggs{4}. Again, I know some like to have their salads with finely diced ingredients, but I don't. So dice them however you like.

Chop @pickles{6} finely. I used small snacking dill pickles, so I needed to use six of them. If you have larger pickles, try using three and see if that is enough for you.

Add the meat if using and mix everything together gently before you add the @mayonnaise{1%cup}.

Stir in one cup of mayo to start with, and add more if you think that the salad needs more binding together.

Cover the salad and chill for at least one hour or overnight to allow the flavors to come together. And of course, garnish with finelly chopped @dill{1%tbsp}. This is a Russian salad, after all.
"#;

const SPECIAL_CHARS_STR: &str = ">:@#~?+-/*&|%{}()[]";

fn is_special_str(c: char) -> bool {
    SPECIAL_CHARS_STR.contains(c)
}

const SPECIAL_CHARS_LIST: &[char] = &[
    '>', ':', '@', '#', '~', '?', '+', '-', '/', '*', '&', '|', '%', '{', '}', '(', ')', '[', ']',
];

fn is_special_list(c: char) -> bool {
    SPECIAL_CHARS_LIST.contains(&c)
}

const SPECIAL_CHARS_LIST_ORDERED: &[char] = &[
    '{', '}', '@', '&', '%', '>', ':', '#', '~', '[', '?', '+', '-', '/', '*', '|', '(', ')', ']',
];

fn is_special_list_ordered(c: char) -> bool {
    SPECIAL_CHARS_LIST_ORDERED.contains(&c)
}

fn is_special_match(c: char) -> bool {
    match c {
        '>' | ':' | '@' | '#' | '~' | '?' | '+' | '-' | '/' | '*' | '&' | '|' | '%' | '{' | '}'
        | '(' | ')' | '[' | ']' => true,
        _ => false,
    }
}

fn is_special_match_ordered(c: char) -> bool {
    match c {
        '{' | '}' | '@' | '&' | '%' | '>' | ':' | '#' | '~' | '[' | '?' | '+' | '-' | '/' | '*'
        | '|' | '(' | ')' | ']' => true,
        _ => false,
    }
}

fn test(f: fn(char) -> bool) {
    TEST.chars().for_each(|c| {
        f(c);
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("str", |b| b.iter(|| test(is_special_str)));
    c.bench_function("list", |b| b.iter(|| test(is_special_list)));
    c.bench_function("list_ordered", |b| b.iter(|| test(is_special_list_ordered)));
    c.bench_function("match", |b| b.iter(|| test(is_special_match)));
    c.bench_function("match_ordered", |b| {
        b.iter(|| test(is_special_match_ordered))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);