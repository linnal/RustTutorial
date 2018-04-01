fn main() {
    let vowel_word = "apple";
    let consonant_word = "first";

    println!("{} is converted into {}", vowel_word, convert_to_piglatin(&vowel_word));
    println!("{} is converted into {}", consonant_word, convert_to_piglatin(&consonant_word));
}


fn convert_to_piglatin(word: &str) -> String {
    let first_letter = &word.chars().next().unwrap();
    if is_constant(*first_letter) {
        return format!("{}-{}ay", &word[1..], first_letter);
    }
    format!("{}-hay", word)
}

fn is_vowel(ch: char) -> bool {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'y' | 'u' => true,
        _ => false
    }
}

fn is_constant(ch: char) -> bool {
    !is_vowel(ch)
}
