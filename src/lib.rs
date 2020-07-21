use cursive::traits::Identifiable; //for with_name() and .call_on_name()
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};
use cursive::Cursive;

//Wrap all form field in one struct
pub struct CatSayOption<'a> {
    pub message: &'a str,
    pub dead: bool,
}

pub fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    .child("Message:", EditView::new().with_name("message"))
                    .child("Dead", Checkbox::new().with_name("dead"))      
            )
            .button("submit", |s| {
                //Get input from form and creates a new CatSayOptions Struct
                let message = s
                    .call_on_name("message", |t: &mut EditView| t.get_content()).unwrap();
                let is_dead = s.call_on_name("dead", |t: &mut Checkbox| t.is_checked()).unwrap();
                let options = CatSayOption{
                    message: if message.is_empty() { "Nothing" }else { &message }, 
                    dead: is_dead
                };
                //Call result step with info provided
                result_step(s, &options);
            })
    )
}

pub fn result_step(siv: &mut Cursive, options: &CatSayOption){
    let eye = if options.dead { "X" } else { "0"};
    let cat_text = format!("{message}
    \\
     \\
      /\\_/\\
     ( {eye} {eye} )
     =( I )=",
     message = options.message, 
     eye = eye);

     siv.pop_layer();
     siv.add_layer(Dialog::around(TextView::new(cat_text))
         .title("The cat says ....")
         .button("Done", |s | s.quit())
        )
}