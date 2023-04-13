use std::process::{Command, Stdio, exit};
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::path::{Path};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use colored::Colorize;
use url::Url;

fn exe_dir(file: &str) -> String {
	return format!("{}\\extra\\{}", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string(), file);
}

struct Filename {
	video: String,
	audio: String
}

struct Video {
	url: String,
	filename: Filename,
	ext: Filename,
	title: String,
	channel: String,
	duration: String,
	views: u64,
	likes: u64,
	channel_subs: u64,
	description: String
}

impl Video {
	fn new(url: String) -> Self {

		let parse = Url::parse(url.as_str());
		match parse {
			Ok(u) => {
				if !(u.host() == Some(url::Host::Domain("youtube.com")) || u.host() == Some(url::Host::Domain("www.youtube.com")) || u.host() == Some(url::Host::Domain("www.youtu.be")) || u.host() == Some(url::Host::Domain("youtu.be"))) {
					eprintln!("Invalid video url");
					exit(1);
				}
				println!("Verfied URL");
			},
			Err(e) => {
				eprintln!("Could not parse url: {}", e);
				exit(1);
			}
		}

		let mut threads: Vec<thread::JoinHandle<std::process::Output>> = Vec::new();

		let mut url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("bv[ext=mp4]")
				.arg("--restrict-filenames")
				.arg("--print")
				.arg("filename")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("ba[ext=m4a]")
				.arg("--restrict-filenames")
				.arg("--print")
				.arg("filename")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("--print")
				.arg("title")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("bv[ext=mp4]")
				.arg("--restrict-filenames")
				.arg("--print")
				.arg("ext")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("ba[ext=m4a]")
				.arg("--restrict-filenames")
				.arg("--print")
				.arg("ext")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("ba[ext=m4a]")
				.arg("--print")
				.arg("channel")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("ba[ext=m4a]")
				.arg("--print")
				.arg("duration_string")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("ba[ext=m4a]")
				.arg("--print")
				.arg("view_count")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("ba[ext=m4a]")
				.arg("--print")
				.arg("like_count")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("ba[ext=m4a]")
				.arg("--print")
				.arg("channel_follower_count")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("-f")
				.arg("ba[ext=m4a]")
				.arg("--print")
				.arg("description")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		let video = threads.remove(0).join().unwrap();
		let audio = threads.remove(0).join().unwrap();
		let title = threads.remove(0).join().unwrap();
		let video_ext = threads.remove(0).join().unwrap();
		let audio_ext = threads.remove(0).join().unwrap();
		let channel = threads.remove(0).join().unwrap();
		let duration = threads.remove(0).join().unwrap();
		let views = threads.remove(0).join().unwrap();
		let likes = threads.remove(0).join().unwrap();
		let channel_subs = threads.remove(0).join().unwrap();
		let description = threads.remove(0).join().unwrap();

		return Self {
			url: url,
			filename: Filename {
				video: String::from_utf8(video.stdout).unwrap().trim().to_string(),
				audio: String::from_utf8(audio.stdout).unwrap().trim().to_string()
			},
			ext: Filename {
				video: String::from_utf8(video_ext.stdout).unwrap().trim().to_string(),
				audio: String::from_utf8(audio_ext.stdout).unwrap().trim().to_string()
			},
			title: String::from_utf8(title.stdout).unwrap().trim().to_string(),
			channel: String::from_utf8(channel.stdout).unwrap().trim().to_string(),
			duration: String::from_utf8(duration.stdout).unwrap().trim().to_string(),
			views: String::from_utf8(views.stdout).unwrap().trim().parse().unwrap(),
			likes: String::from_utf8(likes.stdout).unwrap().trim().parse().unwrap(),
			channel_subs: String::from_utf8(channel_subs.stdout).unwrap().trim().parse().unwrap(),
			description: String::from_utf8(description.stdout).unwrap().trim().to_string()

		}
	}

	fn download_video(&mut self) -> Receiver<i32> {
		let mut proc = Command::new(&exe_dir("yt-dlp.exe"))
			.arg("--newline")
			.arg("--progress")
			.arg("-q")
			.arg("--restrict-filenames")
			.arg("-f")
			.arg("bv[ext=mp4]")
			.arg(self.url.clone())
			.stderr(Stdio::piped())
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
			.expect("Unable to call yt-dlp");

		let stdout = proc.stdout.take().unwrap();

		let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

		thread::spawn(move || {
			let reader = BufReader::new(stdout);
	
			reader 
				.lines()
				.filter_map(|line| line.ok())
				.for_each(|line| {
					let mut perc: &str = (&line[line.find("[download]").unwrap()+"[download]".len()..line.find("%").unwrap()]).trim();
					if perc.contains(".") {
						perc = &perc[..perc.find(".").unwrap()];
					}
					tx.send(perc.parse::<i32>().unwrap()).unwrap();
				});
		});

		return rx;
	
	}
	
	fn download_audio(&mut self) -> Receiver<i32> {
		let mut proc = Command::new(&exe_dir("yt-dlp.exe"))
			.arg("--newline")
			.arg("--progress")
			.arg("-q")
			.arg("--restrict-filenames")
			.arg("-f")
			.arg("ba[ext=m4a]")
			.arg(self.url.clone())
			.stderr(Stdio::piped())
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
			.expect("Unable to call yt-dlp");

		let stdout = proc.stdout.take().unwrap();

		let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

		thread::spawn(move || {
			let reader = BufReader::new(stdout);
	
			reader 
				.lines()
				.filter_map(|line| line.ok())
				.for_each(|line| {
					let mut perc: &str = (&line[line.find("[download]").unwrap()+"[download]".len()..line.find("%").unwrap()]).trim();
					if perc.contains(".") {
						perc = &perc[..perc.find(".").unwrap()];
					}
					tx.send(perc.parse::<i32>().unwrap()).unwrap();
				});
		});

		return rx;
	
	}

}

struct Playlist {
	title: String,
	video_amount: u32,
	creator: String
}

impl Playlist {
	fn new(url: String) -> Self {

		let parse = Url::parse(url.as_str());
		match parse {
			Ok(u) => {
				if !(u.host() == Some(url::Host::Domain("youtube.com")) || u.host() == Some(url::Host::Domain("www.youtube.com"))) {
					eprintln!("Invalid video url");
					exit(1);
				}
				println!("Verfied URL");
			},
			Err(e) => {
				eprintln!("Could not parse url: {}", e);
				exit(1);
			}
		}

		let mut threads: Vec<thread::JoinHandle<std::process::Output>> = Vec::new();

		let mut url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("--lazy-playlist")
				.arg("--print")
				.arg("playlist_title")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("--lazy-playlist")
				.arg("--print")
				.arg("playlist_count")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		url_cp = url.clone();

		threads.push(thread::spawn(|| {
			return Command::new(&exe_dir("yt-dlp.exe"))
				.arg("-q")
				.arg("--lazy-playlist")
				.arg("--print")
				.arg("playlist_uploader")
				.arg(url_cp)
				.output()
				.expect("Unable to call yt-dlp");
		}));

		let title = threads.remove(0).join().unwrap();
		let count = threads.remove(0).join().unwrap();
		let creator = threads.remove(0).join().unwrap();

		return Self {
			title: String::from_utf8(title.stdout).unwrap().trim().split("\n").collect::<Vec<&str>>()[0].trim().to_string(),
			video_amount: String::from_utf8(count.stdout).unwrap().split("\n").collect::<Vec<&str>>()[0].trim().parse().unwrap(),
			creator: String::from_utf8(creator.stdout).unwrap().trim().split("\n").collect::<Vec<&str>>()[0].trim().to_string()
		}

	}
}

fn download_yt_dlp() {

	println!("Downloading yt-dlp.exe.");

	// let client = reqwest::Client::default();
	// let res = client.get("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe").send().await;

	let res = reqwest::blocking::get("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe").unwrap();

	let mut file = File::create(&exe_dir("yt-dlp.exe")).unwrap();
	file.write_all(&res.bytes().unwrap()).unwrap();

}

fn update() {
	println!("Updating.");

	let res = reqwest::blocking::get("https://github.com/LandStander27/YT-Downloader/releases/latest/download/yt_down.exe").unwrap();

	let current = std::env::current_exe().unwrap();
	let new = format!("{}\\yt_down_new.exe", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap());

	let mut file = File::create(new.clone()).unwrap();
	file.write_all(&res.bytes().unwrap()).unwrap();

	println!("Writing over executable.");
	std::fs::rename(current.clone(), format!("{}\\yt_down_old.exe", current.parent().unwrap().to_str().unwrap())).unwrap();
	if let Err(e) = std::fs::rename(new, current.clone()) {
		std::fs::rename(format!("{}\\yt_down_old.exe", current.parent().unwrap().to_str().unwrap()), current).unwrap();
		panic!("{}", e);
	}
	println!("Updated.");

}

fn get_latest_ffmpeg_version() -> String {
	return ureq::get("https://www.gyan.dev/ffmpeg/builds/release-version").call().unwrap().into_string().unwrap();
}

fn download_ffmpeg() {
	println!("Downloading ffmpeg.exe.");

	// let client = reqwest::Client::default();
	// let res = client.get(format!("https://github.com/GyanD/codexffmpeg/releases/latest/download/ffmpeg-{}-essentials_build.zip", get_latest_ffmpeg_version())).send();
	
	let res = reqwest::blocking::get(format!("https://github.com/GyanD/codexffmpeg/releases/latest/download/ffmpeg-{}-essentials_build.zip", get_latest_ffmpeg_version())).unwrap();

	// if let Err(e) = res {
	// 	return Err(e.to_string());
	// } else if let Ok(o) = res {
	// 	let resp = res.unwrap();
	// }
	let mut file = File::create(&exe_dir("ffmpeg.zip")).unwrap();
	file.write_all(&res.bytes().unwrap()).unwrap();

	thread::sleep(std::time::Duration::from_millis(250));

	let file_archive = File::open(&exe_dir("ffmpeg.zip")).unwrap();

	let mut archive = zip::ZipArchive::new(file_archive).unwrap();
	let mut ffmpeg = archive.by_name(format!("ffmpeg-{}-essentials_build/bin/ffmpeg.exe", get_latest_ffmpeg_version()).as_str()).unwrap();
	let mut outfile = File::create(&exe_dir("ffmpeg.exe")).unwrap();

	std::io::copy(&mut ffmpeg, &mut outfile).unwrap();

	std::fs::remove_file(&exe_dir("ffmpeg.zip")).unwrap()

	// zip_extract::extract(Cursor::new(archive), &PathBuf::from(&exe_dir("ffmpeg")), true).unwrap();
	

}

fn progressbar(rx: Receiver<i32>, label: String) {
	// let columns: u16 = terminal_size::Height(terminal_size::terminal_size().unwrap().1.0).0;
	let columns = if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
		w
	} else {
		10
	};

	print!("{} │ 0% │{}│", label, " ".repeat((columns-8-label.len() as u16) as usize));
	std::io::stdout().flush().unwrap();
	loop {
		let perc = match rx.recv() {
			Ok(r) => r,
			Err(e) => {
				if e.to_string() == "receiving on a closed channel" {
					break;
				} else {
					panic!("{}", e);
				}
			}
		}.to_string();

		let full_bar: String = " ".repeat((columns - label.len() as u16 - 7 - perc.len() as u16) as usize);

		let bar = &full_bar[..(full_bar.len()-1) * perc.parse::<usize>().unwrap()/100];

		print!("\r{} │ {}% │{}{}│", label, perc, bar.on_green(), " ".repeat(full_bar.len() - bar.len()));
		std::io::stdout().flush().unwrap();

	}
	print!("\r{}\r", " ".repeat(columns as usize));

}

fn combine_files(video: String, audio: String, out: String) {
	let out_2 = out.replace("/", "").replace("\\", "").replace(":", "").replace("*", "").replace("?", "").replace("\"", "").replace("<", "").replace(">", "").replace("|", "");
	let _ = Command::new(&exe_dir("ffmpeg.exe"))
		.arg("-i")
		.arg(video)
		.arg("-i")
		.arg(audio)
		.arg("-c")
		.arg("copy")
		.arg("-y")
		.arg(out_2)
		.stderr(Stdio::piped())
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.output()
		.expect("Unable to call ffmpeg");
}

fn download_playlist(url: &str, numbered: bool) {

	println!("If the selected playlist is big (>100), it could take a while to start downloading.");

	let playlist = Playlist::new(url.to_string());
	let folder = playlist.title.clone().replace("/", "").replace("\\", "").replace(":", "").replace("*", "").replace("?", "").replace("\"", "").replace("<", "").replace(">", "").replace("|", "");
	if let Err(e) = std::fs::create_dir(folder.clone()) {
		if e.to_string() == "Cannot create a file when that file already exists." {
			eprintln!("Playlist folder already exists.");
			exit(1);
		}
	}

	let amount = playlist.video_amount;

	for i in 0..amount {
		let url_out = Command::new(&exe_dir("yt-dlp.exe"))
			.arg("-q")
			.arg("--playlist-items")
			.arg((i+1).to_string())
			.arg("--print")
			.arg("original_url")
			.arg(url)
			.output()
			.expect("Unable to call yt-dlp");
		
		let url = String::from_utf8(url_out.stdout).unwrap().trim().to_string();
		
		let mut vid = Video::new(url);
		// progressbar(vid.download_video());

		progressbar(vid.download_video(), format!("{}/{}:1", i+1, amount));
		progressbar(vid.download_audio(), format!("{}/{}:2", i+1, amount));
		println!("Combining files.");

		combine_files(vid.filename.video.clone(), vid.filename.audio.clone(), format!("{}.{}", vid.title, vid.ext.video));

		// println!("Removing temp files.");
		thread::sleep(std::time::Duration::from_millis(750));
		std::fs::remove_file(vid.filename.video).unwrap();
		std::fs::remove_file(vid.filename.audio).unwrap();
		if numbered {
			std::fs::rename(format!("{}.{}", vid.title, vid.ext.video), format!("./{}/({}) {}.{}", folder, i+1, vid.title, vid.ext.video)).unwrap();
		} else {
			std::fs::rename(format!("{}.{}", vid.title, vid.ext.video), format!("./{}/{}.{}", folder, vid.title, vid.ext.video)).unwrap();
		}
		
		println!("Downloaded {}", vid.title);

	}

	println!("Downloaded playlist to \"./{}\"", folder);

}

fn read(prompt: &str) -> String {
	print!("{}", prompt);
	std::io::stdout().flush().unwrap();
	let mut buffer: String = String::new();
	let stdin: std::io::Stdin = std::io::stdin();
	stdin.read_line(&mut buffer).unwrap();
	return buffer.trim().to_string();
}



fn main() {

	if !Path::new(&exe_dir("")).exists() {
		std::fs::create_dir(&exe_dir("")).unwrap();
	}

	if !Path::new(&exe_dir("yt-dlp.exe")).exists() {
		download_yt_dlp();
	}
	if !Path::new(&exe_dir("ffmpeg.exe")).exists() {
		download_ffmpeg();
	}

	let args: Vec<String> = std::env::args().collect();

	if let Err(e) = ctrlc::set_handler(|| {
		println!("\nExiting.");
		exit(0);
	}) {
		eprintln!("Could not set ctrl-c handler: {}", e);
	}

	if args.len() == 1 {

		println!("Starting in interactive mode.");
		println!("Note: Running with arguments lets you have more control, run with --help for more info.\n");

		let url = read("Url (Can be playlist or video) ? ");

		let option: String = read("Do you want to download audio or video (1: video, 2: audio) ? ").to_lowercase();
		if option != "1" && option != "2" {
			eprintln!("Not an option");
			exit(1);
		}

		if !url.starts_with("https://youtube.com/playlist") && !url.starts_with("https://www.youtube.com/playlist") {
			println!("Loading.");
			let mut vid = Video::new(url);
			// progressbar(vid.download_video());

			if option == "1" {
				progressbar(vid.download_video(), "Downloading video".to_string());
				progressbar(vid.download_audio(), "Downloading audio".to_string());
				println!("Combining files.");
				combine_files(vid.filename.video.clone(), vid.filename.audio.clone(), format!("{}.{}", vid.title, vid.ext.video));
			
				println!("Removing temp files.");
				thread::sleep(std::time::Duration::from_millis(750));
				std::fs::remove_file(vid.filename.video).unwrap();
				std::fs::remove_file(vid.filename.audio).unwrap();
			} else {
				progressbar(vid.download_audio(), "Downloading audio".to_string());

				let _ = Command::new(&exe_dir("ffmpeg.exe"))
					.arg("-y")
					.arg("-i")
					.arg(vid.filename.audio.clone())
					.arg("-acodec")
					.arg("libmp3lame")
					.arg(format!("{}.mp3", vid.title))
					.output()
					.expect("Unable to call ffmpeg");

				thread::sleep(std::time::Duration::from_millis(250));
				std::fs::remove_file(vid.filename.audio).unwrap();
			}


			println!("Done.");
		} else {
			println!("Loading.");
			download_playlist(url.as_str(), false);
			println!("Done.");

		}
		
		rpassword::prompt_password("Press enter to exit ").unwrap();

	} else {

		if args[1..].contains(&"--help".to_string()) {

			println!("
Youtube Downloader made in Rust

usage:
yt_down [url] [arguments]

Arguments:

--help:     Show this menu
--audio:    Only download the audio (only available for single videos)
--info:     Only show the info of the video/playlist
--github:   Open the github in the default browser
--numbered: Number videos in a playlist incase there are not numbers in the titles (only available for playlists)
--update:   Update YT-Downloader");

			exit(0);
		}

		if args[1..].contains(&"--github".to_string()) {

			open::that("https://github.com/LandStander27/YT-Downloader").unwrap();

			exit(0);
		}

		if args[1..].contains(&"--update".to_string()) {
			update();
			println!("Done.");
			exit(0);
		}

		if !args[1].starts_with("https://youtube.com/playlist") && !args[1].starts_with("https://www.youtube.com/playlist") {
			println!("Loading.");
			let mut vid = Video::new(args[1].to_string());
			// progressbar(vid.download_video());

			if args[1..].contains(&"--info".to_string()) {

// 				println!("
// Title: {}
// Duration: {}
// Views: {}
// Likes: {}
// Channel: {}
// Channel subscribers: {}

// Description:
// {}", vid.title, vid.duration, vid.views, vid.likes, vid.channel, vid.channel_subs, vid.description);

				println!("
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}

{}:
{}", 
"Title".green(), vid.title, 
"Duration".green(), vid.duration, 
"Views".green(), vid.views, 
"Likes".green(), vid.likes, 
"Channel".green(), vid.channel, 
"Channel subscribers".green(), vid.channel_subs, 
"Description".green(), vid.description);

				exit(0);

			} else if args[1..].contains(&"--audio".to_string()) {
				progressbar(vid.download_audio(), "Downloading audio".to_string());
				
				let _ = Command::new(&exe_dir("ffmpeg.exe"))
					.arg("-y")
					.arg("-i")
					.arg(vid.filename.audio.clone())
					.arg("-acodec")
					.arg("libmp3lame")
					.arg(format!("{}.mp3", vid.title))
					.output()
					.expect("Unable to call ffmpeg");

				thread::sleep(std::time::Duration::from_millis(250));
				std::fs::remove_file(vid.filename.audio).unwrap();

			} else {
				progressbar(vid.download_video(), "Downloading video".to_string());
				progressbar(vid.download_audio(), "Downloading audio".to_string());
				println!("Combining files.");
				combine_files(vid.filename.video.clone(), vid.filename.audio.clone(), format!("{}.{}", vid.title, vid.ext.video));
			
				println!("Removing temp files.");
				thread::sleep(std::time::Duration::from_millis(750));
				std::fs::remove_file(vid.filename.video).unwrap();
				std::fs::remove_file(vid.filename.audio).unwrap();
			}

			println!("Done.");
		} else {
			println!("Loading.");

			if args[1..].contains(&"--info".to_string()) {

				let playlist = Playlist::new(args[1].to_string());

				println!("
{}: {}
{}: {}
{}: {}", 
"Title".green(), playlist.title, 
"Creator".green(), playlist.creator, 
"Video amount".green(), playlist.video_amount);

				exit(0);
			}

			download_playlist(&args[1], args[1..].contains(&"--numbered".to_string()));
			println!("Done.");

		}
	}

}