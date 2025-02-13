use leptos::{component, create_rw_signal, spawn_local, view, For, IntoView, SignalGet, SignalSet};

use crate::components::organisms::book_card::BookCard;
use crate::services::static_assets::Book;
use crate::services::Services;

#[component]
pub fn Books() -> impl IntoView {
    let books = create_rw_signal::<Vec<Book>>(Vec::default());
    let error = create_rw_signal::<Option<String>>(None);

    spawn_local(async move {
        let services = Services::new();

        match services.assets().fetch_books().await {
            Ok(db) => {
                books.set(db);
            }
            Err(err) => {
                error.set(Some(err.to_string()));
            }
        }
    });

    view! {
        <article class="safe-zone py-4">
            <h2 class="text-xl pb-2 underline">"Rust Books"</h2>
            <p class="text-sm w-2/3">
                r#"Collection of Rust Books for Rust Engineers and Developers willing to start their Rust Career."#
            </p>
        </article>
        <div class="safe-zone py-4">
            <ul class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <For
                    key=|b: &Book| b.isbn_13.clone()
                    each=move || books.get()
                    children=move |book: Book| {
                        view! {
                            <BookCard book={book} />
                        }
                    }
                />
            </ul>
        </div>
    }
}
