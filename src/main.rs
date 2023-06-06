use cursive::{
    views::{Dialog, TextView},
    Cursive,
};

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(Dialog::text("...").title("...").button("next", show_next));

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Do the thing")
            .title("bruh moment")
            .button("ok", |s| ())
            .button("not okay", |s| ()),
    );
}
