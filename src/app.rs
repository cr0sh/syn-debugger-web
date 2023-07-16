#![allow(non_camel_case_types)]

use gloo_timers::callback::Interval;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let input_ref = use_node_ref();
    let input_value = use_state_eq(String::new);
    let onchange = Callback::<(), _>::from({
        let input_ref = input_ref.clone();
        let input_value = input_value.clone();
        move |_| {
            let ta = input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let value = ta.value();
            let syntax = syn::parse_str::<proc_macro2::TokenStream>(&value);
            match syntax {
                Ok(x) => input_value.set(format!("{x:#?}")),
                Err(e) => input_value.set(format!("{e:#?}")),
            }
        }
    });

    html! {
        <main>
            <div class={ classes!("flex", "flex-col", "items-stretch", "mx-20", "my-10", "h-screen") }>
                <h1 class={ classes!("py-5", "text-xl") }>{ format!("syn-debugger-web v{}", env!("CARGO_PKG_VERSION")) }</h1>
                <div class={ classes!("flex", "flex-row", "grow") }>
                    <div class={ classes!("px-5", "text-md", "w-1/2") }>
                        <Input {input_ref} {onchange}/>
                    </div>
                    <div class={ classes!("px-5", "text-md", "w-1/2") }>
                        <Output output={input_value}/>
                    </div>
                </div>
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
        (),
    );
    html! {
        <>
            <div class={ classes!("text-lg", "my-2") }>{ "Rust code" }</div>
            <textarea ref={props.input_ref.clone()} onchange={move |_| onchange.emit(())} class={
                classes!(
                    "w-full",
                    "h-4/5",
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
}

#[function_component(Output)]
#[allow(unused_variables)]
fn output(props: &OutputProperties) -> Html {
    html! {
        <>
            <div class={ classes!("text-lg", "my-2") }><span class={ classes!("font-mono", "bg-slate-100", "px-1", "rounded-sm") }>{"syn::parse"}</span>{ " output" }</div>
            <textarea readonly=true value={(*props.output).clone()} class={
                classes!(
                    "w-full",
                    "h-4/5",
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
