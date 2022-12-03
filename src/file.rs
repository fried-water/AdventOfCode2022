use std::io::BufRead;

pub fn read_lines<P>(filename: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file)
        .lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .collect())
}
