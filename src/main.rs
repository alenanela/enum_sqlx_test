
#[derive(Debug, PartialEq, sqlx::Type)]
#[sqlx(type_name = "measure_unit")]
enum MeasureUnit {
    g,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {

    dotenv::dotenv().ok();
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    sqlx::query(
        "
        INSERT INTO nutrient (measure_unit, name) 
        VALUES ($1, $2)
        ")
        .bind(MeasureUnit::g)
        .bind("Protein")
        .execute(&db)
        .await?;


    let id: i32 = sqlx::query!(
        "
        INSERT INTO nutrient (measure_unit, name) 
        VALUES ($1, $2)
        RETURNING id
        ",
        MeasureUnit::g,
        "Protein"
    )
    .fetch_one(db)
    .await?
    .id;   

    Ok(())
}