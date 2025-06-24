use std::fmt;
use std::fmt::Display;
use std::ops::MulAssign;
use std::str::FromStr;
use std::{error::Error, ops::Mul};

use nom::Parser;
use nom::character::complete::char;
use nom::multi::many0;
use nom::{IResult, error::ErrorKind};
use num::{Integer, One, pow};
use phf::phf_map;

use crate::DEFAULT_BASE;
use crate::chord::FullChord;
use crate::playable::PlayableChord;

#[derive(Debug)]
pub struct FullNote {
    harmonym: Harmonym,
    base: f32,
}

impl Into<FullChord> for FullNote {
    fn into(self) -> FullChord {
        FullChord {
            tones: vec![self.harmonym],
            base: self.base,
        }
    }
}

impl From<Harmonym> for FullNote {
    fn from(value: Harmonym) -> Self {
        Self {
            harmonym: value,
            base: crate::DEFAULT_BASE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Harmonym {
    notes: [NotePart; 5],
}

impl Into<PlayableChord> for Harmonym {
    fn into(self) -> PlayableChord {
        FullChord {
            tones: vec![self],
            base: DEFAULT_BASE,
        }
        .into()
    }
}

impl Display for Harmonym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut prefixed = false;
        let mut exclude_ah = false;
        for el in self.notes {
            if el.dimension == 1 {
                continue;
            }
            if el.degree != 0 {
                exclude_ah = true;
            }
        }
        for el in self.notes {
            if exclude_ah && el.dim() == 1 {
                continue;
            }
            if el.dim() == 0 {
                continue;
            }
            if prefixed {
                s.push_str(&el.to_suffix());
            } else {
                let root = &el.to_root();
                if root.is_empty() {
                    continue;
                }
                prefixed = true;
                s.push_str(root);
            }
        }
        write!(f, "{}", s)
    }
}

impl Default for Harmonym {
    fn default() -> Self {
        Self {
            notes: [
                NotePart {
                    dimension: 1,
                    degree: 0,
                },
                NotePart {
                    dimension: 2,
                    degree: 0,
                },
                NotePart {
                    dimension: 3,
                    degree: 0,
                },
                NotePart {
                    dimension: 4,
                    degree: 0,
                },
                NotePart {
                    dimension: 5,
                    degree: 0,
                },
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ratio {
    dividend: u32,
    divisor: u32,
}

impl From<(u32, u32)> for Ratio {
    fn from(value: (u32, u32)) -> Self {
        Self {
            dividend: value.0,
            divisor: value.1,
        }
    }
}

impl Into<Ratio> for Harmonym {
    fn into(self) -> Ratio {
        self.eval()
    }
}

fn dim2frac(dim: u8) -> Ratio {
    match dim {
        0 => (1, 1),
        1 => (2, 1),
        2 => (3, 2),
        3 => (5, 4),
        4 => (7, 4),
        5 => (11, 4),
        _ => (1, 1),
    }
    .into()
}

impl Ratio {
    fn flip(self) -> Self {
        Self {
            dividend: self.divisor,
            divisor: self.dividend,
        }
    }
}

impl Into<f32> for Ratio {
    fn into(self) -> f32 {
        self.dividend as f32 / self.divisor as f32
    }
}

impl Mul for Ratio {
    type Output = Ratio;
    fn mul(self, rhs: Self) -> Self::Output {
        let gcd = (self.dividend * rhs.dividend).gcd(&(self.divisor * rhs.divisor));
        Self {
            dividend: self.dividend * rhs.dividend / gcd,
            divisor: self.divisor * rhs.divisor / gcd,
        }
    }
}

impl MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl One for Ratio {
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.dividend == 1 && self.divisor == 1
    }
    fn set_one(&mut self) {
        *self = Self {
            divisor: 1,
            dividend: 1,
        };
    }
    fn one() -> Self {
        Self {
            divisor: 1,
            dividend: 1,
        }
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.dividend, self.divisor)
    }
}

impl Harmonym {
    pub fn eval(&self) -> Ratio {
        let mut ratio = Ratio {
            divisor: 1,
            dividend: 1,
        };
        for i in self.notes {
            if i.degree() > 0 {
                ratio *= pow(dim2frac(i.dim()), i.degree() as usize);
            } else {
                ratio *= pow(dim2frac(i.dim()).flip(), i.degree().abs() as usize)
            }
        }
        ratio
    }
}

impl Ord for Harmonym {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Into::<f32>::into(self.eval()).total_cmp(&other.eval().into())
    }
}

impl PartialOrd for Harmonym {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Mul<f32> for Harmonym {
    type Output = f32;
    fn mul(self, rhs: f32) -> Self::Output {
        let ratio = self.eval();
        rhs * ratio.dividend as f32 / ratio.divisor as f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotePart {
    dimension: u8,
    degree: i8,
}

impl NotePart {
    pub fn to_root(&self) -> String {
        match self.dimension {
            1 => "Ah",
            2 => match self.degree {
                1 => "Chy",
                2 => "Scy",
                3 => "Xcy",
                -1 => "Fu",
                -2 => "Schu",
                -3 => "Ju",
                _ => "",
            },
            3 => match self.degree {
                1 => "Ly",
                2 => "Dry",
                3 => "Drvy",
                -1 => "Su",
                -2 => "Sru",
                -3 => "Srvu",
                _ => "",
            },
            4 => match self.degree {
                1 => "My",
                2 => "Mry",
                3 => "Mrvy",
                -1 => "Pu",
                -2 => "Pru",
                -3 => "Prvu",
                _ => "",
            },
            5 => match self.degree {
                1 => "Zy",
                2 => "Zry",
                3 => "Zrvy",
                -1 => "Tschu",
                -2 => "Kru",
                -3 => "Krvu",
                _ => "",
            },
            _ => "",
        }
        .to_owned()
    }
    pub fn to_suffix(&self) -> String {
        match self.dimension {
            1 => "ah",
            2 => match self.degree {
                1 => "chi",
                2 => "sci",
                3 => "xci",
                -1 => "f",
                -2 => "sch",
                -3 => "j",
                _ => "",
            },
            3 => match self.degree {
                1 => "li",
                2 => "dri",
                3 => "drvi",
                -1 => "s",
                -2 => "sr",
                -3 => "srv",
                _ => "",
            },
            4 => match self.degree {
                1 => "mi",
                2 => "mri",
                3 => "mrvi",
                -1 => "p",
                -2 => "pr",
                -3 => "prv",
                _ => "",
            },
            5 => match self.degree {
                1 => "zi",
                2 => "zri",
                3 => "zrvi",
                -1 => "tsch",
                -2 => "kr",
                -3 => "krv",
                _ => "",
            },
            _ => "",
        }
        .to_owned()
    }

    pub fn dim(&self) -> u8 {
        self.dimension
    }

    pub fn degree(&self) -> i8 {
        self.degree
    }
    pub fn new(dimension: u8, degree: i8) -> Option<Self> {
        if degree < 4 && degree > -4 && dimension <= 5 {
            Some(NotePart { dimension, degree })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ParseDimensionDegreeError;

impl fmt::Display for ParseDimensionDegreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid dimension-degree string")
    }
}

impl Error for ParseDimensionDegreeError {}

impl FromStr for NotePart {
    type Err = ParseDimensionDegreeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // dim 1
            "Ah" | "ah" => Ok(Self {
                dimension: 1,
                degree: 0,
            }),

            // dim 2
            "Chy" | "chi" => Ok(Self {
                dimension: 2,
                degree: 1,
            }),
            "Scy" | "sci" => Ok(Self {
                dimension: 2,
                degree: 2,
            }),
            "Xcy" | "xci" => Ok(Self {
                dimension: 2,
                degree: 3,
            }),
            "Fu" | "f" => Ok(Self {
                dimension: 2,
                degree: -1,
            }),
            "Schu" | "sch" => Ok(Self {
                dimension: 2,
                degree: -2,
            }),
            "Ju" | "j" => Ok(Self {
                dimension: 2,
                degree: -3,
            }),

            // dim 3
            "Ly" | "li" => Ok(Self {
                dimension: 3,
                degree: 1,
            }),
            "Dry" | "dri" => Ok(Self {
                dimension: 3,
                degree: 2,
            }),
            "Drvy" | "drvi" => Ok(Self {
                dimension: 3,
                degree: 3,
            }),
            "Su" | "s" => Ok(Self {
                dimension: 3,
                degree: -1,
            }),
            "Sru" | "sr" => Ok(Self {
                dimension: 3,
                degree: -2,
            }),
            "Srvu" | "srv" => Ok(Self {
                dimension: 3,
                degree: -3,
            }),

            // dim 4
            "My" | "mi" => Ok(Self {
                dimension: 4,
                degree: 1,
            }),
            "Mry" | "mri" => Ok(Self {
                dimension: 4,
                degree: 2,
            }),
            "Mrvy" | "mrvi" => Ok(Self {
                dimension: 4,
                degree: 3,
            }),
            "Pu" | "p" => Ok(Self {
                dimension: 4,
                degree: -1,
            }),
            "Pru" | "pr" => Ok(Self {
                dimension: 4,
                degree: -2,
            }),
            "Prvu" | "prv" => Ok(Self {
                dimension: 4,
                degree: -3,
            }),

            // dim 5
            "Zy" | "zi" => Ok(Self {
                dimension: 5,
                degree: 1,
            }),
            "Zry" | "zri" => Ok(Self {
                dimension: 5,
                degree: 2,
            }),
            "Zrvy" | "zrvi" => Ok(Self {
                dimension: 5,
                degree: 3,
            }),
            "Tschu" | "tsch" => Ok(Self {
                dimension: 5,
                degree: -1,
            }),
            "Kru" | "kr" => Ok(Self {
                dimension: 5,
                degree: -2,
            }),
            "Krvu" | "krv" => Ok(Self {
                dimension: 5,
                degree: -3,
            }),

            _ => Err(ParseDimensionDegreeError),
        }
    }
}

// Root maps
static DIM1_ROOTS: phf::Map<&'static str, i8> = phf_map! {
    "Ah" => 0,
};

static DIM2_ROOTS: phf::Map<&'static str, i8> = phf_map! {
    "Chy" => 1, "Scy" => 2, "Xcy" => 3,
    "Fu" => -1, "Schu" => -2, "Ju" => -3,
};

static DIM3_ROOTS: phf::Map<&'static str, i8> = phf_map! {
    "Ly" => 1, "Dry" => 2, "Drvy" => 3,
    "Su" => -1, "Sru" => -2, "Srvu" => -3,
};

static DIM4_ROOTS: phf::Map<&'static str, i8> = phf_map! {
    "My" => 1, "Mry" => 2, "Mrvy" => 3,
    "Pu" => -1, "Pru" => -2, "Prvu" => -3,
};

static DIM5_ROOTS: phf::Map<&'static str, i8> = phf_map! {
    "Zy" => 1, "Zry" => 2, "Zrvy" => 3,
    "Tschu" => -1, "Kru" => -2, "Krvu" => -3,
};

// Suffix maps (2–5 only)
static DIM2_SUFFIXES: phf::Map<&'static str, i8> = phf_map! {
    "chi" => 1, "sci" => 2, "xci" => 3,
    "f" => -1, "sch" => -2, "j" => -3,
};

static DIM3_SUFFIXES: phf::Map<&'static str, i8> = phf_map! {
    "li" => 1, "dri" => 2, "drvi" => 3,
    "s" => -1, "sr" => -2, "srv" => -3,
};

static DIM4_SUFFIXES: phf::Map<&'static str, i8> = phf_map! {
    "mi" => 1, "mri" => 2, "mrvi" => 3,
    "p" => -1, "pr" => -2, "prv" => -3,
};

static DIM5_SUFFIXES: phf::Map<&'static str, i8> = phf_map! {
    "zi" => 1, "zri" => 2, "zrvi" => 3,
    "tsch" => -1, "kr" => -2, "krv" => -3,
};

fn try_match<'a>(
    input: &'a str,
    map: &phf::Map<&'static str, i8>,
    dimension: u8,
) -> Option<(&'a str, NotePart)> {
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len())); // longest match first

    for key in keys {
        if let Some(rest) = input.strip_prefix(key) {
            return Some((
                rest,
                NotePart {
                    dimension,
                    degree: map[key],
                },
            ));
        }
    }

    None
}

pub fn parse_harmonym(input: &str) -> IResult<&str, Harmonym> {
    let mut notes: [NotePart; 5] = [
        NotePart {
            dimension: 1,
            degree: 0,
        },
        NotePart {
            dimension: 2,
            degree: 0,
        },
        NotePart {
            dimension: 3,
            degree: 0,
        },
        NotePart {
            dimension: 4,
            degree: 0,
        },
        NotePart {
            dimension: 5,
            degree: 0,
        },
    ];
    let mut rest = input;

    let root_options = [
        (1, &DIM1_ROOTS),
        (2, &DIM2_ROOTS),
        (3, &DIM3_ROOTS),
        (4, &DIM4_ROOTS),
        (5, &DIM5_ROOTS),
    ];

    let (rest_after_root, root_dim) = {
        let mut found = None;
        for (dim, map) in root_options {
            if let Some((r, note)) = try_match(rest, map, dim) {
                notes[(dim - 1) as usize] = note;
                found = Some((r, dim));
                break;
            }
        }
        match found {
            Some(x) => x,
            None => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    ErrorKind::Tag,
                )));
            }
        }
    };
    rest = rest_after_root;

    let suffix_maps: [(u8, &phf::Map<&'static str, i8>); 4] = [
        (2, &DIM2_SUFFIXES),
        (3, &DIM3_SUFFIXES),
        (4, &DIM4_SUFFIXES),
        (5, &DIM5_SUFFIXES),
    ];

    for (dim, map) in suffix_maps {
        if dim <= root_dim {
            continue;
        }
        if let Some((next, part)) = try_match(rest, map, dim) {
            notes[(dim - 1) as usize] = part;
            rest = next;
        }
    }

    let (next, parts) = many0(char('+')).parse(rest)?;
    notes[0] = NotePart {
        degree: parts.len() as i8,
        dimension: 1,
    };
    rest = next;
    let (next, parts) = many0(char('-')).parse(rest)?;
    notes[0] = NotePart {
        degree: -1 * parts.len() as i8,
        dimension: 1,
    };
    rest = next;
    if rest.is_empty() {
        Ok(("", Harmonym { notes }))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            rest,
            ErrorKind::Eof,
        )))
    }
}
