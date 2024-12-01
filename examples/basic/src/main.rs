use leptos::*;
use ant_leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Theme>
                <Locale>
                    <div>
                        <h1>"Ant Leptos Demo"</h1>
                        <Version/>
                    </div>
                </Locale>
            </Theme>
        </ConfigProvider>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
