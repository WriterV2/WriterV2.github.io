use anyhow::{Context, Result};

// resize all work card images to 250 x 250 px
pub fn resize_workcard_images() -> Result<Vec<()>> {
    let mut result: Vec<()> = Vec::new();
    for image_file in std::fs::read_dir("images/workcardimages")? {
        let path = image_file?.path();
        let filename = if let Some(name) = &path.to_str() {
            Ok(format!("docs/{}", name))
        } else {
            Err(anyhow::Error::msg(""))
        };

        let img = image::open(&path)?;
        let processed_image = img.resize(250, 250, image::imageops::FilterType::Triangle);
        let mut output_file = std::fs::File::create(filename?)?;
        result.push(
            processed_image
                .write_to(&mut output_file, image::ImageFormat::Png)
                .with_context(|| format!("Failed to resize work card image from {:?}", path))?,
        );
    }
    Ok(result)
}
