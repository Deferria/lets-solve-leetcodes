#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    A,
    C,
    G,
    U
}

impl Base {
    fn detail(&self) -> String {
        match self {
            Base::A => "Adenine".into(),
            Base::C => "Cytosine".into(),
            Base::G => "Guanine".into(),
            Base::U => "Uracil".into()
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Base::A),
            'C' => Some(Base::C),
            'G' => Some(Base::G),
            'U' => Some(Base::U),
            _ => None
        }
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Base::A => "Adenine",
            Base::C => "Cytosine",
            Base::G => "Guanine",
            Base::U => "Uracil"
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Codon {
    first: Base,
    second: Base,
    third: Base
}

impl Codon {
    fn from_base(b1: Base, b2: Base, b3: Base) -> Self {
        Codon {
            first: b1,
            second: b2,
            third: b3
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        if s.len() != 3 {
            panic!("Codon string must be exactly 3 characters long");
        }
        let mut bases = [Base::A; 3];
        for (i, c) in s.chars().enumerate() {
            if let Some(base) = Base::from_char(c) {
                bases[i] = base;
            } else {
                panic!("Invalid base character: {}. It must be one of A, C, G, or U", c);
            }
        }
        Some(Codon { first: bases[0], second: bases[1], third: bases[2] })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AminoFromCodon {
    Phe,
    Leu,
    Ser,
    Tyr,
    Cys,
    Trp,
    Pro,
    His,
    Gln,
    Arg,
    Ile,
    Met,
    Thr,
    Asn,
    Lys,
    Val,
    Ala,
    Asp,
    Glu,
    Gly,
    Terminal
}

impl AminoFromCodon {
    fn from_codon(codon: &Codon) -> Self {
        let animo = match codon.first {
            Base::U => match codon.second {
                Base::U => match codon.third {
                    Base::U | Base::C => AminoFromCodon::Phe,
                    Base::A | Base::G => AminoFromCodon::Leu
                },
                Base::C => AminoFromCodon::Ser,
                Base::A => match codon.third {
                    Base::U | Base::C => AminoFromCodon::Tyr,
                    Base::A | Base::G => AminoFromCodon::Terminal
                },
                Base::G => match codon.third {
                    Base::U | Base::C => AminoFromCodon::Cys,
                    Base::A => AminoFromCodon::Terminal,
                    Base::G => AminoFromCodon::Trp
                }
            },
            Base::C => match codon.second {
                Base::U => AminoFromCodon::Leu,
                Base::C => AminoFromCodon::Pro,
                Base::A => match codon.third {
                    Base::U | Base::C => AminoFromCodon::His,
                    Base::A | Base::G => AminoFromCodon::Gln
                },
                Base::G => AminoFromCodon::Arg
            },
            Base::A => match codon.second {
                Base::U => match codon.third {
                    Base::U | Base::C | Base::A => AminoFromCodon::Ile,
                    Base::G => AminoFromCodon::Met
                },
                Base::C => AminoFromCodon::Thr,
                Base::A => match codon.third {
                    Base::U | Base::C => AminoFromCodon::Asn,
                    Base::A | Base::G => AminoFromCodon::Lys
                },
                Base::G => AminoFromCodon::Ser
            },
            Base::G => match codon.second {
                Base::U => AminoFromCodon::Val,
                Base::C => AminoFromCodon::Ala,
                Base::A => match codon.third {
                    Base::U | Base::C => AminoFromCodon::Asp,
                    Base::A | Base::G => AminoFromCodon::Glu
                },
                Base::G => AminoFromCodon::Gly
            }
        };
        animo
    }
}

impl std::fmt::Display for AminoFromCodon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AminoFromCodon::Phe => "Phe",
            AminoFromCodon::Leu => "Leu",
            AminoFromCodon::Ser => "Ser",
            AminoFromCodon::Tyr => "Tyr",
            AminoFromCodon::Cys => "Cys",
            AminoFromCodon::Trp => "Trp",
            AminoFromCodon::Pro => "Pro",
            AminoFromCodon::His => "His",
            AminoFromCodon::Gln => "Gln",
            AminoFromCodon::Arg => "Arg",
            AminoFromCodon::Ile => "Ile",
            AminoFromCodon::Met => "Met",
            AminoFromCodon::Thr => "Thr",
            AminoFromCodon::Asn => "Asn",
            AminoFromCodon::Lys => "Lys",
            AminoFromCodon::Val => "Val",
            AminoFromCodon::Ala => "Ala",
            AminoFromCodon::Asp => "Asp",
            AminoFromCodon::Glu => "Glu",
            AminoFromCodon::Gly => "Gly",
            AminoFromCodon::Terminal => "Terminal"
        };
        write!(f, "{}", s)
    }
}

fn string_to_codons(s: &str) -> Vec<Codon> {
    let mut codons = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    for i in (0..chars.len()).step_by(3) {
        if i + 2 < chars.len() {
            let codon_str: String = chars[i..i + 3].iter().collect();
            if let Some(codon) = Codon::from_str(&codon_str) {
                codons.push(codon);
            }
        }
    }
    codons
}

fn codons_to_amino(codons: Vec<Codon>) -> Vec<AminoFromCodon> {
    let mut aminos = Vec::new();
    let mut beginning = false;
    for c in codons.iter() {
        if *c == Codon::from_base(Base::A, Base::U, Base::G) {
            aminos.push(AminoFromCodon::Met);
            beginning = true;
        } else if beginning {
            let amino = AminoFromCodon::from_codon(c);
            if amino == AminoFromCodon::Terminal {
                break;
            }
            aminos.push(amino);
        }
    }
    aminos
}

pub fn translate_rna_to_protein(rna: &str) -> Vec<AminoFromCodon> {
    let codons = string_to_codons(rna);
    codons_to_amino(codons)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_translate_rna_to_protein() {
        let rna = "AACAUGUUUUCGUACGGUGGCAACACGUAA";
        let protein = translate_rna_to_protein(rna);
        println!("Protein: {:?}", protein);
        //assert_eq!(protein, vec![AminoFromCodon::Met, AminoFromCodon::Phe]);
    }
}