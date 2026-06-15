pub struct UploadService;

impl UploadService {
    pub async fn upload_image(_data: &[u8]) -> anyhow::Result<String> {
        Ok("https://placeholder.url".to_string())
    }
}
