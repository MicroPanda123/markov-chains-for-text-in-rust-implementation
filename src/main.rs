use rand;
use rand::prelude::SliceRandom;
use json;
use std::fs;
use std::process::Command;

fn search(model: &json::JsonValue, num: f64) -> usize{
    let mut bottom: usize = 0;
    let mut top: usize = model.len() - 1;
    let mut closest: usize;

    if Some(num) < model[0].as_f64(){
        return 0;
    }
    while bottom <= top {
        closest = ((bottom+top) / 2) as usize;
        if model[closest].as_f64() < Some(num) {
            bottom = closest + 1;
        }
        else if model[closest].as_f64() > Some(num) {
            top = closest - 1;
        }
        else {
            break;
        }
    }
    return bottom;
}

fn main() {
    let content = fs::read_to_string("model.json")
        .expect("There's been problem while reading file.");
    let model = json::parse(&content).unwrap();
    let mut wordlist = vec![""; model.entries().len()];
    let mut count: usize = 0;

    for entry in model.entries() {
        wordlist[count] = entry.0;
        count += 1;
    }

    for _ in 0..10 {
        let mut word = wordlist.choose(&mut rand::thread_rng()).unwrap().to_string();
        let mut string: String = word.to_owned();

        loop {
            let rand = rand::random::<f64>();
            let sel = search(&model[&word][1], rand);
            word = model[&word][0][sel].to_string();
            if word == "null" {
                break
            }
            string.push_str(&(" ".to_owned() + &word));
        }

        println!("{}", string);
    }
}
