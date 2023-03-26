use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

fn init_environment_var(environment_var: &str) -> String {
    return var(environment_var).expect(format!("{} must be set", environment_var).as_str());
}

impl Config {
    pub fn init() -> Config {
        let database_url = init_environment_var("DATABASE_URL");
        let jwt_secret = init_environment_var("JWT_SECRET");
        let jwt_expires_in = init_environment_var("JWT_EXPIRES_IN");
        let jwt_maxage = init_environment_var("JWT_MAXAGE");

        return Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        };
    }
}
