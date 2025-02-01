use leptos::{component, view, IntoView};

#[component]
pub fn About() -> impl IntoView {
    view! {
        <article class="safe-zone py-4">
            <h2 class="text-xl pb-2 underline">"Who we are?"</h2>
            <p class="text-sm w-2/3">
                r#"Rustacean.sh members, centered around the Rust Programming Language, form an inclusive group
                inspired by reliable software development. They come from diverse fields in computer science,
                including systems programming and web development.
                
                Welcoming to all expertise levels, the community offers resources like forums and meetups to
                support learning and collaboration, fostering a culture of inclusivity and high-quality software
                creation."#
            </p>
        </article>
    }
}
