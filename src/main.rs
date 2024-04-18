use std::cmp::min;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use clap::Parser;
use anyhow::{anyhow, Result};
use gpx::read;
use gpx::Gpx;
use url::Url;

/// Generate Animated Route Maps v3 links for a gpx file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Gpx file
	file: PathBuf,
}

fn main() -> Result<()> {
	let args: Args = Args::parse();

	let file = File::open(args.file)?;
	let reader = BufReader::new(file);
	let gpx: Gpx = read(reader)?;
	let route = gpx.routes.first().ok_or(anyhow!("No route in file"))?;
	let points = &route.points;
	let mut url = Url::parse("https://www.mapchannels.com/routemaps3/map.htm")?;

	let mut i = 0;
	while i + 1 < points.len() {
		let slice = &points[i..min(i + 10, points.len())];
		let point_string = slice.iter().map(|p| format!("{},{}", p.point().y(), p.point().x())).collect::<Vec<_>>().join(",");
		url.set_query(Some(format!("route={}", point_string).as_str()));
		println!("{}", url);
		i += 9;
	}

	Ok(())
}