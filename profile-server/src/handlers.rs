use shared::payloads::InCreateUser;
use shared::payloads::OutCreateUser;
use shared::types::User;

use shared::types::UserHandle;
use sqlx::Pool;
use sqlx::Postgres;
use warp::http::StatusCode;

pub async fn create_user(
    payload: InCreateUser,
    pool: Pool<Postgres>,
    me_url: String,
) -> Result<OutCreateUser, (StatusCode, String)> {
    let result = sqlx::query!(
        r#"
    INSERT INTO users (handle, display_name)
    VALUES ($1, $2)
    RETURNING id, handle, display_name, created_at
    "#,
        "",
        payload.name,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let out = OutCreateUser {
        user: User {
            handle: UserHandle {
                url: me_url,
                user_name: result.handle,
            },
            name: result.display_name,
        },
    };
    Ok(out)
}
