use dioxus::prelude::*;

use crate::api::{Difficulty, Tower};
use chrono::Local;

#[component]
pub fn DataTable(caption: String, towers: Vec<Tower>) -> Element {
    let beat_all = if towers.iter().all(|t| t.time.is_some()) {
        "beaten_true"
    } else {
        ""
    };
    
    rsx! {
        table {
            caption {
                class: beat_all,
                b { "{caption}" }
            }
            tr {
                th { "Tower Name" }
                th { "Tower Difficulty" }
            }
            for tower in towers {
                tr {
                    key: "{caption}{tower.acronym}",
                    td { class: "beaten_{tower.time.is_some()}", span { {tower.acronym} } }
                    td { class: "{Difficulty::new(tower.difficulty).unwrap().to_string()}_bg", span { "{tower.difficulty:.2}" } }
                }
            }
        }
    }
}

#[component]
pub fn ClearTimeTable(towers: Vec<Tower>) -> Element {
    let mut towers: Vec<Tower> = towers.into_iter().filter(|t| t.time.is_some()).collect();
    towers.sort_by_key(|t| t.time.unwrap());

    let format_string = "%y-%m-%d %T";
    let local = Local {};

    rsx! {
        table {
            caption { b { "Tower Completions" } }
            tr {
                th { "Compl#" }
                th { "Tower Name" }
                th { "Tower Difficulty" }
                th { "Realm" }
                th { "Date Completed" }
            }
            for (i, tower) in towers.into_iter().enumerate() {
                tr {
                    key: "completion{tower.acronym}",
                    td { "{i + 1}" }
                    td { span { {tower.acronym} } }
                    td { class: "{Difficulty::new(tower.difficulty).unwrap().to_string()}_bg", span { "{tower.difficulty:.2}" } }
                    td { "{tower.realm}" }
                    td { "{tower.time.unwrap().and_utc().with_timezone(&local).format(format_string)}" }
                }
            }
        }
    }
}
