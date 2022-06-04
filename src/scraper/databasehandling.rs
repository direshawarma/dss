//use std::borrow::BorrowMut;
//use std::fmt::Display;
//use std::default::default;
use std::time::{SystemTime, UNIX_EPOCH};

//, Result};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use rusqlite::{Connection, params};

#[derive(Debug)]
struct Episode {
    show: String,
    url: String,
    eptitle: String,
    scraped: f64,
    episode: i32,
    season: i32, // Someday Sesame Street might manage to break this limit
}

pub fn db_add(showtitle:&String,title:String,url:String) {
    let dbconn = Connection::open("db.db").unwrap();
    db_create(&dbconn); // if necessary


    lazy_static! {
        static ref SEASONREGEX:Regex = Regex::new(r"[Ss]eason:? (\d+)").unwrap();
        static ref EPISODEREGEX:Regex = Regex::new(r"[Ee]pisode:? (\d+)").unwrap();
        static ref SPACEKILLER:Regex = Regex::new(r" +").unwrap();
        static ref BEGINFIXER:Regex = Regex::new(r"^.+?(\w)").unwrap();
    }
    ;

    //entry.eptitle = SEASONREGEX.replace(&entry.eptitle.to_string(), "$before $after").to_string();
    /*

*/
    let string = format!("{} {}",r"Watch", &showtitle);
    let title_killer:Regex = Regex::new(&string).unwrap();

    let mut entry = Episode {
        show: showtitle.to_string(),
        url: url.to_string(),
        eptitle: title.to_string(),
        scraped: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
        episode: 0,
        season: 1
    };
    entry.eptitle = title_killer.replace(&entry.eptitle, "$after").to_string();

    for cap in SEASONREGEX.captures_iter(&entry.eptitle.to_string()) {
        entry.season = cap[1].parse::<i32>().unwrap();
        entry.eptitle = SEASONREGEX.replace(&entry.eptitle,"$before $after").to_string();
        break;
    }
    for cap in EPISODEREGEX.captures_iter(&entry.eptitle.to_string()) {
        entry.episode = cap[1].parse::<i32>().unwrap();
        entry.eptitle = EPISODEREGEX.replace(&entry.eptitle,"$before $after").to_string();
        break;
    }

    entry.eptitle = SPACEKILLER.replace(&entry.eptitle,"$before $after").to_string();
    entry.eptitle = BEGINFIXER.replace(&entry.eptitle,|caps: &Captures| {format!("{}",&caps[1])}).to_string();


    match dbconn.execute(
        "INSERT INTO shows (show, url, eptitle, scraped, season, episode) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&entry.show, &entry.url, &entry.eptitle, &entry.scraped, &entry.season, &entry.episode],
    ) {
        Err(rusqlite::Error::SqliteFailure(errnum, errmsg)) => {
            if errnum.extended_code.eq(&1555) { () } else { panic!("Error -> {:?}: {:?}", errnum, errmsg); }
        },
        Ok(_) => { () },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    };
}

fn db_create(connection:&Connection) {
    match connection.execute(
        "
        CREATE TABLE shows (
            show TEXT,
            url TEXT NOT NULL PRIMARY KEY,
            eptitle TEXT,
            scraped FLOAT,
            season INT,
            episode INT
    )", params![],
    ) {
        Err(rusqlite::Error::SqliteFailure(errnum, errmsg)) => {
            if errnum.extended_code.eq(&1) { () } else { panic!("Error -> {:?}: {:?}", errnum, errmsg); }
        },
        Ok(_) => { () },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    };
}