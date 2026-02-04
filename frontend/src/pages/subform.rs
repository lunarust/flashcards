use yew::prelude::*;
use web_sys::HtmlInputElement;
//use log::info;

use common::word::Word;

#[derive(Properties, PartialEq)]
pub struct Props {
    //pub key: String,
    pub word: Word,
    pub has_gender: bool,
    pub on_change: Callback<Word>,
}
#[function_component(SubFormDisplay)]
pub fn subform_display( Props { word, has_gender, on_change }: &Props ) -> HtmlResult {
    let k = word.language.iso.clone();
    let mut hg = false;
    let mut style = "".to_string();
    if !has_gender { hg = true;
        style = "display:none".to_string(); }

    let gender_list = common::word::get_gender();

    let edit_name = Callback::from({
        let w = word.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut w = w.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            w.name = input.value();
            on_change.emit(w.clone());
            //info!("CHILD {:?}", w.name);
        }
    });

    let edit_alt = Callback::from({
        let w = word.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut w = w.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            w.alternate = Vec::from_iter(input.value().split(",").map(String::from));
            on_change.emit(w.clone());
        }
    });



    Ok(
    html!{
        <>
            <div class="field">
              <div class="control">
                <input class="input" type="text" placeholder="word"
                    value={word.name.clone()}
                    oninput={edit_name}
                 />
              </div>
            </div>

            <div class="field">
              <div class="control">
                <input class="input" type="text" placeholder="alt"
                    value={word.alternate.clone().join("-")}
                  oninput={edit_alt}
                />
              </div>
            </div>

            //class={selected.then_some("is-selected").unwrap_or("")}
            <div class="checkboxes" {style}>
            <label class="label">{"Gender: "}</label>
                for g in gender_list {

                  <label class="radio">
                    <input type="radio" disabled={hg}
                    name={format!("gender_{}",k)}
                    checked={ if word.gender == g { true } else { false } }
                    onchange={
                            Callback::from({
                              let value = g.clone();
                              let w = word.clone();
                              let on_change = on_change.clone();
                              move |e: Event| {
                                  let mut w = w.clone();
                                  let selected_gender = value.clone();
                                  let input = e.target_dyn_into::<HtmlInputElement>().unwrap();

                                if input.checked() {
                                    w.gender = selected_gender;
                                }
                                on_change.emit(w.clone());
                              }
                            })
                    }
                 />{ g }</label>
              }
            </div>
            </>
    })

}
