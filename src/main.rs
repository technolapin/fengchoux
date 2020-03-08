use std::path::PathBuf;
use std::result::Result;
use std::process::Command;
use rand::prelude::random;
use std::thread::sleep;
use std::time::Duration;
use std::env;

struct WallpapersManager
{
    history: Vec<PathBuf>,
    mode: String
}

#[derive(Debug)]
struct Error(String);


impl From<std::io::Error> for Error
{
    fn from(error: std::io::Error) -> Self
    {
	Self(format!("{:?}", error))
    }
}

impl From<std::num::ParseIntError> for Error
{
    fn from(error: std::num::ParseIntError) -> Self
    {
	Self(format!("{:?}", error))
    }
}


impl WallpapersManager
{
    fn new(dir: PathBuf) -> Self
    {
	let files = dir.read_dir().expect("read_dir call failed")
	    .filter_map(|unsure_entry| match unsure_entry
			{
			    Err(_) => None,
			    Ok(entry) => Some(entry.path())
			}).collect::<Vec<_>>();
	
	Self
	{
	    history: files,
	    mode: String::from("--bg-fill")
	}
    }

    fn set(&self, path: &PathBuf) -> Result<(), Error>
    {
	//	wallpaper::set_from_path(path.to_str().unwrap())?;
	Command::new("feh")
	    .arg(self.mode.as_str())
	    .arg(path.to_str().unwrap())
	    .output()?;
	Ok(())
    }
    
    fn set_random(&self) -> Result<(), Error>
    {
	let i = random::<usize>() % self.history.len();
	self.set(&self.history[i])?;
	
	Ok(())
    }

    fn cycle(&self, delay: Duration) -> Result<(), Error>
    {
	loop
	{
	    self.set_random()?;
	    sleep(delay);
	}
    }
}

fn main() -> Result<(), Error>
{

    let mut args = env::args();
    args.next();
    let (name, delay) = match (args.next(), args.next())
    {
	(Some(name), Some(delay)) => {(name, delay)},
	_ => return Err(Error(String::from("usage: fengchoux folder duration")))
    };

    let delay = delay.parse()?;
    let delay =  Duration::new(delay, 0);

    let dir_path = PathBuf::from(name);

    let joris = WallpapersManager::new(dir_path);


    joris.cycle(delay)?;

    Ok(())
}
