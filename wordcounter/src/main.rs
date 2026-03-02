fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect(); // separate the string into a vector

    let mut unique_words: Vec<&str> = Vec::new(); // new vector to hold each unique word once
    let mut counts: Vec<usize> = Vec::new(); // new vector to hold the count of each unique word
    // note that unique_words and counts are parallel

    let mut max_index = 0; // initialize index of the unique_word that has the maximum count

    for w in words {
        let mut found = false; // flag to indicate whether we have found a NOT unique word

        // check if the word is already a unique_word
        for i in 0..unique_words.len() {
            if unique_words[i] == w { // the current word is not unique, increase the count
                counts[i] += 1;
                found = true; // flag that a NOT unique word was found

                if counts[i] > counts[max_index] { // if the current word appears more times, it is the new max
                    max_index = i; // increase the max_index
                }

                break;
            }
        }

        if !found { // if the word IS unique
            unique_words.push(w); // add the unique word to the unique_words vector
            counts.push(1); // count the word once

            // if the counts array has length 1, the one is the maximum
            if counts.len() == 1 {
                max_index = 0;
            }
        }
    }
    
    let max_word = unique_words[max_index].to_string(); // word that appears most times
    let max_count = counts[max_index]; // times max word appears
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

// expected output
// Most frequent word: "the" (3 times)
