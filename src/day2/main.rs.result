use std::fs::File;
use std::io::Read;


fn main() {
    let file:Result<File,std::io::Error> = File::open("main.rs.cmp");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents){
                println!("dir:{contents} ({bytes} bytes)");
            }else{
                println!("could not readed");
            }            
        }
        Err(err) => {
            println!("not opened:{err}");
        }
    }
}
