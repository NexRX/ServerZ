use crate::components::*;
use topcoat::{
    Result,
    icon::iconify::iconify_icon,
    icon::*,
    router::page,
    view::{View, attributes, view},
};

#[page]
pub async fn showcase() -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-5xl space-y-8 px-4 py-12">
            <header class="space-y-2">
                <h1 class="text-3xl font-bold">"Component library showcase"</h1>
                <p class="text-base-content/70">"The showcase is composed from ServerZ's reusable components."</p>
            </header>

            <section class="grid gap-4 md:grid-cols-2">
                card::card(
                    card_header(
                        card_title(
                            <span class="iconify solar--wi-fi-bold-duotone"></span>
                            " "
                            "ServerZ"
                        )
                        card_description(
                            "Edit "
                            <code>"src/app.rs"</code>
                            " and save to see your changes."
                        )
                    )
                )
                card::card(
                    card_header(card_title("Feedback"))
                    card_content(
                        alert::alert(attrs: attributes! { class="alert-info" }, "Information alert")
                        <div class="flex flex-wrap gap-2">
                            badge::badge(attrs: attributes! { class="badge-primary" }, "Primary")
                            badge::badge(attrs: attributes! { class="badge-success" }, "Success")
                            loading::loading(attrs: attributes! { class="loading-dots loading-md" })
                        </div>
                    )
                )
                card::card(
                    card_header(card_title("Form controls"))
                    card_content(
                        input::input(attrs: attributes! { type="email" placeholder="Email" class="input" })
                        <div class="flex items-center gap-4">
                            checkbox::checkbox(attrs: attributes! { class="checkbox-primary" })
                            toggle::toggle(attrs: attributes! { class="toggle-success" })
                            range::range(attrs: attributes! { class="range-primary" min="0" max="100" value="40" })
                        </div>
                    )
                )
                card::card(
                    card_header(card_title("Progress"))
                    card_content(
                        progress::progress(value: 65, max: 100, attrs: attributes! { class="progress-primary" })
                        radial_progress::radial_progress(value: 65, attrs: attributes! { class="text-primary" })
                    )
                )
                card::card(
                    card_header(card_title("Navigation"))
                    card_content(
                        <div class="join">
                            button::button(attrs: attributes! { class="join-item" }, "Previous")
                            button::button(attrs: attributes! { class="join-item btn-active" }, "Current")
                            button::button(attrs: attributes! { class="join-item" }, "Next")
                        </div>
                        breadcrumbs::breadcrumbs(breadcrumb_item(href: "/".to_string(), "Home") <li>"Showcase"</li>)
                    )
                )
            </section>

            <section class="grid gap-4 md:grid-cols-2">
                card::card(card_header(card_title("Modal and OTP")) card_content(
                    modal::modal(id: "showcase-modal".to_string(),
                        <h3 class="font-bold text-lg">"Reusable modal"</h3>
                        <p class="py-4">"Modal content is supplied by the caller."</p>
                    )
                    otp::otp(attrs: attributes! { maxlength="4" })
                ))
                card::card(card_header(card_title("Dropdown and accordion")) card_content(
                    dropdown::dropdown(<summary class="btn">"Open menu"</summary> dropdown_content(<li><a href="/">"Home"</a></li><li><a href="/todos">"Todos"</a></li>))
                    accordion_group(name: "showcase-accordion".to_string(),
                        accordion_item(title: "What is ServerZ?".to_string(), group: "showcase-accordion".to_string(), "A Rust component library application.")
                        accordion_item(title: "Does it compose?".to_string(), group: "showcase-accordion".to_string(), "Every component accepts caller content and attributes.")
                    )
                    collapse::collapse(collapse_title("More details") collapse_content("Components keep markup and variants consistent."))
                ))
                card::card(card_header(card_title("Carousel and chat")) card_content(
                    carousel::carousel(carousel_item(attrs: attributes! { class="bg-primary p-8 text-primary-content" }, "Slide one") carousel_item(attrs: attributes! { class="bg-secondary p-8 text-secondary-content" }, "Slide two"))
                    chat::chat(chat_bubble(attrs: attributes! { class="chat-bubble-info" }, "Hello from chat"))
                    chat::chat(end: true, chat_bubble(attrs: attributes! { class="chat-bubble-primary" }, "Reusable chat bubble"))
                ))
                card::card(card_header(card_title("Statistics and controls")) card_content(
                    stat::stat(stat_title("Users") stat_value("1.2K") stat_desc("12% this month"))
                    swap::swap(<span class="swap-on">"On"</span> <span class="swap-off">"Off"</span>)
                    rating::rating(attrs: attributes! { class="rating-half" })
                ))
            </section>

            <section class="grid gap-4 md:grid-cols-2">
                card::card(card_header(card_title("Navigation components")) card_content(
                    drawer::drawer(
                        drawer_toggle(id: "showcase-drawer".to_string())
                        drawer_content(drawer_button(for_id: "showcase-drawer".to_string(), "Open drawer"))
                        drawer_side(<nav class="menu min-h-full w-64 bg-base-200 p-4"><a href="/">"Home"</a></nav>)
                    )
                    dock::dock(dock_item(active: true, dock_label("Home")) dock_item(active: false, dock_label("Settings")))
                    steps::steps(<li class="step step-primary">"Start"</li><li class="step step-primary">"Build"</li><li class="step">"Ship"</li>)
                    timeline::timeline(<li><div class="timeline-start">"Today"</div><div class="timeline-end">"Component work"</div></li>)
                ))
                card::card(card_header(card_title("Visual components")) card_content(
                    hero::hero(attrs: attributes! { class="min-h-32 bg-primary text-primary-content" }, <h2 class="text-2xl font-bold">"Hero"</h2>)
                    mask::mask(attrs: attributes! { class="mask-heart h-24 w-24" }, <img src="https://picsum.photos/96/96" alt="Decorative sample">)
                    fab::fab(<button class="btn btn-primary btn-circle">"+"</button>)
                    tooltip::tooltip(text: "Helpful information".to_string(), <button class="btn btn-ghost">"Hover me"</button>)
                    list::list(<li class="list-row">"Reusable list item"</li>)
                    table::table(<thead><tr><th>"Name"</th><th>"Status"</th></tr></thead><tbody><tr><td>"ServerZ"</td><td>"Ready"</td></tr></tbody>)
                ))
            </section>
            card::card(card_header(card_title("Additional library components")) card_content(
                aura::aura(attrs: attributes! { class="aura-rainbow p-4" }, "Aura")
                avatar::avatar(avatar_image(src: "https://picsum.photos/64/64".to_string(), alt: "Sample avatar".to_string()))
                join::join(button::button("One") button::button("Two"))
                menu::menu(<li><a href="/">"Menu item"</a></li>)
                link::link(attrs: attributes! { href="/" }, "Reusable link")
                status::status(attrs: attributes! { class="status-success" })
                skeleton::skeleton(attrs: attributes! { class="h-4 w-32" })
            ))
            card::card(card_header(card_title("Typed variants")) card_content(
                alert::alert(color: ComponentColor::Warning, "Warning variant")
                alert_styled(color: ComponentColor::Info, style: ComponentStyle::Outline, direction: ComponentDirection::Horizontal, "Styled alert")
                badge::badge(color: ComponentColor::Secondary, size: ComponentSize::Lg, "Large secondary")
                badge_styled(color: ComponentColor::Success, size: ComponentSize::Sm, style: ComponentStyle::Soft, "Soft success")
                input::input(color: ComponentColor::Primary, size: ComponentSize::Lg, attrs: attributes! { placeholder="Typed input" })
                checkbox::checkbox(color: ComponentColor::Accent, size: ComponentSize::Sm)
                toggle::toggle(color: ComponentColor::Success, size: ComponentSize::Lg)
                loading::loading(style: LoadingStyle::Ring, size: ComponentSize::Sm)
                radial_progress::radial_progress(value: 80, size: RadialSize::Lg, thickness: RadialThickness::Thick)
                aura::aura(style: AuraStyle::Rainbow, size: AuraSize::Lg, duration_ms: 1000, "Rainbow aura")
            ))
            footer::footer(<p>"ServerZ component library showcase"</p>)
        </main>
    })
}
