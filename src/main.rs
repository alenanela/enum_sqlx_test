#[derive(Debug, PartialEq, sqlx::Type)]
#[sqlx(type_name = "measure_unit", rename_all = "snake_case")]
enum MeasureUnit {
    G,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    dotenv::dotenv().ok();
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let id: i32 = sqlx::query!(
        "
        INSERT INTO nutrient (measure_unit, name)
        VALUES ($1, $2)
        RETURNING id
        ",
        MeasureUnit::G as MeasureUnit,
        "Protein"
    )
    .fetch_one(&db)
    .await?
    .id;

    dbg!(id);

    Ok(())
}