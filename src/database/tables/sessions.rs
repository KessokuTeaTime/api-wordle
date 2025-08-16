//! Table `sessions`.

use chrono::Utc;
use entity::{prelude::*, sessions};
use migration::OnConflict;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use tracing::{error, info};

pub async fn insert_or_update_session(db: &DatabaseConnection, session: &str) -> Result<(), DbErr> {
    info!("inserting or updating session {session}…");
    let now = Utc::now().naive_utc();
    let active_session = sessions::ActiveModel {
        session: ActiveValue::Set(session.to_owned()),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    };

    match Sessions::insert(active_session)
        .on_conflict(
            OnConflict::column(sessions::Column::Session)
                .update_column(sessions::Column::UpdatedAt)
                .to_owned(),
        )
        .exec(db)
        .await
    {
        Ok(_) => {
            info!("inserted or updated session {session} at {now}");
            Ok(())
        }
        Err(err) => {
            error!("failed to insert or update session {session}: {err}");
            Err(err)
        }
    }
}

pub async fn delete_session(db: &DatabaseConnection, session: &str) -> Result<(), DbErr> {
    match Sessions::delete_by_id(session.to_owned()).exec(db).await {
        Ok(_) => {
            info!("deleted session {session}");
            Ok(())
        }
        Err(err) => {
            error!("failed to delete session {session}: {err}");
            Err(err)
        }
    }
}
