use Catsay_TUI::input_step;
use cursive::event::Key;
fn main() {
    let mut siv = cursive::default();
    input_step(&mut siv);
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run();
}

