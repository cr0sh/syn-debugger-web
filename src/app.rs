#![allow(non_camel_case_types)]

use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ParseMethod {
    TokenStream,
    Item,
    Unknown,
}

#[function_component(App)]
pub fn app() -> Html {
    let input_ref = use_node_ref();
    let input_value = use_state_eq(String::new);
    let parse_method = use_state_eq(|| ParseMethod::Item);
    let onchange = use_callback(
        {
            move |_,
                  (input_ref, input_value, parse_method): &(
                NodeRef,
                UseStateHandle<String>,
                UseStateHandle<ParseMethod>,
            )| {
                let ta = input_ref.cast::<HtmlTextAreaElement>().unwrap();
                let value = ta.value();
                match **parse_method {
                    ParseMethod::TokenStream => {
                        let syntax = syn::parse_str::<proc_macro2::TokenStream>(&value);
                        match syntax {
                            Ok(x) => input_value.set(format!("{x:#?}")),
                            Err(e) => input_value.set(format!("{e:#?}")),
                        }
                    }
                    ParseMethod::Item => {
                        let syntax = syn::parse_str::<syn::Item>(&value);
                        match syntax {
                            Ok(x) => input_value.set(format!("{x:#?}")),
                            Err(e) => input_value.set(format!("{e:#?}")),
                        }
                    }
                    ParseMethod::Unknown => {
                        input_value.set(String::from("it should not be selected"))
                    }
                }
            }
        },
        (input_ref.clone(), input_value.clone(), parse_method.clone()),
    );

    use_effect_with_deps(
        {
            let onchange = onchange.clone();
            move |_| {
                let interval = Interval::new(400, move || {
                    onchange.emit(());
                });
                move || {
                    interval.cancel();
                }
            }
        },
        onchange.clone(),
    );

    html! {
        <main>
            <div class={ classes!("flex", "flex-col", "items-stretch", "px-20", "py-10", "h-screen") }>
                <h1 class={ classes!("py-5", "text-xl") }>{ format!("syn-debugger-web v{}", env!("CARGO_PKG_VERSION")) }</h1>
                <div class={ classes!("flex", "flex-row", "grow", "h-4/5") }>
                    <div class={ classes!("flex", "flex-col", "px-5", "text-md", "w-1/2") }>
                        <Input {input_ref} onchange={onchange.clone()}/>
                    </div>
                    <div class={ classes!("flex", "flex-col", "px-5", "text-md", "w-1/2") }>
                        <Output output={input_value} {parse_method} {onchange}/>
                    </div>
                </div>
                <span class={ classes!("text-md", "py-2") }>
                        { "(c) 2023 Junghyun Nam, dual-licensed under Apache 2.0/MIT " }
                        <a class={ classes!("text-blue-600", "underline") } href={"https://github.com/cr0sh/syn-debugger-web"} target={"_blank"}>
                            { "source code" }
                        </a>
                </span>
            </div>
        </main>
    }
}

#[derive(PartialEq, Properties)]
struct InputProperties {
    input_ref: NodeRef,
    onchange: Callback<(), ()>,
}

#[function_component(Input)]
fn input(props: &InputProperties) -> Html {
    let onchange = props.onchange.clone();
    html! {
        <>
            <div class={ classes!("text-lg", "py-2") }>{ "Rust code" }</div>
            <textarea ref={props.input_ref.clone()} onchange={move |_| onchange.emit(())} class={
                classes!(
                    "w-full",
                    "flex-1",
                    "px-2",
                    "pt-2",
                    "text-justify",
                    "text-start",
                    "font-mono",
                    "bg-slate-100",
                    "border-solid",
                    "border-2",
                    "border-slate-800",
                    "rounded-md"
                )
            }/>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct OutputProperties {
    output: UseStateHandle<String>,
    onchange: Callback<(), ()>,
    parse_method: UseStateHandle<ParseMethod>,
}

#[function_component(Output)]
#[allow(unused_variables)]
fn output(props: &OutputProperties) -> Html {
    let onmethodchange = Callback::<Event, _>::from({
        let onchange = props.onchange.clone();
        let parse_method = props.parse_method.clone();
        move |ev: Event| {
            let sel = ev
                .target()
                .unwrap()
                .dyn_into::<HtmlSelectElement>()
                .unwrap();
            let sel = match sel.value().as_str() {
                "TokenStream" => ParseMethod::TokenStream,
                "Item" => ParseMethod::Item,
                _ => ParseMethod::Unknown,
            };
            parse_method.set(sel);
        }
    });
    html! {
        <>
            <div class={ classes!("text-md", "my-2") }>
                <span class={ classes!("font-mono", "bg-slate-100", "px-1", "rounded-sm") }>{"syn::parse::<"}</span>
                <select onchange={onmethodchange} class={ classes!("font-mono", "text-center", "appearance-none", "px-0.5") }>
                    <option value="TokenStream">{ "TokenStream" }</option>
                    <option value="Item" selected=true>{ "syn::Item" }</option>
                </select>
                <span class={ classes!("font-mono", "bg-slate-100", "px-1", "rounded-sm") }>{">"}</span>
                { " output" }</div>
            <textarea readonly=true value={(*props.output).clone()} class={
                classes!(
                    "w-full",
                    "flex-1",
                    "px-2",
                    "pt-2",
                    "text-justify",
                    "text-start",
                    "font-mono",
                    "bg-slate-100",
                    "border-solid",
                    "border-2",
                    "border-slate-800",
                    "rounded-md"
                )
            }></textarea>
        </>
    }
}
