//! Web interface templates.

use crate::measurement;

// Base page.
markup::define! {
    Base(body: Box<std::fmt::Display>) {
        {markup::doctype()}
        html[lang = "en"] {
            head {
                title { "My IoT" }
                meta[charset = "utf-8"];
                meta[name = "viewport", content = "width=device-width, initial-scale=1"];
                meta["http-equiv" = "refresh", content = "60"];
                link[rel = "stylesheet", href = "https://cdnjs.cloudflare.com/ajax/libs/bulma/0.7.4/css/bulma.min.css"];
                script[src = "https://kit.fontawesome.com/e88ef3598d.js"] {}
                script[src = "https://cdn.plot.ly/plotly-1.5.0.min.js"] {}
                style { ".measurement { height: 100% }" }
            }
            body {
                {markup::raw(body)}
                footer.footer {
                    div.container {
                        div.columns {
                            div.column."is-4" {
                                p {
                                    i.fas."fa-circle"."has-text-info" {} " "
                                    a[href = "https://github.com/eigenein/my-iot-rs"] {
                                        strong { "My IoT " {clap::crate_version!()} }
                                    }
                                    " by " a[href = "https://github.com/eigenein"] { strong { "eigenein" } }
                                }
                                p {
                                    i.fas."fa-certificate"."has-text-primary" {} " "
                                    "Made with " a[href = "https://bulma.io/"] { strong { "Bulma" } }
                                }
                                p {
                                    i.fab."fa-fort-awesome"."has-text-success" {} " "
                                    "Icons by " a[href = "https://fontawesome.com/"] { strong { "Font Awesome" } }
                                }
                            }
                        }
                    }
                }
                script {
                    {markup::raw(r#"
                        document.addEventListener('DOMContentLoaded', () => {
                            const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);
                            if ($navbarBurgers.length > 0) {
                                $navbarBurgers.forEach(el => {
                                    el.addEventListener('click', () => {
                                        const $target = document.getElementById(el.dataset.target);
                                        el.classList.toggle('is-active');
                                        $target.classList.toggle('is-active');
                                    });
                                });
                            }
                        });
                    "#)}
                }
            }
        }
    }
}

// Shared navigation bar.
markup::define! {
    NavBar {
        nav.navbar[role = "navigation", "aria-label" = "main navigation"] {
            div.container {
                div."navbar-brand" {
                    a."navbar-burger".burger[
                        role = "button",
                        "aria-label" = "menu",
                        "aria-expanded" = "false",
                        "data-target" = "navbar-menu",
                    ] {
                        span["aria-hidden" = "true"] {}
                        span["aria-hidden" = "true"] {}
                        span["aria-hidden" = "true"] {}
                    }
                }

                div#"navbar-menu"."navbar-menu" {
                    div."navbar-start" {
                        a."navbar-item"[href = "/"] {
                            span.icon { i.fas."fa-home" {} }
                            span { "Home" }
                        }
                        a."navbar-item"[href = "/services"] {
                            span.icon { i.fas."fa-cog" {} }
                            span { "Services" }
                        }
                        a."navbar-item"[href = "/sensors"] {
                            span.icon { i.fas."fa-chart-line" {} }
                            span { "Sensors" }
                        }
                        a."navbar-item"[href = "/measurements"] {
                            span.icon { i.fas."fa-ruler-combined" {} }
                            span { "Measurements" }
                        }
                    }

                    div."navbar-end" {
                        a."navbar-item"[href = "https://eigenein.github.io/my-iot-rs/my_iot/"] {
                            span.icon { i.fas."fa-external-link-alt" {} }
                            span { "Docs" }
                        }
                    }
                }
            }
        }
    }
}

// Index page.
markup::define! {
    Index(measurements: Vec<measurement::Measurement>) {
        section.hero."is-info" {
            div."hero-head" { {NavBar {}} }
            div."hero-body" {
                div.container {
                    h1.title."is-4" {
                        "Dashboard"
                    }
                    h2.subtitle."is-6" {
                        {measurements.len()} " sensors"
                    }
                }
            }
        }
        section.section {
            div.container {
                div.columns."is-multiline" {
                    @for measurement in {measurements} {
                        {Measurement { measurement }}
                    }
                }
            }
        }
    }
}

// Sensor.
// TODO: title should be human-readable.
markup::define! {
    Measurement<'a>(measurement: &'a measurement::Measurement) {
        div."column"."is-one-quarter" {
            a[href = {format!("/sensors/{}", &measurement.sensor)} ] {
                div.notification.measurement.{measurement.value.class()} {
                    p.title."is-6"[title = {&measurement.sensor}] {
                        {&measurement.sensor}
                    }
                    p.subtitle."is-7"[title = {&measurement.timestamp.to_string()}] {
                        {&measurement.timestamp.format("%b %d, %H:%M:%S").to_string()}
                    }
                    p."has-text-centered"."has-text-weight-bold"[title = {format!("{:?}", &measurement.value)}] {
                        {&measurement.value}
                    }
                }
            }
        }
    }
}
