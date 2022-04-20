#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut cnt = 0;
        let mut ans = -1;
        for ch in dna.chars() {
            let p = match ch {
                'A' | 'C' | 'T' | 'G' => -1,
                _ => {
                    if ans == -1 {
                        cnt
                    } else {
                        ans
                    }
                }
            };
            if p != -1 {
                ans = p;
            }
            cnt += 1;
        }
        if ans == -1 {
            return Ok(Self {
                dna: dna.to_string(),
            });
        } else {
            return Err(ans as usize);
        }
    }

    pub fn into_rna(self) -> Rna {
        let mut result = String::with_capacity(self.dna.len());

        for ch in self.dna.chars() {
            let _new_ch = match ch {
                'A' => 'U',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => 'T',
            };
            result.push(_new_ch);
        }

        return Rna { rna: result };
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut cnt = 0;
        let mut ans = -1;
        for ch in rna.chars() {
            let p = match ch {
                'A' | 'C' | 'U' | 'G' => -1,
                _ => {
                    if ans == -1 {
                        cnt
                    } else {
                        ans
                    }
                }
            };
            if p != -1 {
                ans = p;
            }
            cnt += 1;
        }
        if ans == -1 {
            return Ok(Self {
                rna: rna.to_string(),
            });
        } else {
            return Err(ans as usize);
        }
    }
}

///
#[derive(Debug, PartialEq)]
pub struct Dna(String);
#[derive(Debug, PartialEq)]
pub struct Rna(String);
impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A' | 'C' | 'G' | 'T' => Ok(c),
                _ => Err(i),
            })
            .collect::<Result<String, usize>>()
            .map(Dna)
    }
    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|nucleotide| match nucleotide {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect::<String>())
    }
}
impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A' | 'C' | 'G' | 'U' => Ok(c),
                _ => Err(i),
            })
            .collect::<Result<String, usize>>()
            .map(Rna)
    }
}

/// ----
#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: String,
}
#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: String,
}
const RNA: [char; 4] = ['C', 'G', 'A', 'U'];
const DNA: [char; 4] = ['G', 'C', 'T', 'A'];
fn validate(s: &str, chars: [char; 4]) -> Result<String, usize> {
    match s.chars().position(|c| !chars.contains(&c)) {
        Some(x) => Err(x),
        None => Ok(s.to_string()),
    }
}
fn transcribe(nucleotide: char) -> char {
    RNA[DNA.iter().position(|&c| c == nucleotide).unwrap()]
}
impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, DNA).map(|nucleotides| Dna { nucleotides })
    }
    pub fn into_rna(self) -> Rna {
        Rna {
            nucleotides: self.nucleotides.chars().map(|c| transcribe(c)).collect(),
        }
    }
}
impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate(rna, RNA).map(|nucleotides| Rna { nucleotides })
    }
}
