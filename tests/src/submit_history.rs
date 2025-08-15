use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use entity::{
    Matched, PuzzleDate, PuzzleSolution, SubmitLetter, SubmitWord,
    histories::{self, Entity as Histories},
    puzzles, sessions,
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, TransactionTrait};

#[tokio::test]
async fn test() {
    let db = crate::setup().await;
    let tran = db.begin().await.unwrap();
    seed_data(&db).await;

    let date = PuzzleDate::try_from("1970-01-01").unwrap();
    let solution = PuzzleSolution::try_from("rusty").unwrap();
    let session = "session".to_owned();

    let history = Histories::find_by_id((date.clone(), session.clone()))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(history.date, date);
    assert_eq!(history.session, session);
    assert_eq!(history.original_solution, solution);
    assert!(!history.is_dirty);

    let mut submit_history = history.submit_history.unwrap_or_default();
    submit_history
        .submit(SubmitWord::new([
            SubmitLetter::new('R', Matched::Yes),
            SubmitLetter::new('U', Matched::No),
            SubmitLetter::new('S', Matched::Partially),
            SubmitLetter::new('T', Matched::Yes),
            SubmitLetter::new('Y', Matched::Partially),
        ]))
        .unwrap();

    let active_history = histories::ActiveModel {
        date: ActiveValue::Unchanged(date),
        session: ActiveValue::Unchanged(session),
        submit_history: ActiveValue::Set(Some(submit_history)),
        ..Default::default()
    };
    active_history.update(&db).await.unwrap();

    tran.rollback().await.unwrap();
}

async fn seed_data(db: &DatabaseConnection) {
    let date = PuzzleDate::try_from("1970-01-01").unwrap();
    let solution = PuzzleSolution::try_from("rusty").unwrap();
    let session = "session".to_owned();
    let date_time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );

    let active_puzzle = puzzles::ActiveModel {
        date: ActiveValue::Set(date.clone()),
        solution: ActiveValue::Set(solution.clone()),
        ..Default::default()
    };

    let active_session = sessions::ActiveModel {
        session: ActiveValue::Set(session.clone()),
        created_at: ActiveValue::Set(date_time),
        updated_at: ActiveValue::Set(date_time),
    };

    let active_history = histories::ActiveModel {
        date: ActiveValue::Set(date.clone()),
        session: ActiveValue::Set(session.clone()),
        submit_history: ActiveValue::Set(None),
        original_solution: ActiveValue::Set(solution.to_owned()),
        uploaded_at: ActiveValue::Set(date_time),
        ..Default::default()
    };

    active_puzzle.insert(db).await.unwrap();
    active_session.insert(db).await.unwrap();
    active_history.insert(db).await.unwrap();
}
