use atoi::{FromRadix10, FromRadix10Signed, FromRadix16};
#[cfg(debug_assertions)]
use log::debug;

use derive_more::*;
use std::cmp::{Ordering, PartialOrd};
use std::fmt::{Debug, Display, Error as FmtError, Formatter};
use std::str::{from_utf8, from_utf8_unchecked};

use crate::args::Mode;

#[derive(Constructor, Copy, Clone)]
pub struct HumanNumericLine<'a> {
    pub buf: &'a [u8],
    pub mode: Mode,
}

impl Debug for HumanNumericLine<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
        let mut bind = String::new();
        write!(
            fmt,
            "HumNumLine({})",
            from_utf8(self.buf)
                .map(|s| {
                    bind = format!(r#""{}""#, &s[..s.len() - 1]);
                    bind.as_str()
                })
                .unwrap_or("!!INVALID_UTF8!!")
        )
    }
}

impl Display for HumanNumericLine<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
        unsafe { write!(fmt, "{}", from_utf8_unchecked(self.buf)) }
    }
}

impl std::convert::AsRef<[u8]> for HumanNumericLine<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.buf
    }
}

impl PartialEq<dyn AsRef<[u8]>> for HumanNumericLine<'_> {
    fn eq(&self, other: &dyn AsRef<[u8]>) -> bool {
        self.eq(&HumanNumericLine::new(other.as_ref(), self.mode))
    }
}

impl HumanNumericLine<'_> {
    // Note: my implementation is recursive!
    fn humnum_compare(&self, other: &Self) -> Ordering {
        // A buffer is left string in comparison, B buffer is right.
        let a = self.buf;
        let b = other.buf;

        // First, we generate a Non EQual Index
        let neqi = {
            let oneqi = a.iter().zip(b.iter()).position(|(a, b)| a != b);
            if let Some(i) = oneqi {
                i
            } else {
                return Ordering::Equal;
            }
        };

        // Second, we define a bunch of closures which will calculate indices into the buffers.
        type BytesIter<'a> = &'a mut dyn std::iter::Iterator<Item = &'a u8>;
        let take_digits =
            move |i: &usize, a: BytesIter| a.nth(*i).unwrap_or(&('\0' as u8)).is_ascii_digit();
        type IndexIter<'a> = &'a mut dyn std::iter::Iterator<Item = usize>;
        type RangeClbk<'a> = &'a dyn Fn(&usize) -> bool;
        let nrange = move |r: IndexIter, f: RangeClbk| r.take_while(f).last().unwrap_or(neqi);

        let begr = move || (0..neqi).rev();
        let finr = move |buf: &[u8]| (neqi..buf.len());
        let nbeg = move |buf: &[u8]| nrange(&mut begr(), &|i| take_digits(i, &mut buf.into_iter()));
        let nfin = move |buf: &[u8]| nrange(&mut finr(buf), &|i| take_digits(i, &mut buf.into_iter()));
        let xbeg = move |buf: &[u8]| nrange(&mut begr(), &|i| !take_digits(i, &mut buf.into_iter()));
        let xfin = move |buf: &[u8]| nrange(&mut finr(buf), &|i| !take_digits(i, &mut buf.into_iter()));

        // Third, using our ≠ index, we figure out boundaries of ASCII numbers.
        let anbeg = nbeg(a);
        let anfin = nfin(a);
        let bnbeg = nbeg(b);
        let bnfin = nfin(b);

        // Possible bailout: if we don't really have a number. Attempt a regular match,
        // and...
        if self.mode.consider_hex() && (!a[anbeg].is_ascii_hexdigit() || !b[bnbeg].is_ascii_hexdigit())
            || (!a[anbeg].is_ascii_digit() || !b[bnbeg].is_ascii_digit())
        {
            let axbeg = xbeg(a);
            let axfin = xfin(a);
            let bxbeg = xbeg(b);
            let bxfin = xfin(b);
            let mut axs: String;
            let mut bxs: String;
            let ax = &a[axbeg..=axfin];
            let bx = &b[bxbeg..=bxfin];
            let cmp;
            // ...after we check the case sensitivity option...
            if self.mode.insensitive() {
                axs = String::with_capacity(ax.len());
                bxs = String::with_capacity(bx.len());
                unsafe {
                    axs.push_str(from_utf8_unchecked(ax).to_lowercase().as_str());
                    bxs.push_str(from_utf8_unchecked(bx).to_lowercase().as_str());
                }
                cmp = axs.cmp(&bxs);
            } else {
                cmp = ax.cmp(bx);
            }
            if cmp != Ordering::Equal {
                // ...return it if it is not equal, otherwise...
                return cmp;
            } else {
                // ...recurse if matched intermediary bytestrings are equal.
                return HumanNumericLine::new(&a[axfin + 1..a.len()], self.mode)
                    .humnum_compare(&HumanNumericLine::new(&b[bxfin + 1..b.len()], self.mode));
            }
        }

        // This makes the below parse unwrap safe by discarding final \n
        let subrange = |buf: &[u8], mut beg: usize, fin: usize| {
            if self.mode.is_default() && beg >= 1 && buf[beg - 1] == '-' as u8 {
                beg -= 1;
            }
            if fin != buf.len() - 1 {
                beg..fin + 1 // a/k/a beg..=fin
            } else {
                beg..fin
            }
        };

        // Fourth, set up buffers to hold ranges.
        let (an, bn) = (&a[subrange(a, anbeg, anfin)], &b[subrange(b, bnbeg, bnfin)]);

        #[cfg(debug_assertions)]
        debug!("{:?}", (an, anbeg, anfin, bn, bnbeg, bnfin));

        // Finally, do a normal comparison on the numbers.
        let (ai, bi): (i64, i64) = if self.mode.sort_negatives() {
            (
                FromRadix10Signed::from_radix_10_signed(an).0,
                FromRadix10Signed::from_radix_10_signed(bn).0,
            )
        } else {
            if self.mode.consider_hex() {
                (FromRadix16::from_radix_16(an).0, FromRadix16::from_radix_16(bn).0)
            } else {
                (FromRadix10::from_radix_10(an).0, FromRadix10::from_radix_10(bn).0)
            }
        };

        let cmp = ai.cmp(&bi);
        if cmp != Ordering::Equal {
            cmp
        } else {
            // Recurse on rest of line if numbers equal.
            HumanNumericLine::new(&a[anfin + 1..], self.mode)
                .humnum_compare(&HumanNumericLine::new(&b[bnfin + 1..], self.mode))
        }
    }
}

impl PartialOrd<dyn AsRef<[u8]>> for HumanNumericLine<'_> {
    fn partial_cmp(&self, other: &dyn AsRef<[u8]>) -> Option<Ordering> {
        Some(self.humnum_compare(&HumanNumericLine::new(other.as_ref(), self.mode)))
    }
}

impl PartialOrd for HumanNumericLine<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.humnum_compare(other))
    }
}

impl PartialEq for HumanNumericLine<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.humnum_compare(other) == Ordering::Equal
    }
}

impl Eq for HumanNumericLine<'_> {}
impl Ord for HumanNumericLine<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.humnum_compare(other)
    }
}
