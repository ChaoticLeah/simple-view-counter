//Make a hashmap that stores string ips and timestamps. If the ip is in the hashmap, check if the timestamp is older than 12 hours. If it is, or it doesnt exist in the hashmap set the time to now, and return true
//If the timestamp is less than 12 hours, return false
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Cooldown {
    cooldowns: HashMap<String, u64>,
    hours: u64,
    blacklist: Vec<String>,
}

impl Cooldown {
    pub fn new(hours: u64, blacklist: Vec<String>) -> Self {
        Cooldown {
            cooldowns: HashMap::new(),
            hours,
            blacklist,
        }
    }

    pub fn check(&mut self, ip: &str) -> bool {
        if self.blacklist.contains(&ip.to_string()) {
            return false;
        }
        
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        match self.cooldowns.get_mut(ip) {
            Some(time) => {
                // Calculate elapsed time in hours
                let elapsed_hours = (now - *time) / 3600;
                
                if elapsed_hours >= self.hours {
                    *time = now;
                    true
                } else {
                    false
                }
            }
            None => {
                self.cooldowns.insert(ip.to_string(), now);
                true
            }
        }
    }
    
}