use clap::{self, Parser};
use const_format::formatcp;

#[derive(Debug, Clone, Copy, Parser, Default)]
#[clap(multicall(true), disable_help_subcommand(true), max_term_width(80))]
#[cfg_attr(feature = "force_color", clap(color(clap::ColorChoice::Always)))]
pub struct Options {
    #[clap(subcommand)]
    pub mode: Mode,
}

impl Options {
    pub fn new() -> Self {
        let p = Self::try_parse();
        let r = match p {
            Ok(o) => o,
            Err(clap::Error {
                kind: clap::ErrorKind::DisplayHelp,
                ..
            }) => Self::parse(),
            Err(clap::Error { .. }) => Default::default(),
        };
        r
    }
}

const HNS: &str = "hns";
const HNS_NN: &str = formatcp!("{}+", HNS);
const HXS: &str = "hxs";
const ABOUT: &str = "``Human numerically'' sorts.";
const LONG_ABOUT: &str = "\nA \"human numeric\" sorting program — does what `sort -h` is supposed to do!

(That is to say, it does what you likely already thought or may've assumed GNU/BSD `sort -h` does.)";
macro_rules! usage {
    ($arg:expr) => {
        formatcp!(
            "{0} < input > output
    {0} < <(…) > output
    … | {0} > output",
            $arg
        )
    };
}
macro_rules! about_nn {
    ($arg:expr) => {
        formatcp!("{} (no negatives)", $arg)
    };
}
macro_rules! about_x {
    ($arg:expr) => {
        formatcp!("{} (hexadecimal)", $arg)
    };
}
const USAGE_HNS: &str = usage!(HNS);
const USAGE_HNSNN: &str = usage!(HNS_NN);
const USAGE_HXS: &str = usage!(HXS);

#[derive(Copy, Clone, Debug, Parser, PartialEq, Eq, derive_more::IsVariant)]
#[clap(author, version)]
#[clap(
    about = ABOUT,
    after_long_help = "\n",
    long_about = LONG_ABOUT
)]
#[clap(subcommand_required(true))]
#[repr(C)]
pub enum Mode {
    #[clap(author, version, about = ABOUT, after_long_help = "\n", long_about = LONG_ABOUT)]
    #[clap(name = HNS, bin_name = HNS)]
    #[clap(override_usage(USAGE_HNS))]
    Default(hns::Args),
    #[clap(author, version, about = about_nn!(ABOUT), after_long_help = "\n", long_about = about_nn!(LONG_ABOUT))]
    #[clap(name = HNS_NN, bin_name = HNS_NN)]
    #[clap(override_usage(USAGE_HNSNN))]
    NoNegatives(hnsnn::Args),
    #[clap(author, version, about = about_x!(ABOUT), after_long_help = "\n", long_about = LONG_ABOUT)]
    #[clap(name = HXS, bin_name = HXS)]
    #[clap(override_usage(USAGE_HXS))]
    Hexadecimal(hxs::Args),
}

impl Mode {
    #[inline]
    pub const fn consider_hex(&self) -> bool {
        match self {
            Mode::Hexadecimal(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub const fn sort_negatives(&self) -> bool {
        match self {
            Mode::Default(_) | Mode::Hexadecimal(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub const fn insensitive(&self) -> bool {
        match self {
            Mode::Default(a) => a.insensitive,
            Mode::Hexadecimal(a) => a.insensitive,
            Mode::NoNegatives(a) => a.insensitive
        }
    }
}

impl Default for Mode {
    #[inline]
    fn default() -> Self {
        Self::Default(Default::default())
    }
}

// i am so good at rust holy shit 🤯
const EXAMPLES: &str = "EXAMPLES";

macro_rules! examples {
    ($arg:ident, $const:ident) =>
    {
        mod $arg {
            use super::*;
            #[derive(Debug, Copy, Clone, Parser, PartialEq, Eq, Default)]
            #[clap(subcommand_help_heading(EXAMPLES))]
            #[clap(setting(clap::AppSettings::DeriveDisplayOrder))]
            #[repr(C)]
            pub struct Args {
                #[clap(short = 'C', long = "insensitive", help = "Case insensitive?")]
                pub insensitive: bool,

                /// This is a documentation-only “subcommand”.
                #[clap(subcommand)]
                examples: Option<Examples>,
            }

            #[derive(Copy, Clone, Debug, Parser, PartialEq, Eq, Default)]
            #[clap(next_line_help(true))]
            #[repr(C)]
            enum Examples {
                #[default]
                #[clap(name = formatcp!("find . | {}", $const))]
                #[clap(about = "Numerically sort the names of the files in the current directory.\n")]
                FindExample,
                #[clap(name = formatcp!("{} < <(dig peeweeharms.hk)", $const))]
                #[clap(about = "Numerically sort the IP addresses in the output of `dig`.\n")]
                DigExample,
                #[clap(name = formatcp!("# ①
    seq 0 1000
        |
    xargs -I{{}} bash -c \"echo {{{{}}}} > {{}}\"
    \n    # ②
    seq 0 1000
        |
    awk '{{printf \"mv %s topsecret_%s.json\\n\", $0, $0}}'
        |
    \n    # ③
    parallel
    \n    # ④
    paste <(seq 0 1000) <({0} < <(echo topsecret_[[:digit:]]*.json))
        |
    awk '{{mv %s topsecret_%04d.json\\n\", $2, $1}}' | parallel", $const))]
                #[clap(about = formatcp!("Use as part of a pipeline to fix accidentally unzeropadded numbers :-)
↓
① write some top secret JSON-format files as 0..=1000
② move them to .json file extensions
③ oops! forgot to zeropad!! managing these files will suck now :-(
④ luckily we have `{0}`!", $const))]
                OopsExample,
            }
        }
    }
}

examples!(hns, HNS);
examples!(hnsnn, HNS_NN);
examples!(hxs, HXS);
