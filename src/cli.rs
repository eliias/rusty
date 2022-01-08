use std::io;

pub(crate) fn input() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}
