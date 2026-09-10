use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use server_shared::routes::Context;
use shared::endpoints::GetUserParams;
use shared::endpoints::NoParams;
use shared::payloads::InCreateUser;
use shared::payloads::NoPayload;
use shared::payloads::OutCreateUser;
use shared::payloads::OutGetUser;
use shared::types::User;

use shared::types::UserHandle;

pub async fn create_user(
    context: Context,
    _: NoParams,
    payload: InCreateUser,
) -> Result<OutCreateUser, String> {
    /*let result = sqlx::query!(
            r#"
    INSERT INTO users (handle, display_name)
    VALUES ($1, $2)
    RETURNING id, handle, display_name, created_at
    "#,
            "",
            payload.name,
        )
        .fetch_one(&context.pool)
        .await
        .unwrap();
    */
    let out = OutCreateUser {
        user: User {
            handle: UserHandle {
                url: context.me_url,
                user_name: UserHandle::new_random().user_name,
            },
            name: String::from("display_name"),
        },
    };
    Ok(out)
}

pub async fn get_user(
    context: Context,
    _: GetUserParams,
    payload: NoPayload,
) -> Result<OutGetUser, String> {
    let out = OutGetUser {
        user: User {
            handle: UserHandle {
                url: context.me_url,
                user_name: UserHandle::new_random().user_name,
            },
            name: String::from("display_name"),
        },
    };
    Ok(out)
}

pub async fn get_test(
    context: Context,
    _: NoParams,
    payload: NoPayload,
) -> Result<NoPayload, String> {
    Ok(NoPayload)
}
