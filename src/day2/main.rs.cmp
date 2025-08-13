use std::cmp::Ordering;


struct Key {
    id:u32,
    meta: Option<String>,
}


impl PartialEq for Key {
    fn eq(&self,other:&Self) -> bool {
        self.id == other.id
    }
}



#[derive(Eq,PartialEq)]
struct Citation {
    author:String,
    year:u32,
}

impl PartialOrd for Citation {
    fn partial_cmp(&self,other:&Self) -> Option<Ordering>{
        match self.author.partial_cmp(&other.author) {
            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
            author_ord => author_ord,
        }
        
    }
}
//#[derive(Eq, PartialEq)]
//struct Citation {
//    author: String,
//    year: u32,
//}
//impl PartialOrd for Citation {
//    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//        match self.author.partial_cmp(&other.author) {
//            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
//            author_ord => author_ord,
//        }
//    }
//}


fn main() {
    let a = "h";
    let b = String::from("h");
    assert_eq!(a,b);
}
