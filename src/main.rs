use cursive::views::{LinearLayout, SelectView, ScrollView, TextView};

fn main() {
	let mut siv = cursive::default();

	siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(
        LinearLayout::horizontal()
            .child(build_selector())
            .child(
                ScrollView::new(TextView::new("In the beginning, ..."))
            )
    );

    siv.run();
}

fn build_selector() -> LinearLayout {
    let mut book_view = SelectView::new();
    book_view.add_all_str([
        "Genesis",
        "Exodus",
        "Leviticus"
    ]);
    let mut chapter_view = SelectView::new();
    chapter_view.add_all_str((1..50 + 1).map(|x| x.to_string()));
    let mut verse_view = SelectView::new();
    verse_view.add_all_str((1..31 + 1).map(|x| x.to_string()));
    return LinearLayout::horizontal()
        .child(ScrollView::new(book_view))
        .child(ScrollView::new(chapter_view))
        .child(ScrollView::new(verse_view));
}
