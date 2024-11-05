use std::ffi::OsStr;
use std::fs::{read_dir, remove_file};
use std::io::{BufWriter, Write};

use anyhow::Context;
use axum::async_trait;
use maud::html;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::db::product::Product;
use crate::handlers::frontend_builder::PageBuilder;

use super::product::ProductMarker;
use super::ProductDatabaseHandler;


#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Story {
    pub id: i64,
    pub language: String,
    pub pdf: Vec<u8>,
    pub epub: Vec<u8>,
    pub pid: i64,
}

impl ProductMarker for Story {
    fn product_id(&self) -> i64 {
        self.pid
    }
}

pub async fn synchronize_story_files(pool: &SqlitePool) -> Result<(), anyhow::Error> {
    for entry in read_dir("static").with_context(|| "Failed to read directory")? {
        let path = entry.with_context(|| "Failed to get entry")?.path();
        if path.extension().is_some_and(|ext| ext == OsStr::new("pdf") || ext == OsStr::new("epub")) {
            remove_file(path).with_context(|| "Failed to remove file")?;
        }
    }
    let results = sqlx::query!("SELECT p.name, s.pdf, s.epub FROM story s INNER JOIN product p ON s.pid = p.id").fetch_all(pool).await?;
    for result in results.iter() {
        let pdf_filename = format_filepath(&result.name, "pdf");
        let file = std::fs::File::create_new(&pdf_filename).with_context(|| format!("Failed to create {}", &pdf_filename))?;
        BufWriter::new(file).write_all(&result.pdf.to_vec()).with_context(|| format!("Failed to write {}", &pdf_filename))?;

        let epub_filename = format_filepath(&result.name, "epub");
        let file = std::fs::File::create_new(&epub_filename).with_context(|| format!("Failed to create {}", &epub_filename))?;
        BufWriter::new(file).write_all(&result.epub.to_vec()).with_context(|| format!("Failed to write {}", &epub_filename))?;
    }
    Ok(())
}

#[async_trait]
impl ProductDatabaseHandler for Story {
    async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, AppError>
    where
        Self: Sized,
    {
        Ok(sqlx::query_as!(Story, "SELECT * FROM story")
            .fetch_all(pool)
            .await?)
    }

    async fn post(&self, pool: &SqlitePool, name: String, description: String) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let time = std::time::SystemTime::now();
        let uploaddate = time.duration_since(std::time::UNIX_EPOCH)?.as_millis() as i64;

        let mut tx = pool.begin().await?;
        let product = sqlx::query_as!(
            Product, 
            "INSERT INTO product (name, description, uploaddate, updatedate) VALUES ($1, $2, $3, $4) RETURNING id, name, description, uploaddate, updatedate", 
            name, 
            description,
            uploaddate,
            uploaddate)
            .fetch_one(&mut *tx)
            .await?;

        let story = sqlx::query_as!(
            Story,
            "INSERT INTO story (language, pdf, epub, pid) VALUES ($1, $2, $3, $4) RETURNING id, language, pdf, epub, pid", 
            self.language,
            self.pdf,
            self.epub,
            product.id)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(story)
    }
}

impl PageBuilder for Story {
   fn page_title() -> String {
        "Stories".to_string()
   } 

   // TODO: Add pdf and epub icons
   fn product_specific_card_content<T: PageBuilder + ProductMarker>
       (&self, specific_product: &super::product::SpecificProduct<T>) -> maud::Markup {
       html!(
           div class="flex justify-start mt-5 text-sm" {
               span class="mr-10" { (&self.language) }
               div class="flex justify-between" {
                   a href=(format_filepath(&specific_product.product.name, "pdf")) { "PDF" }
                   a href=(format_filepath(&specific_product.product.name, "epub")) { "EPUB" }
               }
           }
        )
   }
}

pub fn format_filepath(name: &str, extension: &str) -> String {
    let mut filename = format!("static/{}.{}", name, extension).to_lowercase();
    filename.retain(|c| !c.is_whitespace());
    filename
}

