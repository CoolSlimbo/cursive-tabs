use cursive::view::Identifiable;
use cursive::views::{Dialog, TextView, TextArea};
use cursive::Cursive;
use cursive_tabs::TabPanel;

fn main() {
    let mut siv = Cursive::default();
    let mut panel = TabPanel::new()
        .with_view(0, TextView::new("This is on ofthe first views, definetly not the last there will be more to come in due time, but at the moment that is all we have, but to besure in the future there shall be plenty more!"))
        .with_view(1, TextArea::new())
        .with_view(2, TextView::new("This is the third view!"))
        .with_view(3, TextView::new("This is the fourth view!"));
    panel.set_tab(0).expect("oh no");

    siv.add_layer(Dialog::around(panel.with_id("Tabs")).button("Next", |siv| {
        let mut tabs: cursive::views::ViewRef<TabPanel<i32>> =
            siv.find_id("Tabs").expect("id not found");
        let pos = (tabs.tab().unwrap() + 1) % 4;
        tabs.set_tab(pos).expect("Switch refused");
    }));

    siv.add_global_callback('q', |siv| siv.quit());

    siv.run();
}
