use clap::Parser;

const DB_HOST: &'static str = "host.docker.internal";
const DB_PORT: &'static str = "5432";
const DB_NAME: &'static str = "recipe_search";
const DB_USER: &'static str = "postgres";
const DB_PASSWORD: &'static str = "postgres";

const CSV_FILENAME: &'static str = "dataset/full_dataset.csv";

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(format!("postgres://{DB_USER}:{DB_PASSWORD}@{DB_HOST}:{DB_PORT}/{DB_NAME}").as_str())
        .await
        .expect("database connection pool");

    let mut tx = pool.begin().await.expect("begin transaction");

    if args.clear {
        clear(&mut tx).await;
        println!("Cleared table");
    }

    if !args.no_insert {
        let rows_inserted = insert(&mut tx, CSV_FILENAME, args.limit).await;
        println!("Inserted {} rows", rows_inserted);
    }

    tx.commit().await.expect("commit transaction");
}

async fn clear(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query("DELETE FROM recipe;")
        .execute(&mut **tx)
        .await
        .expect("clear table");

    sqlx::query("ALTER SEQUENCE recipe_id_seq RESTART WITH 1;")
        .execute(&mut **tx)
        .await
        .expect("reset sequence");
}

async fn insert(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, csv_filename: &str, limit: Option<u64>) -> u64 {
    let bar = indicatif::ProgressBar::new(limit.unwrap_or(2231142));

    let rows_inserted = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_filename)
        .expect("csv file")
        .records()
        .map(|record| {
            bar.inc(1);
            let record = record.expect("Read record");
            let recipe_raw = RecipeRaw {
                title: record.get(1).expect("title").to_string().trim().to_string(),
                ingredients: serde_json::from_str(record.get(2).expect("ingredients")).expect("ingredients"),
                directions: serde_json::from_str(record.get(3).expect("directions")).expect("directions"),
                link: record.get(4).expect("link").to_string().trim().to_string(),
                source: record.get(5).expect("source").to_string().trim().to_string(),
                ner: serde_json::from_str(record.get(6).expect("ner")).expect("ner"),
            };
            Recipe {
                name: recipe_raw.title.clone(),
                ingredients: recipe_raw.ingredients.clone(),
                instructions: recipe_raw.directions.clone(),
                raw: recipe_raw,
            }
        })
        .map(|recipe| futures::executor::block_on(
            sqlx::query("INSERT INTO recipe (name, ingredients, instructions, raw) VALUES ($1, $2, $3, $4);")
                .bind(recipe.name)
                .bind(serde_pickle::to_vec(&recipe.ingredients, serde_pickle::SerOptions::new()).expect("recipe ingredients"))
                .bind(serde_pickle::to_vec(&recipe.instructions, serde_pickle::SerOptions::new()).expect("recipe instructions"))
                .bind(serde_json::to_string(&recipe.raw).expect("recipe raw"))
                .execute(&mut **tx)
        ))
        .map(|result| result.expect("insert recipe").rows_affected())
        .take(limit.unwrap_or(u64::MAX) as usize)
        .sum::<u64>();

    bar.finish();

    rows_inserted
}

#[derive(Parser)]
#[command(name = "Recipe Search 2m Ingestion")]
#[command(about = "Ingests the 2m recipe dataset into the database")]
struct Args {
    #[clap(short, long)]
    clear: bool,
    #[clap(short, long)]
    no_insert: bool,
    #[clap(short, long)]
    limit: Option<u64>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RecipeRaw {
    title: String,
    ingredients: Vec<String>,
    directions: Vec<String>,
    link: String,
    source: String,
    ner: Vec<String>,
}

struct Recipe {
    name: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
    raw: RecipeRaw
}
