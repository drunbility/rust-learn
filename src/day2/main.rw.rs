use std::io::{BufRead,BufReader,Read,Write,Result};


fn counts<R:Read>(reader:R) -> usize {
    let buf_r = BufReader::new(reader);
    buf_r.lines().count()        
}

fn log<W:Write>(writer:&mut W,msg:&str) -> Result<()> {

    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}




fn main() -> Result<()> {
    let slice:&[u8] = b"foo\nlast\nnaz\n";
    println!("slice lines:{}",counts(slice));
    let file = std::fs::File::open(std::env::current_exe()?)?;

    println!("ext: {}",counts(file));

    let mut buf = Vec::new();
    log(&mut buf,"hhh")?;
    log(&mut buf,"ffff")?;
    println!("writer: {buf:?}");
    
    Ok(())
}

