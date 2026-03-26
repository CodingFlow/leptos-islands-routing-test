use std::sync::OnceLock;

use leptos::{prelude::*, reactive::signal};
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
    let sum = move || {
        let number_one = (GlobalState::get().first_number)();
        let number_two = (GlobalState::get().second_number)();

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
    };

    let (local_sum, set_local_sum) = signal::<i32>(0);

    #[cfg(feature = "hydrate")] {
        Effect::new(move |_| {
            let sum_value = sum();
            leptos::logging::log!("sum is: {sum_value}");

            set_local_sum(sum_value);
        });
    }

    view! {
        <div>
            The sum is: {local_sum}
        </div>
    }
}

#[island]
fn ChangeValueOne() -> impl IntoView {
    let on_input = move |_| {
        let number_one_signal = GlobalState::get().first_number;
        let number_one = number_one_signal();
        let is_enabled = !number_one.is_enabled;

        number_one_signal(Number {
            is_enabled,
            ..number_one
        })
    };

    view! {
        <div class="p-8">
            <label>
                Change value one:
                <input on:input=on_input type="checkbox" class="m-2" />
            </label>
        </div>
    }
}

#[island]
fn ChangeValueTwo() -> impl IntoView {
    let on_input = move |_| {
        let number_two_signal = GlobalState::get().second_number;
        let number_two = number_two_signal();
        let is_enabled = !number_two.is_enabled;

        number_two_signal(Number {
            is_enabled,
            ..number_two
        })
    };

    view! {
        <div class="p-8">
            <label>
                Change value two:
                <input on:input=on_input type="checkbox" class="m-2" />
            </label>
        </div>
    }
}

#[component]
fn OtherPage() -> impl IntoView {
    view! {
        <h1 class="p-6">"The other page"</h1>
        <a href="/" class="p-6">go back to home</a>
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Number {
    pub is_enabled: bool,
    pub value: i32,
}

#[derive(Clone, Debug)] // ArcRwSignal is Clone, not Copy
pub struct GlobalState {
    pub first_number: ArcRwSignal<Number>,
    pub second_number: ArcRwSignal<Number>,
}

impl GlobalState {
    pub fn get() -> Self {
        static STATE: OnceLock<GlobalState> = OnceLock::new();
        
        // This works on both Server and Client. 
        // ArcRwSignal does not require a reactive Owner to exist.
        STATE.get_or_init(|| {
            Self {
                first_number: ArcRwSignal::new(Number { is_enabled: false, value: 1 }),
                second_number: ArcRwSignal::new(Number { is_enabled: false, value: 2 }),
            }
        }).clone()
    }
}