use crate::Route;
use dioxus::{
    html::{strong, th, tr},
    prelude::*,
};

const CALENDAR_CSS: Asset = asset!("/assets/styling/calendar.css");

#[component]
pub fn Calendar(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CALENDAR_CSS}

        div {
            id: "calendar",

            // Content
            h1 { "This is Calendar #{id}!" }
            table {
                tr {
                    th { "Monday" }
                    th { "Tuesday" }
                    th { "Wednesday" }
                    th { "Thursday" }
                    th { "Friday" }
                    th { "Saturday" }
                    th { "Sunday" }
                }

                tr {
                    th {
                        strong { "Family Guy" }
                        p { "S23E05" }
                        p { "01:00 Uhr" }
                        p { "FOX" }
                    }
                    th {
                        strong { "Die Rosenheim-Cops" }
                        p { "S24E23" }
                        p { "01:00 Uhr" }
                        p { "ZDFmediathek" }
                    }
                    th {}
                    th {}
                    th {}
                    th {}
                    th {}
                }

                tr {
                    th {
                        strong { "Married to Medicine" }
                        p { "S11E15" }
                        p { "01:00 Uhr" }
                        p { "Hayu" }
                    }
                    th {
                        strong { "SOKO Köln" }
                        p { "S23E23" }
                        p { "01:00 Uhr" }
                        p { "ZDFmediathek" }
                    }
                    th {}
                    th {}
                    th {}
                    th {}
                    th {}
                }

                tr {
                    th {
                        strong { "SOKO Potsdam" }
                        p { "S07E11" }
                        p { "01:00 Uhr" }
                        p { "ZDFmediathek" }
                    }
                    th {}
                    th {}
                    th {}
                    th {}
                    th {}
                    th {}
                }

                tr {
                    th {
                        strong { "Tracker" }
                        p { "S02E13" }
                        p { "01:00 Uhr" }
                        p { "CBS" }
                    }
                    th {}
                    th {}
                    th {}
                    th {}
                    th {}
                    th {}
                }
            }
        }
    }
}
