mod bible;

use bible::Bible;
use cursive::{CursiveRunnable, Cursive};
use cursive::views::{LinearLayout, NamedView, SelectView, ScrollView, TextView};
use rust_i18n::t;
rust_i18n::i18n!("locales");
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
	let mut siv = Rc::new(RefCell::new(cursive::default()));

	siv.borrow_mut().add_global_callback('q', |s| s.quit());

    let bible = Rc::new(Bible::new("kjv".to_string()));

    siv.borrow_mut().add_layer(
        LinearLayout::horizontal()
            .child(build_selector(Rc::clone(&bible)))
            .child(
                ScrollView::new(TextView::new("In the beginning, ..."))
            )
    );

    (move |bible, siv: Rc<RefCell<CursiveRunnable>>| {
        let mut book_view = siv.borrow_mut().find_name::<SelectView<String>>("book_view").unwrap();
        (|bible| {
            (*book_view).set_on_select(move |s, book_name| {
                rebuild_chapter_selector(s, Rc::clone(&bible), book_name.to_string());
                let chapter_number = s.find_name::<SelectView<String>>("chapter_view").unwrap().selected_id().unwrap();
                rebuild_verse_selector(s, Rc::clone(&bible), book_name.to_string(), chapter_number);
            });
        })(Rc::clone(&bible));
        let mut chapter_view = siv.borrow_mut().find_name::<SelectView<String>>("chapter_view").unwrap();
        (*chapter_view).set_on_select(move |s, chapter| {
            let book_name = s.find_name::<SelectView<String>>("book_view").unwrap().selection().unwrap().to_string();
            rebuild_verse_selector(s, Rc::clone(&bible), book_name, chapter.parse::<usize>().unwrap() - 1);
        });
    })(Rc::clone(&bible), Rc::clone(&siv));

    siv.borrow_mut().run();
}

fn build_selector(bible: Rc<Bible>) -> LinearLayout {
    let mut book_view = SelectView::new();
    book_view.add_all_str(bible.books.iter().map(|x| x.name.clone()));
    book_view.set_autojump(true);

    let mut chapter_view = build_chapter_selector(Rc::clone(&bible), get_book_name(&book_view));
    
    let mut verse_view = build_verse_selector(Rc::clone(&bible), &book_view, &chapter_view);

    let mut named_book_view = NamedView::new("book_view", book_view);
    let mut named_chapter_view = NamedView::new("chapter_view", chapter_view);
    let mut named_verse_view = NamedView::new("verse_view", verse_view);
    return LinearLayout::horizontal()
        .child(ScrollView::new(named_book_view))
        .child(ScrollView::new(named_chapter_view))
        .child(ScrollView::new(named_verse_view));
}

fn get_book_name(book_view: &SelectView) -> String { book_view.selection().unwrap().to_string() }
fn build_chapter_selector<'a>(bible: Rc<Bible>, book_name: String) -> SelectView<String> {
    let mut chapter_view = SelectView::new();
    let book_number = (&bible.books).into_iter().position(|x| x.name == book_name).unwrap();
    chapter_view.add_all_str(bible.books[book_number].chapters.iter().map(|x| x.number.to_string()));
    return chapter_view;
}

fn rebuild_chapter_selector<'a>(siv: &mut Cursive, bible: Rc<Bible>, book_name: String) {
    let mut chapter_view = siv.find_name::<SelectView<String>>("chapter_view").unwrap();
    chapter_view.clear();
    let book_number = (&bible.books).into_iter().position(|x| x.name == book_name).unwrap();
    chapter_view.add_all_str(bible.books[book_number].chapters.iter().map(|x| x.number.to_string()));
    chapter_view.set_selection(0);
}

fn build_verse_selector<'a>(bible: Rc<Bible>, book_view: &SelectView, chapter_view: &SelectView) -> SelectView<String> {
    let mut verse_view = SelectView::new();
    let book_name = book_view.selection().unwrap().to_string();
    let book_number = (&bible.books).into_iter().position(|x| x.name == book_name).unwrap();
    let chapter_number = chapter_view.selection().unwrap().to_string().parse::<usize>().unwrap();
    verse_view.add_all_str(bible.books[book_number].chapters[chapter_number].verses.iter().map(|x| x.number.to_string()));
    return verse_view;
}

fn rebuild_verse_selector<'a>(siv: &mut Cursive, bible: Rc<Bible>, book_name: String, chapter_number: usize) {
    let mut verse_view = siv.find_name::<SelectView<String>>("verse_view").unwrap();
    verse_view.clear();
    let book_number = (&bible.books).into_iter().position(|x| x.name == book_name).unwrap();
    verse_view.add_all_str(bible.books[book_number].chapters[chapter_number].verses.iter().map(|x| x.number.to_string()));
    verse_view.set_selection(0);
}
