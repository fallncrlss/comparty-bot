use redis;

pub async fn new_redis_client() -> redis::Client {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL env var needs to be set");
    redis::Client::open(redis_url).expect("Unable to connect to Redis")
}