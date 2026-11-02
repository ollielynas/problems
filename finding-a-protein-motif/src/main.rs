//! Finding a Protein Motif
use regex::Regex;
use std::{error::Error, fs, io::Read};

/// Parse FASTA into (id, sequence) pairs, in file order.
fn fasta(text: &str) -> Vec<(String, String)> {
    text.split('>')
        .skip(1)
        .map(|block| {
            let mut lines = block.lines();
            let name = lines.next().unwrap_or("").trim().to_string();
            (name, lines.map(str::trim).collect())
        })
        .collect()
}

/// Convert Rosalind motif shorthand (e.g. "N{P}[ST]{P}") into a regex
/// anchored at the start, so it can be tested at every offset (overlaps included).
fn shorthand_to_regex(shorthand: &str) -> Result<Regex, regex::Error> {
    let mut pattern = String::with_capacity(shorthand.len() * 2);
    let mut in_class = false;
    for c in shorthand.chars() {
        match c {
            '{' => {
                pattern.push_str("[^");
                in_class = true;
            }
            '[' => {
                pattern.push('[');
                in_class = true;
            }
            '}' | ']' => {
                pattern.push(']');
                in_class = false;
            }
            'x' | 'X' if !in_class => pattern.push('.'),
            other => pattern.push(other),
        }
    }
    Regex::new(&format!("^(?:{pattern})"))
}

/// 1-based start positions of every (possibly overlapping) match.
fn motif_positions(re: &Regex, seq: &str) -> Vec<usize> {
    seq.char_indices()
        .filter(|&(i, _)| re.is_match(&seq[i..]))
        .map(|(i, _)| i + 1)
        .collect()
}

/// GET a URL, retrying a few times on 5xx responses (UniProt returns
/// transient 500s, sometimes with a truncated body).
fn get_bytes(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    const ATTEMPTS: u32 = 4;
    for attempt in 1.. {
        match ureq::get(url).call().and_then(|mut r| r.body_mut().read_to_vec()) {
            Ok(bytes) => return Ok(bytes),
            Err(ureq::Error::StatusCode(code)) if code >= 500 && attempt < ATTEMPTS => {
                eprintln!("{url}: HTTP {code}, retrying ({attempt}/{ATTEMPTS})");
                std::thread::sleep(std::time::Duration::from_secs(1 << attempt));
            }
            Err(e) => return Err(format!("{url}: {e}").into()),
        }
    }
    unreachable!()
}

/// GET a URL and parse the first FASTA record, if any.
/// Handles gzip-compressed responses. Errors if the record contains
/// anything other than amino-acid letters.
fn fetch_first_record(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    let bytes = get_bytes(url)?;

    let body = if bytes.starts_with(&[0x1f, 0x8b]) {
        let mut text = String::new();
        flate2::read::MultiGzDecoder::new(&bytes[..]).read_to_string(&mut text)?;
        text
    } else {
        String::from_utf8(bytes)?
    };

    let Some((_, seq)) = fasta(&body).into_iter().next() else {
        return Ok(None);
    };
    if seq.is_empty() {
        return Ok(None);
    }
    if let Some(bad) = seq.chars().find(|c| !c.is_ascii_uppercase()) {
        let preview: String = body.chars().take(200).collect();
        return Err(format!(
            "{url}: unexpected character {bad:?} in sequence.\nResponse starts with:\n{preview}"
        )
        .into());
    }
    Ok(Some(seq))
}

/// Fetch a protein sequence from UniProt. IDs like "P07204_TRBM_HUMAN"
/// are reduced to their accession ("P07204") for the URL. Retired entries
/// fall back to UniSave's archived versions.
fn fetch_sequence(id: &str) -> Result<String, Box<dyn Error>> {
    let accession = id.split('_').next().unwrap_or(id);

    let current = format!("https://rest.uniprot.org/uniprotkb/{accession}.fasta");
    if let Some(seq) = fetch_first_record(&current)? {
        return Ok(seq);
    }

    // Asking UniSave for FASTA without `versions` returns every archived
    // version (100KB+, ~20s for long-lived entries). Look up the newest
    // entry version from the (much lighter) metadata listing, which is
    // ordered newest first, and fetch only that one.
    let not_found = || format!("no sequence found for {id} in UniProtKB or UniSave");
    let listing = String::from_utf8(get_bytes(&format!(
        "https://rest.uniprot.org/unisave/{accession}"
    ))?)?;
    let latest = Regex::new(r#""entryVersion":(\d+)"#)?
        .captures(&listing)
        .map(|c| c[1].to_string())
        .ok_or_else(not_found)?;

    let archived =
        format!("https://rest.uniprot.org/unisave/{accession}?format=fasta&versions={latest}");
    fetch_first_record(&archived)?.ok_or_else(|| not_found().into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let dir = env!("CARGO_MANIFEST_DIR");
    let text = fs::read_to_string(format!("{dir}/input.txt"))?;
    let re = shorthand_to_regex("N{P}[ST]{P}")?;

    let mut blocks = Vec::new();
    for id in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let seq = fetch_sequence(id)?;
        let positions = motif_positions(&re, &seq);
        if !positions.is_empty() {
            let joined = positions
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            blocks.push(format!("{id}\n{joined}"));
        }
    }

    let answer = blocks.join("\n");
    println!("{answer}");
    fs::write(format!("{dir}/output.txt"), format!("{answer}\n"))?;
    Ok(())
}
