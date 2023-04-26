// pub async fn create_db_if_required(state: Data<AppState>) {
//     let schema_exists =
//         sqlx::query("SELECT schema_name FROM information_schema.schemata WHERE schema_name = 'db'")
//             .fetch_optional(&conn)
//             .await?
//             .is_some();
//     if !schema_exists {


//         sqlx::query(""
//         )
//         .execute(&pool)
//         .await?;

//         // create schema
//     }
// }
