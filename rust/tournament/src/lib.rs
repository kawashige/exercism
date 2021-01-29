use std::collections::BTreeMap;
pub fn tally(match_results: &str) -> String {
    let mut results = BTreeMap::new();

    for battle in match_results.split("\n") {
        let (first, second, result) = {
            let v: Vec<&str> = battle.split(";").collect();
            (v[0], v[1], v[2])
        };
        match (first, second, result) {
            (winner, loser, "win") | (loser, winner, "loss") => {
                let winner_count = results.entry(winner).or_insert([0; 3]);
                winner_count[0] += 1;
                let loser_count = results.entry(loser).or_insert([0; 3]);
                loser_count[1] += 1;
            }
            _ => {
                let first_count = results.entry(first).or_insert([0; 3]);
                first_count[2] += 1;
                let second_count = results.entry(second).or_insert([0; 3]);
                second_count[2] += 1;
            }
        }
    }

    let mut result = vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    let mut sorted: Vec<(&str, [i32; 3])> = results.into_iter().collect();
    sorted.sort_by_key(|x| -(x.1[0] * 3 + x.1[2]));
    for (k, v) in sorted {
        result.push(format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            k,
            v.iter().sum::<i32>(),
            v[0],
            v[2],
            v[1],
            v[0] * 3 + v[2]
        ));
    }
    result.join("\n")
}
