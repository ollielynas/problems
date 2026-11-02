use std::simd::prelude::*;

const LANES: usize = 32;
type V = Simd<u8, LANES>;

/// What `A` pairs with in `seq`: `U` if the sequence is RNA, otherwise `T`.
#[inline(always)]
fn a_partner(seq: &[u8]) -> u8 {
    if seq.contains(&b'U') { b'U' } else { b'T' }
}

#[inline(always)]
fn complement_v(v: V, a_partner: u8) -> V {
    let a = v.simd_eq(V::splat(b'A'));
    let t = v.simd_eq(V::splat(b'T'));
    let u = v.simd_eq(V::splat(b'U'));
    let cg = v.simd_eq(V::splat(b'C')) | v.simd_eq(V::splat(b'G'));
    v ^ a.select(V::splat(b'A' ^ a_partner), V::splat(0))
        ^ t.select(V::splat(b'T' ^ b'A'), V::splat(0))
        ^ u.select(V::splat(b'U' ^ b'A'), V::splat(0))
        ^ cg.select(V::splat(b'C' ^ b'G'), V::splat(0))
}

#[inline(always)]
fn complement(b: u8, a_partner: u8) -> u8 {
    let cg = (b == b'C') | (b == b'G');
    let to_a = (b == b'T') | (b == b'U');
    b ^ ((b'A' ^ a_partner) * (b == b'A') as u8)
        ^ ((b ^ b'A') * to_a as u8)
        ^ ((b'C' ^ b'G') * cg as u8)
}

/// Picks the best implementation for the CPU at runtime, so one build runs anywhere.
pub fn reverse_complement_u8(dna: &[u8]) -> Vec<u8> {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        return unsafe { reverse_complement_avx2(dna) };
    }
    reverse_complement_impl(dna)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn reverse_complement_avx2(dna: &[u8]) -> Vec<u8> {
    reverse_complement_impl(dna)
}

#[inline(always)]
fn reverse_complement_impl(dna: &[u8]) -> Vec<u8> {
    let n = dna.len();
    let mut out = vec![0u8; n];
    let partner = a_partner(dna);

    let mut chunks = dna.chunks_exact(LANES);
    for (i, chunk) in chunks.by_ref().enumerate() {
        let r = complement_v(V::from_slice(chunk), partner).reverse();
        let end = n - i * LANES;
        r.copy_to_slice(&mut out[end - LANES..end]);
    }

    let rem = chunks.remainder();
    for (o, &b) in out[..rem.len()].iter_mut().zip(rem.iter().rev()) {
        *o = complement(b, partner);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scalar(dna: &[u8]) -> Vec<u8> {
        let partner = a_partner(dna);
        dna.iter().rev().map(|&b| complement(b, partner)).collect()
    }

    #[test]
    fn matches_scalar_around_chunk_boundaries() {
        for n in [0, 1, 7, 31, 32, 33, 63, 64, 65, 100, 1000] {
            let dna: Vec<u8> = b"AAAACCCGGTN".iter().cycle().take(n).copied().collect();
            assert_eq!(reverse_complement_u8(&dna), scalar(&dna), "length {n}");
        }
    }

    #[test]
    fn every_implementation_agrees() {
        for n in [0, 1, 31, 32, 33, 64, 65, 1000] {
            let dna: Vec<u8> = b"AAAACCCGGTN".iter().cycle().take(n).copied().collect();
            assert_eq!(reverse_complement_impl(&dna), scalar(&dna), "length {n}");
            #[cfg(target_arch = "x86_64")]
            if is_x86_feature_detected!("avx2") {
                let avx2 = unsafe { reverse_complement_avx2(&dna) };
                assert_eq!(avx2, scalar(&dna), "avx2, length {n}");
            }
        }
    }

    #[test]
    fn rosalind_sample() {
        assert_eq!(reverse_complement_u8(b"AAAACCCGGT"), b"ACCGGGTTTT");
    }

    #[test]
    fn rna_stays_rna() {
        assert_eq!(reverse_complement_u8(b"AAAACCCGGU"), b"ACCGGGUUUU");
    }

    #[test]
    fn rna_matches_scalar_around_chunk_boundaries() {
        for n in [1, 7, 31, 32, 33, 63, 64, 65, 100, 1000] {
            let rna: Vec<u8> = b"AAAACCCGGUN".iter().cycle().take(n).copied().collect();
            assert_eq!(reverse_complement_u8(&rna), scalar(&rna), "length {n}");
            assert_eq!(reverse_complement_impl(&rna), scalar(&rna), "impl, length {n}");
        }
    }

    #[test]
    fn a_late_u_still_makes_every_a_pair_with_u() {
        // The only `U` is in the last chunk, well after the first `A`.
        let mut rna = vec![b'A'; 100];
        rna.push(b'U');
        let mut expected = vec![b'A'];
        expected.extend_from_slice(&[b'U'; 100]);
        assert_eq!(reverse_complement_u8(&rna), expected);
    }
}

// ---------------------------------------------------------------------------
// Translation: DNA/RNA -> protein (frame 0). Stop = '*', unknown codon = 'X'.
// ---------------------------------------------------------------------------

use std::simd::simd_swizzle;

/// NCBI standard codon table, indexed T=0, C=1, A=2, G=3 (first base * 16 + second * 4 + third).
const NCBI: &[u8; 64] = b"FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG";

/// Cheap 2-bit encoding: (b >> 1) & 3 gives A=0, C=1, T=2, G=3, and U=2 (same as T).
#[inline(always)]
fn enc(b: u8) -> usize {
    ((b >> 1) & 3) as usize
}

/// Codon table re-indexed for `enc`, built at compile time from the NCBI string.
const CODON_TABLE: [u8; 64] = {
    let ours_to_ncbi = [2usize, 1, 0, 3]; // A, C, T, G -> NCBI positions
    let mut t = [0u8; 64];
    let mut i = 0;
    while i < 64 {
        let (a, b, c) = (i >> 4, (i >> 2) & 3, i & 3);
        t[i] = NCBI[ours_to_ncbi[a] * 16 + ours_to_ncbi[b] * 4 + ours_to_ncbi[c]];
        i += 1;
    }
    t
};

#[inline(always)]
fn is_base(b: u8) -> bool {
    matches!(b, b'A' | b'C' | b'G' | b'T' | b'U')
}

#[inline(always)]
fn translate_codon(c: &[u8]) -> u8 {
    if is_base(c[0]) & is_base(c[1]) & is_base(c[2]) {
        CODON_TABLE[(enc(c[0]) << 4) | (enc(c[1]) << 2) | enc(c[2])]
    } else {
        b'X'
    }
}

const CODONS: usize = 16; // codons per SIMD step (48 input bytes)
type W = Simd<u8, CODONS>;
type Wide = Simd<u8, 64>;

const fn stride3(off: usize) -> [usize; CODONS] {
    let mut a = [0; CODONS];
    let mut i = 0;
    while i < CODONS {
        a[i] = 3 * i + off;
        i += 1;
    }
    a
}
const S0: [usize; CODONS] = stride3(0);
const S1: [usize; CODONS] = stride3(1);
const S2: [usize; CODONS] = stride3(2);

#[inline(always)]
fn is_base_v(b: W) -> Mask<i8, CODONS> {
    b.simd_eq(W::splat(b'A'))
        | b.simd_eq(W::splat(b'C'))
        | b.simd_eq(W::splat(b'G'))
        | b.simd_eq(W::splat(b'T'))
        | b.simd_eq(W::splat(b'U'))
}

/// Translates 16 codons given their first, second and third bases.
#[inline(always)]
fn translate_v(b0: W, b1: W, b2: W) -> W {
    let enc_v = |b: W| (b >> W::splat(1)) & W::splat(3);
    let hi = enc_v(b0); // picks one of four 16-entry sub-tables
    let lo = (enc_v(b1) << W::splat(2)) | enc_v(b2); // index within it

    let mut aa = W::splat(0);
    for k in 0..4 {
        let sub = W::from_slice(&CODON_TABLE[16 * k..16 * k + 16]);
        aa = hi
            .simd_eq(W::splat(k as u8))
            .select(sub.swizzle_dyn(lo), aa);
    }

    let ok = is_base_v(b0) & is_base_v(b1) & is_base_v(b2);
    ok.select(aa, W::splat(b'X'))
}

/// Translates frame 0 of `dna`; a trailing partial codon is ignored.
pub fn translate_u8(dna: &[u8]) -> Vec<u8> {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        return unsafe { translate_avx2(dna) };
    }
    translate_impl(dna)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn translate_avx2(dna: &[u8]) -> Vec<u8> {
    translate_impl(dna)
}

#[inline(always)]
fn translate_impl(dna: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(dna.len() / 3);
    translate_into(dna, &mut out);
    out
}

/// Appends the translation of frame 0 of `dna` to `out`.
#[inline(always)]
fn translate_into(dna: &[u8], out: &mut Vec<u8>) {
    let mut chunks = dna.chunks_exact(3 * CODONS);
    for chunk in chunks.by_ref() {
        let mut buf = [0u8; 64];
        buf[..3 * CODONS].copy_from_slice(chunk);
        let v = Wide::from_array(buf);
        let aa = translate_v(
            simd_swizzle!(v, S0),
            simd_swizzle!(v, S1),
            simd_swizzle!(v, S2),
        );
        out.extend_from_slice(aa.as_array());
    }

    out.extend(chunks.remainder().chunks_exact(3).map(translate_codon));
}

#[cfg(test)]
mod translate_tests {
    use super::*;

    fn scalar(dna: &[u8]) -> Vec<u8> {
        dna.chunks_exact(3).map(translate_codon).collect()
    }

    /// All 64 codons, followed by a few with junk bases.
    fn every_codon() -> Vec<u8> {
        let b = b"ACGT";
        let mut v: Vec<u8> = (0..64)
            .flat_map(|i| [b[i >> 4], b[(i >> 2) & 3], b[i & 3]])
            .collect();
        v.extend_from_slice(b"NACAGNTTU");
        v
    }

    #[test]
    fn matches_scalar_around_chunk_boundaries() {
        let src = every_codon();
        for n in [0, 1, 2, 3, 47, 48, 49, 50, 95, 96, 97, 201, 1000] {
            let dna: Vec<u8> = src.iter().cycle().take(n).copied().collect();
            assert_eq!(translate_u8(&dna), scalar(&dna), "length {n}");
            assert_eq!(translate_impl(&dna), scalar(&dna), "impl, length {n}");
        }
    }

    #[test]
    fn standard_codons() {
        assert_eq!(translate_u8(b"ATGTAATAGTGATGGTTTGGGNNN"), b"M***WFGX");
    }

    #[test]
    fn rosalind_prot_sample() {
        let rna = b"AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
        assert_eq!(translate_u8(rna), b"MAMAPRTEINSTRING*");
    }
}

// ---------------------------------------------------------------------------
// Protein detection: every open reading frame on both strands.
// ---------------------------------------------------------------------------

/// Distinct proteins from all six reading frames of `dna`, longest first.
///
/// A protein runs from an `M` to the next stop codon in the same frame. Every
/// `M` starts its own protein, and one with no stop codon after it is dropped.
pub fn find_proteins_u8(dna: &[u8]) -> Vec<String> {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        return unsafe { find_proteins_avx2(dna) };
    }
    find_proteins_impl(dna)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn find_proteins_avx2(dna: &[u8]) -> Vec<String> {
    find_proteins_impl(dna)
}

#[inline(always)]
fn find_proteins_impl(dna: &[u8]) -> Vec<String> {
    let rc = reverse_complement_impl(dna);

    // All six frames share one buffer, and proteins are slices of it until the end.
    let mut buf = Vec::with_capacity(2 * dna.len());
    let mut frame_ends = [0; 6];
    for (i, end) in frame_ends.iter_mut().enumerate() {
        let strand = if i < 3 { dna } else { &rc[..] };
        translate_into(strand.get(i % 3..).unwrap_or_default(), &mut buf);
        *end = buf.len();
    }
    // Translation only writes ASCII, so checking once here covers every slice below.
    let text = std::str::from_utf8(&buf).unwrap();

    let mut found = Vec::new();
    let mut starts = Vec::new();
    let mut begin = 0;
    for end in frame_ends {
        scan_frame(text, begin, end, &mut starts, &mut found);
        begin = end;
    }

    // Comparing lengths first means bytes are only compared between equal lengths.
    // Proteins from one run between stops are suffixes of each other, so their
    // lengths always differ, which keeps long runs of `M` cheap to sort.
    found.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));
    found.dedup();
    found.into_iter().map(str::to_owned).collect()
}

/// Adds the proteins in `text[begin..end]`, which holds a single translated frame.
#[inline(always)]
fn scan_frame<'a>(
    text: &'a str,
    begin: usize,
    end: usize,
    starts: &mut Vec<usize>,
    found: &mut Vec<&'a str>,
) {
    starts.clear();

    let mut scan = |v: V, base: usize| {
        let stops = v.simd_eq(V::splat(b'*')).to_bitmask();
        let mut events = stops | v.simd_eq(V::splat(b'M')).to_bitmask();
        while events != 0 {
            let bit = events.trailing_zeros() as usize;
            events &= events - 1;
            let pos = base + bit;
            if (stops >> bit) & 1 == 1 {
                found.extend(starts.drain(..).map(|start| &text[start..pos]));
            } else {
                starts.push(pos);
            }
        }
    };

    let mut chunks = text.as_bytes()[begin..end].chunks_exact(LANES);
    let mut base = begin;
    for chunk in chunks.by_ref() {
        scan(V::from_slice(chunk), base);
        base += LANES;
    }
    // The zero padding can't match `M` or `*`.
    let mut last = [0u8; LANES];
    let rem = chunks.remainder();
    last[..rem.len()].copy_from_slice(rem);
    scan(V::from_array(last), base);
}

#[cfg(test)]
mod protein_tests {
    use super::*;

    /// The direct version: walks each frame one protein at a time.
    fn naive(dna: &[u8]) -> Vec<String> {
        let partner = a_partner(dna);
        let rc: Vec<u8> = dna.iter().rev().map(|&b| complement(b, partner)).collect();
        let mut out = Vec::new();
        for strand in [dna, &rc[..]] {
            for off in 0..3 {
                let frame: Vec<u8> = strand
                    .get(off..)
                    .unwrap_or_default()
                    .chunks_exact(3)
                    .map(translate_codon)
                    .collect();
                for (i, _) in frame.iter().enumerate().filter(|(_, c)| **c == b'M') {
                    if let Some(n) = frame[i..].iter().position(|&c| c == b'*') {
                        out.push(String::from_utf8(frame[i..i + n].to_vec()).unwrap());
                    }
                }
            }
        }
        out.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));
        out.dedup();
        out
    }

    /// Small repeatable generator, biased towards start and stop codons when `motifs` is set.
    fn random_dna(n: usize, seed: u64, motifs: bool) -> Vec<u8> {
        let mut x = seed.wrapping_mul(2).wrapping_add(1);
        let mut next = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        let mut dna = Vec::with_capacity(n + 3);
        while dna.len() < n {
            let r = next();
            if motifs && r % 4 == 0 {
                dna.extend_from_slice([&b"ATG"[..], b"TAA", b"TAG", b"TGA"][(r >> 8) as usize % 4]);
            } else {
                dna.push(b"ACGT"[(r >> 8) as usize % 4]);
            }
        }
        dna.truncate(n);
        dna
    }

    #[test]
    fn rosalind_sample() {
        let dna = b"AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG";
        assert_eq!(
            find_proteins_u8(dna),
            ["MLLGSFRLIPKETLIQVAGSSPCNLS", "MGMTPRLGLESLLE", "MTPRLGLESLLE", "M"]
        );
    }

    #[test]
    fn matches_naive_on_random_dna() {
        for seed in 1..40 {
            for n in [0, 1, 2, 3, 4, 5, 95, 96, 97, 98, 99, 100, 101, 287, 288, 289, 2000] {
                for motifs in [false, true] {
                    let dna = random_dna(n, seed, motifs);
                    assert_eq!(find_proteins_u8(&dna), naive(&dna), "n={n} seed={seed}");
                    assert_eq!(find_proteins_impl(&dna), naive(&dna), "impl n={n} seed={seed}");
                }
            }
        }
    }

    #[test]
    fn rna_gives_the_same_proteins_as_dna() {
        for seed in 1..20 {
            for n in [1, 3, 97, 100, 288, 2000] {
                let dna = random_dna(n, seed, true);
                let rna: Vec<u8> = dna.iter().map(|&b| if b == b'T' { b'U' } else { b }).collect();
                assert_eq!(find_proteins_u8(&rna), find_proteins_u8(&dna), "n={n} seed={seed}");
            }
        }
    }

    #[test]
    fn needs_a_stop_codon() {
        // ATGAAA has no stop on either strand, in any frame.
        assert!(find_proteins_u8(b"ATGAAA").is_empty());
    }

    #[test]
    fn open_protein_does_not_cross_into_the_next_frame() {
        // Frame 0 ends with an open `M`, and frame 1 begins with a stop.
        assert_eq!(naive(b"ATGA"), find_proteins_u8(b"ATGA"));
        assert!(find_proteins_u8(b"ATGA").is_empty());
    }

    #[test]
    fn repeated_proteins_appear_once() {
        assert_eq!(find_proteins_u8(b"ATGTAAATGTAA"), ["M"]);
    }

    #[test]
    fn every_m_starts_a_protein() {
        assert_eq!(find_proteins_u8(b"ATGATGTAA"), ["MM", "M"]);
    }

    #[test]
    fn junk_bases_do_not_panic() {
        assert_eq!(find_proteins_u8(b"ATGNNNTAA"), ["MX"]);
        let _ = find_proteins_u8("ATGéTAA".as_bytes());
    }
}
