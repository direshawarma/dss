use std::time::{SystemTime, UNIX_EPOCH};

use lazy_static::lazy_static;
use regex::{Captures, Regex};
//use futures::executor::block_on;
use rusqlite::{Connection, params};

#[derive(Default)]
struct Episode {
    series: String,
    url: String,
    title: String,
    scraped: f64,
    episode: i32,
    season: i32,
}


pub async fn db_add(series_title: &String, title: String, url: String) {
    let con = Connection::open("db.db").unwrap();


    db_create(&con);


    //futures::join!(series_db,episode_db); // execute concurrently and wait until both functions are done

    lazy_static! {
        static ref SEASONREGEX:Regex = Regex::new(r"[Ss]eason:? (\d+)").unwrap();
        static ref EPISODEREGEX:Regex = Regex::new(r"[Ee]pisode:? (\d+)").unwrap();
        static ref SPACEKILLER:Regex = Regex::new(r" +").unwrap();
        static ref BEGINFIXER:Regex = Regex::new(r"^.+?(\w)").unwrap();
    }
    ;

    let string = format!("{} {}", r"Watch", &series_title);
    let title_killer: Regex = Regex::new(&string).unwrap();

    let mut entry = Episode {
        series: series_title.to_string(),
        url: url.to_string(),
        title: title_killer.replace(title.as_str(), "$after").to_string(),
        scraped: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
        ..Default::default()
    };

    for cap in SEASONREGEX.captures_iter(&entry.title.to_string()) {
        entry.season = cap[1].parse::<i32>().unwrap();
        entry.title = SEASONREGEX.replace(&entry.title, "$before $after").to_string();
    }
    for cap in EPISODEREGEX.captures_iter(&entry.title.to_string()) {
        entry.episode = cap[1].parse::<i32>().unwrap();
        entry.title = EPISODEREGEX.replace(&entry.title, "$before $after").to_string();
    }

    entry.series = SPACEKILLER.replace(&entry.title, "$before $after").to_string();
    entry.title = BEGINFIXER.replace(&entry.title, |caps: &Captures| { format!("{}", &caps[1]) }).to_string();
    // FIXME things still don't wind up quite right with entry.title using simple regex
    //  TODO make a more robust way of figuring out episode title names and removing fluff


    match con.execute(
        "INSERT INTO episodes (series, url, title, scraped, season, episode) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&entry.series, &entry.url, &entry.title, &entry.scraped, &entry.season, &entry.episode],
    ) {
        Err(rusqlite::Error::SqliteFailure(E, error_message)) => {
            if E.extended_code.eq(&1545) { () } else { panic!("Error -> {:?}: {:?}", E, error_message); }
        }
        Ok(_) => { () },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    };
}


// TODO find out if a create check or insert check are as fast as directly checking or not because this
//   gets called for every episode read which might give us some I/O issues
// TODO use row id is a much faster, functionally equivalent substitute for actually creating an auto-incrementing
//   index so implement row id


async fn db_create(con: &Connection) {
    let call = "CREATE TABLE series (
            series TEXT,
            url TEXT NOT NULL PRIMARY KEY,
            last_retrieved FLOAT,
            url_type TEXT,
            parent_row INT);
        CREATE TABLE episodes (
        series TEXT,
        url TEXT NOT NULL PRIMARY KEY,
        title TEXT,
        scraped FLOAT,
        season INT,
        episode INT)";

    match con.execute(&call, []) {
        Err(rusqlite::Error::SqliteFailure(E, error_message)) => {
            lazy_static! {
                static ref TABLEEXISTS:Regex = Regex::new(r"^already exists$").unwrap();
            }
            ;
            //if E.eq(rusqlite::Error::()) && TABLEEXISTS.is_match(&error_message.unwrap().as_str())  {}
            //println!("Table {}} already exists",)}
            //else {
            panic!("Error -> {:?}: {:?}", E, error_message);
            //}
        },
        Ok(_) => { () }, // println!("episode db init ok") },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    };
}