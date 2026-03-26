use std::sync::OnceLock;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options islands=true islands_router=true />
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-islands-routing-test.css"/>
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=HomePage/>
                    <Route path=StaticSegment("/other") view=OtherPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1 class="p-6">"Homepage"</h1>
        <div class="p-6">
            <a href="/other" class="p-6">go to other page</a>
            <ChangeValueOne />
            <ChangeValueTwo />
            <Sum />
        </div>
    }
}

#[island]
fn Sum() -> impl IntoView {
    let number_one_signal = GlobalState::get().first_number;
    let number_two_signal = GlobalState::get().second_number;

    let sum = LocalResource::new(move || async move {
        let number_one = number_one_signal();
        let number_two = number_two_signal();

        let value_one = match number_one.is_enabled {
            true => number_one.value,
            false => 0
        };

        let value_two = match number_two.is_enabled {
            true => number_two.value,
            false => 0
        };

        let sum = value_one + value_two;

        sum
    });

    view! {
        <div>
            The sum is: {move || sum.get().unwrap_or(0)}
        </div>
    }
}

#[island]
fn ChangeValueOne() -> impl IntoView {
    let number_one_signal = GlobalState::get().first_number;

    let on_input = move |_| {
        let number_one = number_one_signal();
        let is_enabled = !number_one.is_enabled;

        number_one_signal(Number {
            is_enabled,
            ..number_one
        })
    };

    let on_toggle = move || number_one_signal().is_enabled;

    view! {
        <div class="p-8">
            <label>
                Change value one:
                <input on:input=on_input prop:checked=on_toggle type="checkbox" class="m-2" />
            </label>
        </div>
    }
}

#[island]
fn ChangeValueTwo() -> impl IntoView {
    let number_two_signal = GlobalState::get().second_number;

    let on_input = move |_| {
        let number_two = number_two_signal();
        let is_enabled = !number_two.is_enabled;

        number_two_signal(Number {
            is_enabled,
            ..number_two
        })
    };

    let on_toggle = move || number_two_signal().is_enabled;

    view! {
        <div class="p-8">
            <label>
                Change value two:
                <input on:input=on_input prop:checked=on_toggle type="checkbox" class="m-2" />
            </label>
        </div>
    }
}

#[component]
fn OtherPage() -> impl IntoView {
    view! {
        <h1 class="p-6">"The other page"</h1>
        <a href="/" class="p-6">go back to home</a>
        <OtherIsland />
    }
}

#[island]
fn OtherIsland() -> impl IntoView {
    let first_number = GlobalState::get().first_number;
    let second_number = GlobalState::get().second_number;
    let number = RwSignal::new(0);
    let number_enabled = RwSignal::new(false);

    let on_click = move |_| {
        number(second_number().value);
        number_enabled(second_number().is_enabled);
    };

    view! {
        <div>
            <div>
                First number from homepage: {first_number().value}
            </div>
            <div>
                First number enabled: {first_number().is_enabled}
            </div>
            <div class="pt-3">
                Second number from homepage shown on click: {number}
            </div>
            <div>
                Second number enabled shown on click: {number_enabled}
            </div>
            <button on:click=on_click class="mt-3 p-4 border">Show second number</button>
        </div>
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Number {
    pub is_enabled: bool,
    pub value: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub first_number: RwSignal<Number>,
    pub second_number: RwSignal<Number>,
}

impl GlobalState {
    pub fn get() -> Self {
        static STATE: OnceLock<GlobalState> = OnceLock::new();

        #[cfg(feature = "ssr")] {
            return Self {
                first_number: RwSignal::new(Number { is_enabled: false, value: 1 }),
                second_number: RwSignal::new(Number { is_enabled: false, value: 2 }),
            };
        }

        #[cfg(feature = "hydrate")] {
            *STATE.get_or_init(|| {
                Self {
                    first_number: RwSignal::new(Number { is_enabled: false, value: 1 }),
                    second_number: RwSignal::new(Number { is_enabled: false, value: 2 }),
                }
            })
        }
    }
}