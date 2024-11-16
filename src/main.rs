use std::{thread, time::Duration};

use redis::Commands;
trait RedisTrait {
    fn connect(client_options:RedicClientOptions) -> Result<Redis, Box<dyn std::error::Error>>;
}

pub struct RedicClientOptions {
    pub connection_address:String,
    pub port:i64
}

pub struct Redis {
    pub client:redis::Client
}

impl RedisTrait for Redis {
    fn connect(client_options:RedicClientOptions) -> Result<Redis, Box<dyn std::error::Error>> {
        let redis_url:String = format!("redis://:@{}:{}",client_options.connection_address, client_options.port);
        match redis::Client::open(redis_url.as_str()) {
            Ok(client) => Ok(Redis{client}),
            Err(e) => Err(Box::new(e)),
        }
    }
    
}
fn main() {
    dotenv::dotenv().ok();
    let redis_address = std::env::var("REDIS_ADDRESS").unwrap();
    let redis_port = std::env::var("REDIS_PORT").unwrap().parse::<i64>().unwrap();
    let redis_config = RedicClientOptions { connection_address:redis_address.to_string(), port:redis_port} ;
    let channel = "test";
    match Redis::connect(redis_config) {
        Ok(client) => {
            match client.client.get_connection() {
                Ok(mut conn) => {
                    
                    
                    for i in  0..10 {
                        let msg = format!("this is test {:?} ",i);
                        match conn.publish::<&str, &str,()>(channel, &msg)  {
                            Ok(_) => {
                                println!("message has been send {:?}", msg)
                            },
                            Err(e) => {
                                println!("Publish message error {:?}", e.to_string())
                            },
                        } 

                        thread::sleep(Duration::from_secs(1));
                    }
                    

                },
                Err(e) => {
                    println!("Error while getting redis connection {:?}", e.to_string())
                },
            }
        },
        Err(e) => {
            println!("Redis connection error {:?}", e.to_string())
        },
    }

}
