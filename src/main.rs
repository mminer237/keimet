mod bible;

use bible::Bible;
use cursive::views::{LinearLayout, SelectView, ScrollView, TextView};
use rust_i18n::t;
rust_i18n::i18n!("locales");
use std::rc::Rc;

fn main() {
	let mut siv = cursive::default();

	siv.add_global_callback('q', |s| s.quit());

    let bible = Bible::new("kjv".to_string());

    siv.add_layer(
        LinearLayout::horizontal()
            .child(build_selector(&bible))
            .child(
                ScrollView::new(TextView::new("In the beginning, ..."))
            )
    );

    siv.run();
}

fn build_selector(bible: &Bible) -> LinearLayout {
    let mut book_view = SelectView::new();
    book_view.add_all_str(bible.books.iter().map(|x| x.name.clone()));
    let book_name = book_view.selection().unwrap().to_string();
    println!("{}", book_name);

    let mut chapter_view = SelectView::new();
    let book_number = (&bible.books).into_iter().position(|x| x.name == book_name).unwrap();
    chapter_view.add_all_str(bible.books[book_number].chapters.iter().map(|x| x.number.to_string()));
    
    let mut verse_view = SelectView::new();
    let chapter_number = chapter_view.selection().unwrap().to_string().parse::<usize>().unwrap();
    verse_view.add_all_str(bible.books[book_number].chapters[chapter_number].verses.iter().map(|x| x.number.to_string()));
    return LinearLayout::horizontal()
        .child(ScrollView::new(book_view))
        .child(ScrollView::new(chapter_view))
        .child(ScrollView::new(verse_view));
}
