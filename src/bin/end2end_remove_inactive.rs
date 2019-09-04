use cursive::views::TextView;
use cursive_tabs::TabView;

use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();
    let mut tabs = TabView::new()
        .with_view(0, TextView::new("First"))
        .with_view(1, TextView::new("Second"));
    tabs.remove_view(0).expect("Removal failed.");
    siv.run();
}
