use std::{
	env,
	fs::{self, File},
	path::Path,
	process::Command,
	thread::sleep,
	time::Duration,
};

use chrono::Utc;
use env_logger::{Env, Target};

fn main() {
	let mut time_stamp = Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00");
	dotenv::dotenv().ok();
	env_logger::builder()
		.parse_env(Env::new().default_filter_or("debug"))
		.format_indent(Some(4))
		.format_target(false)
		.target(if let Ok(path) = env::var("LOGS") {
			Target::Pipe(Box::new({
				let path = Path::new(&path);
				if !path.exists() {
					fs::create_dir_all(path).unwrap();
				}

				while Path::new(&format!("{}/{}.log", path.display(), time_stamp)).exists() {
					time_stamp = Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00");
					sleep(Duration::new(1, 0))
				}
				File::create(&format!("{}/{}.log", path.display(), time_stamp)).unwrap()
			}))
		} else {
			Target::Stdout
		})
		.init();

	let file_content = match env::var("EXCLUDE") {
		Ok(path) => match fs::read_to_string(path) {
			Ok(file_content) => file_content,
			Err(err) => panic!("{}", err),
		},
		Err(_) => String::new(),
	};
	let archive_name = format!("::{}-{}.zst", env!("NAME"), time_stamp);
	let mut args = vec![
		"create",
		"-psvC",
		"auto,zstd,22",
		"--exclude-caches",
		"--exclude-nodump",
	];
	args.extend(file_content.split("\n").map(|x| ["-e", x.trim()]).flatten());
	args.extend([archive_name.as_ref(), env!("IN_DIR")].iter());

	log::info!("Backing Up!");
	Command::new(format!("{}/borg", env!("BORG_LOC")))
		.env(
			"BORG_REPO",
			format!("Borg:{}/{}", env!("OUT_DIR"), env!("NAME")),
		)
		.env("BORG_PASSPHRASE", env!("PASS"))
		.args(args)
		.spawn()
		.unwrap()
		.wait()
		.unwrap();

	log::info!("Pruning!");
	Command::new(format!("{}/borg", env!("BORG_LOC")))
		.env(
			"BORG_REPO",
			format!("Borg:{}{}", env!("OUT_DIR"), env!("NAME")),
		)
		.env("BORG_PASSPHRASE", env!("PASS"))
		.args(["prune", "-psvd", "7", "-w", "4", "-m", "6"])
		.spawn()
		.unwrap()
		.wait()
		.unwrap();

	log::info!("Compacting!");
	Command::new(format!("{}/borg", env!("BORG_LOC")))
		.env(
			"BORG_REPO",
			format!("Borg:{}{}", env!("OUT_DIR"), env!("NAME")),
		)
		.env("BORG_PASSPHRASE", env!("PASS"))
		.arg("compact")
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
}
