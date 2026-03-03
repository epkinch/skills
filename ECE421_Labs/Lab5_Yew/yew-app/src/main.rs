use yew::prelude::*;
extern crate meval;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    label: String,
    onclick: Callback<String>,
    onkeydown: Callback<KeyboardEvent>,
}

#[function_component(Button)]
fn calc_button(props: &ButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        let label = props.label.clone();
        Callback::from(move |_| onclick.emit(label.clone()))
    };
    let onkeydown = {
        let onkeydown = props.onkeydown.clone();
        Callback::from(move |e: KeyboardEvent| onkeydown.emit(e))
    };
    html! {
        <input type="button" value={props.label.clone()} {onclick} {onkeydown} />
    }
}

#[function_component(App)]
fn app() -> Html {
    let result = use_state(|| String::new());
    let onclick = Callback::from({
        let result = result.clone();
        move |label| {
            if label == "c" {
                result.set("".to_string());
            } else if label == "=" {
                result.set(meval::eval_str(&*result).unwrap().to_string());
            } else if label == "L" {
                result.set(meval::eval_str(&*result).unwrap().ln().to_string());
            } else {
                result.set(format!("{}{}", *result, label));
            }
        }
    });

    let onkeydown = {
        let result = result.clone();
        Callback::from(move |e: KeyboardEvent| {
            result.set("9".to_string());
            let key = e.key();
            if "0123456789+-*/".contains(&key) {
                result.set(key);
            }
        })
    };

    html! {
        <>
        // Use Table to Create Calculator Structure Design
        <img src="https://opb-opb-prod.cdn.arcpublishing.com/resizer/v2/CR7NCO5JY3H7QBABRBXIMENNUQ.jpg?auth=3a6a0be2b6ae089ae7343248814882e9d85c7051e7603c9f09756f1f6e535a25&width=1572"
        width=400 height=350 style="display: block; margin: 0 auto;"/>
        <table id="calcu">
            <tr>
                <td colspan={3}><input type="text" value={(*result).clone()} /></td>
                <td><Button label={"c"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
            </tr>
            <tr>
                <td><Button label={"1"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"2"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"3"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"/"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
            </tr>
            <tr>
                <td><Button label={"4"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"5"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"6"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"*"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
            </tr>
            <tr>
                <td><Button label={"7"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"8"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"9"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"-"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
            </tr>
            <tr>
                <td><Button label={"0"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"."} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"="} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"+"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
            </tr>
            <tr>
                <td><Button label={"^"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"%"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
                <td><Button label={"L"} onclick={onclick.clone()} onkeydown={onkeydown.clone()} /></td>
            </tr>
        </table>
        </>

    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}