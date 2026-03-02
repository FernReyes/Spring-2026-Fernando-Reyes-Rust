fn most_frequent_word(text: &str) -> (String, usize) {
    
    let words: Vec<&str> = text.split_whitespace().collect();

    //let i=0;
    let mut max_count = 0;
    let mut max_word = "...";

    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    let mut count4 = 0;
    let mut count5 = 0;
    let mut count6 = 0;
    let mut count7 = 0;
    let mut count8 = 0;
    
    for i in words.iter(){
        if *i == "the"{
            count1 += 1;
        }
        else if *i == "quick"{
            count2 += 1;
        }
        else if *i == "brown"{
            count3 += 1;
        }
        else if *i == "fox"{
            count4 += 1;
        }
        else if *i == "jumps"{
            count5 += 1;
        }
        else if *i == "over"{
            count6 += 1;
        }
        else if *i == "lazy"{
            count7 += 1;
        }
        else if *i == "dog"{
            count8 += 1;
        }
    }

    if count1 > max_count{
        max_count = count1;
        max_word = "the";
    }
    if count2 > max_count{
        max_count = count2;
        max_word = "quick";
    }
    if count3 > max_count{
        max_count = count3;
        max_word = "brown";
    }
    if count4 > max_count{
        max_count = count4;
        max_word = "fox";
    }
    if count5 > max_count{
        max_count = count5;
        max_word = "jumps";
    }
    if count6 > max_count{
        max_count = count6;
        max_word = "over";
    }
    if count7 > max_count{
        max_count = count7;
        max_word = "lazy";
    }
    if count8 > max_count{
        max_count = count8;
        max_word = "dog";
    }

    return (max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}