use leptos::{component, view, For, IntoView};

use crate::services::static_assets::Book;

#[component]
pub fn BookCard(#[prop(into)] book: Book) -> impl IntoView {
    view! {
        <li class="flex flex-col items-center justify-evenly space-y-4">
            <figure class="w-2/4">
                <img src={book.cover} alt={format!("{} cover", book.name)} />
            </figure>
            <article class="flex flex-col justify-center">
                <span class="text-center">{book.name}</span>
                <span class="text-center">{book.author}</span>
            </article>
            <ul class="flex flex-wrap gap-2">
                <For
                    key=|b: &String| b.clone()
                    each=move || book.tags.clone()
                    children=move |tag: String| {
                        view! {
                            <li class="font-mono border py-0.5 px-1 text-xs">
                                {tag}
                            </li>
                        }
                    }
                />
            </ul>
        </li>
    }
}
