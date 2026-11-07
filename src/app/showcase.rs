use crate::components::prelude::*;
use topcoat::{
    Result,
    router::page,
    view::{View, attributes, view},
};

#[page]
pub async fn showcase() -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-5xl space-y-8 px-4 py-12">
            <header class="space-y-2">
                <h1 class="text-3xl font-bold">"Component library showcase"</h1>
                <p class="text-base-content/70">
                    "The showcase is composed from ServerZ's reusable components."
                </p>
            </header>

            <section class="grid gap-4 md:grid-cols-2">
                card(
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
                card(
                    card_header(card_title("Feedback"))
                    card_content(
                        alert(
                            attrs: attributes! { class="alert-info" },
                            "Information alert"
                        )
                        <div class="flex flex-wrap gap-2">
                            badge(
                                attrs: attributes! { class="badge-primary" },
                                "Primary"
                            )
                            badge(
                                attrs: attributes! { class="badge-success" },
                                "Success"
                            )
                            loading(
                                attrs: attributes! { class="loading-dots loading-md" }
                            )
                        </div>
                    )
                )
                card(
                    card_header(card_title("Form controls"))
                    card_content(
                        input(
                            attrs: attributes! { type="email" placeholder="Email" class="input" }
                        )
                        <div class="flex items-center gap-4">
                            checkbox(attrs: attributes! { class="checkbox-primary" })
                            toggle(attrs: attributes! { class="toggle-success" })
                            range(
                                attrs: attributes! { class="range-primary" min="0" max="100" value="40" }
                            )
                        </div>
                    )
                )
                card(
                    card_header(card_title("Progress"))
                    card_content(
                        progress(
                            value: 65,
                            max: 100,
                            attrs: attributes! { class="progress-primary" }
                        )
                        radial_progress(
                            value: 65,
                            attrs: attributes! { class="text-primary" }
                        )
                    )
                )
                card(
                    card_header(card_title("Navigation"))
                    card_content(
                        <div class="join">
                            button(attrs: attributes! { class="join-item" }, "Previous")
                            button(
                                attrs: attributes! { class="join-item btn-active" },
                                "Current"
                            )
                            button(attrs: attributes! { class="join-item" }, "Next")
                        </div>
                        breadcrumbs(
                            breadcrumb_item(href: "/".to_string(), "Home")
                            <li>"Showcase"</li>
                        )
                    )
                )
            </section>

            <section class="grid gap-4 md:grid-cols-2">
                card(
                    card_header(card_title("Modal and OTP"))
                    card_content(
                        modal(
                            id: "showcase-modal".to_string(),
                            <h3 class="font-bold text-lg">"Reusable modal"</h3>
                            <p class="py-4">
                                "Modal content is supplied by the caller."
                            </p>
                        )
                        otp(attrs: attributes! { maxlength="4" })
                    )
                )
                card(
                    card_header(card_title("Dropdown and accordion"))
                    card_content(
                        dropdown(
                            <summary class="btn">"Open menu"</summary>
                            dropdown_content(
                                <li><a href="/">"Home"</a></li>
                                <li><a href="/todos">"Todos"</a></li>
                            )
                        )
                        accordion_group(
                            name: "showcase-accordion".to_string(),
                            accordion_item(
                                title: "What is ServerZ?".to_string(),
                                group: "showcase-accordion".to_string(),
                                "A Rust component library application."
                            )
                            accordion_item(
                                title: "Does it compose?".to_string(),
                                group: "showcase-accordion".to_string(),
                                "Every component accepts caller content and attributes."
                            )
                        )
                        collapse(
                            collapse_title("More details")
                            collapse_content(
                                "Components keep markup and variants consistent."
                            )
                        )
                    )
                )
                card(
                    card_header(card_title("Carousel and chat"))
                    card_content(
                        carousel(
                            carousel_item(
                                attrs: attributes! { class="bg-primary p-8 text-primary-content" },
                                "Slide one"
                            )
                            carousel_item(
                                attrs: attributes! { class="bg-secondary p-8 text-secondary-content" },
                                "Slide two"
                            )
                        )
                        chat(
                            chat_bubble(
                                attrs: attributes! { class="chat-bubble-info" },
                                "Hello from chat"
                            )
                        )
                        chat(
                            end: true,
                            chat_bubble(
                                attrs: attributes! { class="chat-bubble-primary" },
                                "Reusable chat bubble"
                            )
                        )
                    )
                )
                card(
                    card_header(card_title("Statistics and controls"))
                    card_content(
                        stat(
                            stat_title("Users")
                            stat_value("1.2K")
                            stat_desc("12% this month")
                        )
                        swap(
                            <span class="swap-on">"On"</span>
                            <span class="swap-off">"Off"</span>
                        )
                        rating(attrs: attributes! { class="rating-half" })
                    )
                )
            </section>

            <section class="grid gap-4 md:grid-cols-2">
                card(
                    card_header(card_title("Navigation components"))
                    card_content(
                        drawer(
                            drawer_toggle(id: "showcase-drawer".to_string())
                            drawer_content(
                                drawer_button(
                                    for_id: "showcase-drawer".to_string(),
                                    "Open drawer"
                                )
                            )
                            drawer_side(
                                <nav class="menu min-h-full w-64 bg-base-200 p-4">
                                    <a href="/">"Home"</a>
                                </nav>
                            )
                        )
                        dock(
                            dock_item(active: true, dock_label("Home"))
                            dock_item(active: false, dock_label("Settings"))
                        )
                        steps(
                            <li class="step step-primary">"Start"</li>
                            <li class="step step-primary">"Build"</li>
                            <li class="step">"Ship"</li>
                        )
                        timeline(
                            <li>
                                <div class="timeline-start">"Today"</div>
                                <div class="timeline-end">"Component work"</div>
                            </li>
                        )
                    )
                )
                card(
                    card_header(card_title("Visual components"))
                    card_content(
                        hero(
                            attrs: attributes! { class="min-h-32 bg-primary text-primary-content" },
                            <h2 class="text-2xl font-bold">"Hero"</h2>
                        )
                        mask(
                            attrs: attributes! { class="mask-heart h-24 w-24" },
                            <img
                                src="https://picsum.photos/96/96"
                                alt="Decorative sample"
                            >
                        )
                        fab(<button class="btn btn-primary btn-circle">"+"</button>)
                        tooltip(
                            text: "Helpful information".to_string(),
                            <button class="btn btn-ghost">"Hover me"</button>
                        )
                        list(<li class="list-row">"Reusable list item"</li>)
                        table(
                            <thead>
                                <tr>
                                    <th>"Name"</th>
                                    <th>"Status"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>"ServerZ"</td>
                                    <td>"Ready"</td>
                                </tr>
                            </tbody>
                        )
                    )
                )
            </section>
            card(
                card_header(card_title("Additional library components"))
                card_content(
                    aura(attrs: attributes! { class="aura-rainbow p-4" }, "Aura")
                    avatar(
                        avatar_image(
                            src: "https://picsum.photos/64/64".to_string(),
                            alt: "Sample avatar".to_string()
                        )
                    )
                    join(
                        button("One")
                        button("Two")
                    )
                    menu(<li><a href="/">"Menu item"</a></li>)
                    link(attrs: attributes! { href="/" }, "Reusable link")
                    status(attrs: attributes! { class="status-success" })
                    skeleton(attrs: attributes! { class="h-4 w-32" })
                )
            )
            card(
                card_header(card_title("Typed variants"))
                card_content(
                    alert(color: AlertColor::Warning, "Warning variant")
                    alert_styled(
                        color: AlertColor::Info,
                        style: AlertStyle::Outline,
                        direction: AlertDirection::Horizontal,
                        "Styled alert"
                    )
                    badge(
                        color: BadgeColor::Secondary,
                        size: BadgeSize::Lg,
                        "Large secondary"
                    )
                    badge_styled(
                        color: BadgeColor::Success,
                        size: BadgeSize::Sm,
                        style: BadgeStyle::Soft,
                        "Soft success"
                    )
                    input(
                        color: InputColor::Primary,
                        size: InputSize::Lg,
                        attrs: attributes! { placeholder="Typed input" }
                    )
                    checkbox(color: CheckboxColor::Accent, size: CheckboxSize::Sm)
                    toggle(color: ToggleColor::Success, size: ToggleSize::Lg)
                    loading(style: LoadingStyle::Ring, size: LoadingSize::Sm)
                    radial_progress(
                        value: 80,
                        size: RadialSize::Lg,
                        thickness: RadialThickness::Thick
                    )
                    aura(
                        style: AuraStyle::Rainbow,
                        size: AuraSize::Lg,
                        duration_ms: 1000,
                        "Rainbow aura"
                    )
                )
            )
            footer(<p>"ServerZ component library showcase"</p>)
        </main>
    })
}
