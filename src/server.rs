use std::{collections::BTreeMap, sync::LazyLock};

use crate::api::{
    BadgeAward, BadgeAwardList, Difficulty, DifficultyRequirement, PlayerDataList, Realm, Tower,
};
use dioxus::{logger::tracing::info, prelude::ServerFnError};
use reqwest::{header, Client};
use sqlx::SqlitePool;

const CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .default_headers(header::HeaderMap::from_iter([(
            header::HeaderName::from_static("x-api-key"),
            header::HeaderValue::from_static(include_str!("../api_key")),
        )]))
        .build()
        .unwrap()
});

static POOL: LazyLock<SqlitePool> =
    LazyLock::new(|| SqlitePool::connect_lazy("sqlite://etoh.db").unwrap());

pub async fn get_tower_completions(user_id: i64) -> Result<Vec<Tower>, ServerFnError> {
    // get all uncompleted towers
    let to_check = sqlx::query!(
        "SELECT t.badge_id FROM Tower t WHERE NOT EXISTS (
         SELECT 1 FROM TowerCompletion c WHERE t.badge_id = c.badge_id AND user_id = ?)",
        user_id
    )
    .fetch_all(&*POOL)
    .await?;

    if !to_check.is_empty() {
        let stringified = to_check
            .iter()
            .map(|rec| rec.badge_id.to_string())
            .collect::<Vec<_>>();

        let badges = call_for_badges(stringified, user_id).await?;

        if !badges.is_empty() {
            // if we found new badges we have to check the legacy badges to get the earliest time
            let mut legacy_ids = Vec::new();
            let mut id_map = BTreeMap::new();
            for badge in &badges {
                let tower = sqlx::query!(
                    "SELECT t.legacy_badge_id FROM Tower t WHERE t.badge_id = ?",
                    badge.badge_id
                )
                .fetch_one(&*POOL)
                .await?;
                if let Some(legacy_id) = tower.legacy_badge_id {
                    legacy_ids.push(legacy_id);
                    id_map.insert(legacy_id, badge.badge_id);
                }
            }

            let legacy_stringified = legacy_ids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>();

            let legacy_badges = call_for_badges(legacy_stringified, user_id).await?;

            for badge in legacy_badges {
                sqlx::query!(
                    "INSERT INTO TowerCompletion VALUES (?, ?, ?)",
                    user_id,
                    *id_map.get(&badge.badge_id).unwrap(),
                    badge.awarded_date
                )
                .execute(&*POOL)
                .await?;
            }

            for badge in badges {
                sqlx::query!(
                    "INSERT OR IGNORE INTO TowerCompletion VALUES (?, ?, ?)",
                    user_id,
                    badge.badge_id,
                    badge.awarded_date
                )
                .execute(&*POOL)
                .await?;
            }
        }
    }

    let towers = sqlx::query_as!(Tower,
        "SELECT t.full_name, t.acronym, t.difficulty, t.realm, c.time FROM Tower t LEFT JOIN TowerCompletion c ON t.badge_id = c.badge_id AND user_id = ?",
        user_id
    ).fetch_all(&*POOL).await?;

    Ok(towers)
}

async fn call_for_badges(
    to_check: Vec<String>,
    user_id: i64,
) -> Result<Vec<BadgeAward>, ServerFnError> {
    let mut res = Vec::new();
    for chunk in to_check.chunks(100) {
        let stringified = chunk.join(",");

        let response = CLIENT.get(format!("https://badges.roblox.com/v1/users/{user_id}/badges/awarded-dates?badgeIds={stringified}")).send().await?.text().await?;
        let badges = serde_json::from_str::<BadgeAwardList>(&response)?;
        res.extend(badges.data);
    }
    Ok(res)
}

pub async fn get_user_id(name: &str) -> Result<i64, ServerFnError> {
    let name = name.trim().trim_start_matches('@');
    if let Some(id) = sqlx::query!("SELECT id FROM User u WHERE u.name = ?", name)
        .fetch_optional(&*POOL)
        .await?
    {
        return Ok(id.id);
    }

    let response = CLIENT
        .post("https://users.roblox.com/v1/usernames/users")
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"usernames\":[\"{name}\"],\"excludeBannedUsers\":false}}"
        ))
        .send()
        .await?
        .text()
        .await?;

    info!("res: {response}");

    let players = serde_json::from_str::<PlayerDataList>(&response)?;
    for player in players.data {
        info!("{player:?}");
        sqlx::query!("INSERT INTO User VALUES (?, ?)", player.id, player.name)
            .execute(&*POOL)
            .await?;
        return Ok(player.id);
    }

    Err(ServerFnError::ServerError("Couldn't get user id".into()))
}

pub async fn get_realms() -> Result<Vec<Realm>, ServerFnError> {
    let realms = sqlx::query!("SELECT * FROM Realm")
        .fetch_all(&*POOL)
        .await?;
    let diff_reqs = sqlx::query!("SELECT * FROM DifficultyRequirement")
        .fetch_all(&*POOL)
        .await?;
    let map: BTreeMap<i64, DifficultyRequirement> = diff_reqs
        .into_iter()
        .map(|rec| {
            (
                rec.id,
                DifficultyRequirement {
                    difficulty: rec.difficulty.parse::<Difficulty>().unwrap(),
                    amount: rec.amount,
                },
            )
        })
        .collect();

    Ok(realms
        .into_iter()
        .map(|rec| Realm {
            name: rec.name,
            tower_points: rec.tower_points,
            difficulty_requirent1: rec
                .difficulty_requirement1
                .and_then(|r| map.get(&r).cloned()),
            difficulty_requirent2: rec
                .difficulty_requirement2
                .and_then(|r| map.get(&r).cloned()),
        })
        .collect())
}
