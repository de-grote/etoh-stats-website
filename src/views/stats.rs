use dioxus::prelude::*;

use crate::{
    api::{Difficulty, Realm, Tower},
    components::{ClearTimeTable, DataTable, Scrollable},
    server,
};

const TABLE_CSS: Asset = asset!("/assets/styling/datatable.css");

#[component]
pub fn Stats(name: String) -> Element {
    let towers = use_server_future(move || get_data(name.clone()))?()
        .unwrap()
        .unwrap();
    let realms = use_server_future(move || get_realms())?().unwrap().unwrap();

    rsx! {
        document::Link { rel: "stylesheet", href: TABLE_CSS }

        div {
            for realm in realms {
                DataTable {
                    caption: realm.name.clone(),
                    towers: sorted_by_difficulty(towers.iter().filter(|t| t.realm == realm.name).cloned().collect()),
                }
            }
        }
        div {
            class: "difficulty",
            for diff in (1..=11).flat_map(|x| Difficulty::new(x as f64)) {
                DataTable {
                    caption: diff.to_string(),
                    towers: sorted_by_difficulty(towers.iter().filter(|t| t.difficulty >= diff as u8 as f64 && t.difficulty < diff as u8 as f64 + 1.0).cloned().collect()),
                 }
            }
        }
        Scrollable {
            height: 500,
            ClearTimeTable { towers: towers }
        }
    }
}

fn sorted_by_difficulty(mut v: Vec<Tower>) -> Vec<Tower> {
    v.sort_by_key(|tower| (tower.difficulty * 100.0) as u32);
    v
}

#[server]
async fn get_data(name: String) -> Result<Vec<Tower>, ServerFnError> {
    let user_id = server::get_user_id(&name).await?;

    let stats = server::get_tower_completions(user_id).await?;

    Ok(stats)
}

#[server]
async fn get_realms() -> Result<Vec<Realm>, ServerFnError> {
    server::get_realms().await
}
