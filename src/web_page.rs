use actix_files::Files;

pub fn vue() -> Files {
    Files::new("/", "./static").index_file("index.html")
}