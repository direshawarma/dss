use std::time::{SystemTime, UNIX_EPOCH};

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use rusqlite::{Connection, params};

#[derive(Debug)]
struct Episode {
    series: String,
    url: String,
    eptitle: String,
    scraped: f64,
    episode: i32,
    season: i32, // Someday Sesame Street might manage to break this limit
}

pub fn db_add(seriestitle: &String, title: String, url: String) {
    let dbconn = Connection::open("db.db").unwrap();
    db_create(&dbconn); // if necessary


    lazy_static! {
        static ref SEASONREGEX:Regex = Regex::new(r"[Ss]eason:? (\d+)").unwrap();
        static ref EPISODEREGEX:Regex = Regex::new(r"[Ee]pisode:? (\d+)").unwrap();
        static ref SPACEKILLER:Regex = Regex::new(r" +").unwrap();
        static ref BEGINFIXER:Regex = Regex::new(r"^.+?(\w)").unwrap();
    }
    ;

    let string = format!("{} {}", r"Watch", &seriestitle);
    let title_killer: Regex = Regex::new(&string).unwrap();

    let mut entry = Episode {
        series: seriestitle.to_string(),
        url: url.to_string(),
        eptitle: title_killer.replace(title.as_str(), "$after").to_string(),
        scraped: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
        episode: 0,
        season: 1,
    };

    for cap in SEASONREGEX.captures_iter(&entry.eptitle.to_string()) {
        entry.season = cap[1].parse::<i32>().unwrap();
        entry.eptitle = SEASONREGEX.replace(&entry.eptitle, "$before $after").to_string();
        break;
    }
    for cap in EPISODEREGEX.captures_iter(&entry.eptitle.to_string()) {
        entry.episode = cap[1].parse::<i32>().unwrap();
        entry.eptitle = EPISODEREGEX.replace(&entry.eptitle, "$before $after").to_string();
        break;
    }

    entry.eptitle = SPACEKILLER.replace(&entry.eptitle, "$before $after").to_string();
    entry.eptitle = BEGINFIXER.replace(&entry.eptitle, |caps: &Captures| { format!("{}", &caps[1]) }).to_string();
    // FIXME things still don't wind up quite right with entry.eptitle using simple regex
    //  TODO make a more robust way of figuring out episode title names and removing fluff


    match dbconn.execute(
        "INSERT INTO series (series, url, eptitle, scraped, season, episode) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&entry.series, &entry.url, &entry.eptitle, &entry.scraped, &entry.season, &entry.episode],
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
        CREATE TABLE episodes (
            series TEXT,
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