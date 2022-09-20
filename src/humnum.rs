use atoi::FromRadix10Signed;
#[cfg(debug_assertions)]
use log::debug;

use derive_more::*;
use std::cmp::{Ordering, PartialOrd};
use std::fmt::{Debug, Display, Error as FmtError, Formatter};
use std::str::{from_utf8, from_utf8_unchecked};

#[derive(Constructor, From, Deref, AsRef, AsMut, Copy, Clone)]
pub struct HumanNumericLine<'a>(pub &'a [u8]);

impl Debug for HumanNumericLine<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
        let mut bind = String::new();
        write!(
            fmt,
            "HumNumLine({})",
            from_utf8(self.0)
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
        unsafe { write!(fmt, "{}", from_utf8_unchecked(self.0)) }
    }
}

impl std::convert::AsRef<[u8]> for HumanNumericLine<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl PartialEq<dyn AsRef<[u8]>> for HumanNumericLine<'_> {
    fn eq(&self, other: &dyn AsRef<[u8]>) -> bool {
        self.eq(&HumanNumericLine(other.as_ref()))
    }
}

impl HumanNumericLine<'_> {
    // Note: my implementation is recursive!
    fn humnum_compare(&self, other: &Self) -> Ordering {
        // A buffer is left string in comparison, B buffer is right.
        let a = self.0;
        let b = other.0;

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
        let take_digits = |i: &usize, a: BytesIter| a.nth(*i).unwrap_or(&('\0' as u8)).is_ascii_digit();
        type IndexIter<'a> = &'a mut dyn std::iter::Iterator<Item = usize>;
        type RangeClbk<'a> = &'a dyn Fn(&usize) -> bool;
        let nrange = |r: IndexIter, f: RangeClbk| r.take_while(f).last().unwrap_or(neqi);

        let begr = || (0..neqi).rev();
        let finr = |buf: &[u8]| (neqi..buf.len());
        let nbeg = |buf: &[u8]| nrange(&mut begr(), &|i| take_digits(i, &mut buf.into_iter()));
        let nfin = |buf: &[u8]| nrange(&mut finr(buf), &|i| take_digits(i, &mut buf.into_iter()));
        let xbeg = |buf: &[u8]| nrange(&mut begr(), &|i| !take_digits(i, &mut buf.into_iter()));
        let xfin = |buf: &[u8]| nrange(&mut finr(buf), &|i| !take_digits(i, &mut buf.into_iter()));

        // Third, using our ≠ index, we figure out boundaries of ASCII numbers.
        let anbeg = nbeg(a);
        let anfin = nfin(a);
        let bnbeg = nbeg(b);
        let bnfin = nfin(b);

        // Possible bailout: if we don't really have a number. Attempt a regular match,
        // and...
        if !a[anbeg].is_ascii_digit() || !b[bnbeg].is_ascii_digit() {
            let axbeg = xbeg(a);
            let axfin = xfin(a);
            let bxbeg = xbeg(b);
            let bxfin = xfin(b);
            let ax = &a[axbeg..=axfin];
            let bx = &b[bxbeg..=bxfin];
            let cmp = ax.cmp(bx);
            if cmp != Ordering::Equal {
                // ...return it if it is not equal, otherwise...
                return cmp;
            } else {
                // ...recurse if matched intermediary bytestrings are equal.
                return HumanNumericLine(&a[axfin + 1..a.len()])
                    .humnum_compare(&HumanNumericLine(&b[bxfin + 1..b.len()]));
            }
        }

        // This makes the below parse unwrap safe by discarding final \n
        let subrange = |buf: &[u8], mut beg: usize, fin: usize| {
            if beg != 0 && buf[beg-1] == '-' as u8 {
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
        let ai: u64 = FromRadix10Signed::from_radix_10_signed(an).0;
        let bi: u64 = FromRadix10Signed::from_radix_10_signed(bn).0;

        let cmp = ai.cmp(&bi);
        if cmp != Ordering::Equal {
            cmp
        } else {
            // Recurse on rest of line if numbers equal.
            HumanNumericLine(&a[anfin + 1..]).humnum_compare(&HumanNumericLine(&b[bnfin + 1..]))
        }
    }
}

impl PartialOrd<dyn AsRef<[u8]>> for HumanNumericLine<'_> {
    fn partial_cmp(&self, other: &dyn AsRef<[u8]>) -> Option<Ordering> {
        Some(self.humnum_compare(&HumanNumericLine(other.as_ref())))
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
