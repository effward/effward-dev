pub async fn insert(
    pool: &MySqlPool,
    name: &String,
    email: &String,
    password: &Secret<String>,
) -> Result<u64, EntityError> {
    
}