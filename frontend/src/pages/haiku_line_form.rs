use yew::prelude::*;
use web_sys::HtmlInputElement;
//use log::info;
use common::{haiku::*};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub haikuline: HaikuLine,
    pub order: i32,
    pub on_change: Callback<HaikuLine>,
}

#[function_component(SubFormDisplay)]
pub fn subform_display( Props { haikuline, order, on_change }: &Props ) -> HtmlResult {
    let idx = order.clone();
    
    let edit_line = Callback::from({
        let hl = haikuline.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut hl = hl.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            hl.line = input.value();
            on_change.emit(hl.clone());
        }
    });

    let edit_alt = Callback::from({
        let hl = haikuline.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut hl = hl.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            hl.alt = input.value();
            on_change.emit(hl.clone());
        }
    });

    let edit_reading = Callback::from({
        let hl = haikuline.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut hl = hl.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            hl.reading = input.value();
            on_change.emit(hl.clone());
        }
    });


    let edit_romaji = Callback::from({
        let hl = haikuline.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut hl = hl.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            hl.romaji = input.value();
            on_change.emit(hl.clone());
        }
    });


    let edit_meaning = Callback::from({
        let hl = haikuline.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut hl = hl.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            hl.meaning = input.value();
            on_change.emit(hl.clone());
        }
    });
    let edit_scene = Callback::from({
        let hl = haikuline.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut hl = hl.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            hl.scene = input.value();
            on_change.emit(hl.clone());
        }
    });
    let edit_image = Callback::from({
        let hl = haikuline.clone();
        let on_change = on_change.clone();
        move |e: InputEvent| {
            let mut hl = hl.clone();
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            hl.image = input.value();
            on_change.emit(hl.clone());
        }
    });


    Ok(
        html!{
        <>
            <h1>{ format!("Line: #{}", idx) }</h1>

            <div class="field"><div class="control">
            <input class="input" type="text" placeholder="line"
                value={haikuline.line.clone()}
                oninput={edit_line}/>
            </div></div>

            <div class="field"><div class="control">
            <input class="input" type="text" placeholder="alt"
                value={haikuline.alt.clone()}
                oninput={edit_alt}/>
            </div></div>

            <div class="field"><div class="control">
            <input class="input" type="text" placeholder="reading"
                value={haikuline.reading.clone()}
                oninput={edit_reading}/>
            </div></div>

            <div class="field"><div class="control">
            <input class="input" type="text" placeholder="romaji"
                value={haikuline.romaji.clone()}
                oninput={edit_romaji}/>
            </div></div>

            <div class="field"><div class="control">
            <input class="input" type="text" placeholder="meaning"
                value={haikuline.meaning.clone()}
                oninput={edit_meaning}/>
            </div></div>

            <div class="field"><div class="control">
            <input class="input" type="text" placeholder="scene"
                value={haikuline.scene.clone()}
                oninput={edit_scene}/>
            </div></div>

            <div class="field"><div class="control">
            <input class="input" type="text" placeholder="image"
                value={haikuline.image.clone()}
                oninput={edit_image}/>
            </div></div>

        </>
        } //html
    ) //ok
}
