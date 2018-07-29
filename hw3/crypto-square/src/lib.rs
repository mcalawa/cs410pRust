pub fn encrypt(input: &str) -> String {
    let mut filtered = "".to_string();
    let mut char_iter = input.chars();
    let mut length = char_iter.clone().count();
    for _i in 0..length {
        let character = char_iter.next().unwrap();
        if character.is_alphanumeric() {
            filtered.push(character);
        }
    }
    filtered = filtered.to_lowercase();
    char_iter = filtered.chars();
    length = char_iter.clone().count();
    let mut rows = 0;
    let mut columns = 0;
    let mut difference;
    let half;
    if length < 6 {
        half = 3;
    }
    else {
        half = length / 2;
    }
    for i in 1..half {
        let k = i + 2;
        for j in i..k {
            let product = i * j;
            if product >= length {
                difference = product - length;
                if difference == 0 {
                    rows = i;
                    columns = j;
                    break;
                }
                else if difference < j {
                    rows = i;
                    columns = j;
                }
            }
        }
    }

    let mut vec = Vec::new();
    for _c in 0..columns {
        vec.push("".to_string());
    }
    let mut index = 0;
    for _r in 0..rows {
        for c in 0..columns {
            if index < length {
                vec[c].push(char_iter.next().unwrap());
            }
            else {
                vec[c].push(' ');
            }
            index += 1;
        }
    }

    let mut output = "".to_string();
    for c in 0..columns {
        output.push_str(&vec[c]);

        if c < columns - 1 {
            output.push(' ');
        }
    }

    output
}
