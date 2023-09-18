use std::fs;

#[derive(Debug)]
pub struct Bible {
	pub books: Vec<Book>,
	pub abbreviation: String,
	pub name: String,
	pub language: String,
}

impl Bible {
	pub fn new(abbreviation: String) -> Bible {
		let mut bible = Bible {
			books: Vec::new(),
			abbreviation: abbreviation,
			name: String::new(),
			language: String::new()
		};

		/* Read bible info */
		let info = fs::read_to_string(format!("./bibles/{}.yet", &bible.abbreviation)).unwrap();
		for line in info.lines() {
			let parts = line.split("\t").collect::<Vec<&str>>();
			/* If line starts with "info" */
			if line.starts_with("info") {
				match parts[1] {
					"longName" => bible.name = (*parts[2]).to_string(),
					"locale" => bible.language = (*parts[2]).to_string(),
					&_ => ()
				}
			}
			/* If line starts with "book_name" */
			else if line.starts_with("book_name") {
				bible.books.push(Book {
					name: (*parts[2]).to_string(),
					chapters: Vec::new()
				});
			}
			/* If line starts with "verse" */
			else if line.starts_with("verse") {
				let book = &mut bible.books[parts[1].parse::<usize>().unwrap() - 1];
				let chapter = if (parts[2].parse::<usize>().unwrap() - 1) >= book.chapters.len() {
					book.chapters.push(Chapter {
						number: parts[2].parse::<u8>().unwrap(),
						verses: Vec::new()
					});
					book.chapters.last_mut().unwrap()
				} else {
					&mut book.chapters[parts[2].parse::<usize>().unwrap() - 1]
				};
				chapter.verses.push(Verse {
					number: parts[3].parse::<u8>().unwrap(),
					text: (*parts[4]).to_string()
						.replace("@@@^", "")
						.replace("@@", "")
						.replace("@9", r"\e[3m")
						.replace("@7", r"\e[0m")
				});
			}
		}

		return bible;
	}
}

#[derive(Debug)]
pub struct Book {
	pub name: String,
	pub chapters: Vec<Chapter>
}

#[derive(Debug)]
pub struct Chapter {
	pub number: u8,
	pub verses: Vec<Verse>
}

#[derive(Debug)]
pub struct Verse {
	pub number: u8,
	pub text: String
}

fn list_translations() {
	/* Scan ./bibles for .yet files */
	let mut translations: Vec<String> = Vec::new();
	let paths = fs::read_dir("./bibles").unwrap();
	for result in paths {
		let file_name = result.unwrap().file_name().into_string().unwrap();
		if file_name.ends_with(".yet") {
			translations.push(file_name[..file_name.len()-4].to_string());
		}
	}
}