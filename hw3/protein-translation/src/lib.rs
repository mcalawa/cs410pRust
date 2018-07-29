use std::collections::HashMap;

pub struct Proteins<'a> {
    codon_name_pair: HashMap<&'a str, &'a str>,
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Proteins {
    Proteins {
        codon_name_pair: pairs.into_iter().collect(),
    }
}

impl<'a> Proteins<'a> {
    pub fn name_for(&self, codon: &str) -> Result<&str, &str> {
        if self.codon_name_pair.contains_key(codon) {
            Ok(self.codon_name_pair.get(codon).unwrap())
        }
        else {
            Err("Error!")
        }
    }

    pub fn of_rna(&self, rna: &'a str) -> Result<Vec<&str>, &str> {
        let mut codons = Vec::new();
        let mut names = Vec::new();
        let mut length = rna.clone().chars().count();

        if length % 3 != 0 {
            return Err("Error!")
        }
        else {
            let mut sequence = rna.clone();
            for _i in 0..length / 3 {
                let (codon, second) = sequence.split_at(3);
                codons.push(codon);
                sequence = second;
            }

            length = codons.len();
            for i in 0..length {
                if self.codon_name_pair.contains_key(codons[i]) {
                    if codons[i] == "UAA" || codons[i] == "UAG" || codons[i] == "UGA" {
                        return Ok(names)
                    }
                    else {
                        names.push(self.codon_name_pair.get(codons[i]).unwrap());
                    }
                }
                else {
                    return Err("Error!")
                }
            }
            Ok(names)
        }
    }
}
