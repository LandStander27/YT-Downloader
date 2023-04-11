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
	return format!("{}\\{}", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string(), file);
}

struct Filename {
	video: String,
	audio: String
}

struct Video {
	url: String,
	filename: Filename,
	ext: Filename,
	title: String
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

		let video = threads.remove(0).join().unwrap();
		let audio = threads.remove(0).join().unwrap();
		let title = threads.remove(0).join().unwrap();
		let video_ext = threads.remove(0).join().unwrap();
		let audio_ext = threads.remove(0).join().unwrap();

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
			title: String::from_utf8(title.stdout).unwrap().trim().to_string()
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

fn download_yt_dlp() {

	println!("Downloading yt-dlp.exe.");

	// let client = reqwest::Client::default();
	// let res = client.get("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe").send().await;

	let res = reqwest::blocking::get("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe").unwrap();

	let mut file = File::create(&exe_dir("yt-dlp.exe")).unwrap();
	file.write_all(&res.bytes().unwrap()).unwrap();

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
	let _ = Command::new(&exe_dir("ffmpeg.exe"))
		.arg("-i")
		.arg(video)
		.arg("-i")
		.arg(audio)
		.arg("-c")
		.arg("copy")
		.arg("-y")
		.arg(out)
		.stderr(Stdio::piped())
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.output()
		.expect("Unable to call ffmpeg");
}

fn download_playlist(url: &str) {
	let amount_out = Command::new(&exe_dir("yt-dlp.exe"))
		.arg("-q")
		.arg("--print")
		.arg("playlist_count")
		.arg(url)
		.output()
		.expect("Unable to call yt-dlp");

	let amount: u32 = String::from_utf8(amount_out.stdout).unwrap().split("\n").collect::<Vec<&str>>()[0].trim().parse().unwrap();

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

		progressbar(vid.download_video(), format!("{}/{}", i+1, amount));
		progressbar(vid.download_audio(), format!("{}/{}", i+1, amount));
		println!("Combining files.");
		combine_files(vid.filename.video.clone(), vid.filename.audio.clone(), format!("{}.{}", vid.title, vid.ext.video));

		// println!("Removing temp files.");
		thread::sleep(std::time::Duration::from_millis(750));
		std::fs::remove_file(vid.filename.video).unwrap();
		std::fs::remove_file(vid.filename.audio).unwrap();
		println!("Downloaded {}", vid.title);

	}

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

	if !Path::new(&exe_dir("yt-dlp.exe")).exists() {
		download_yt_dlp();
	}
	if !Path::new(&exe_dir("ffmpeg.exe")).exists() {
		download_ffmpeg();
	}

	let args: Vec<String> = std::env::args().collect();
	
	if args.len() == 1 {

		let url = read("Url (Can be playlist or video) ? ");

		let option: String = read("Do you want to download audio or video (1: video, 2: audio) ? ").to_lowercase();
		if option != "1" && option != "2" {
			eprintln!("Not an option");
			exit(1);
		}

		if !url.starts_with("https://www.youtube.com/playlist") {
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
			download_playlist(url.as_str());
			println!("Done.");

		}
		
		rpassword::prompt_password("Press enter to exit ").unwrap();

	} else {
		if !args[1].starts_with("https://www.youtube.com/playlist") {
			println!("Loading.");
			let mut vid = Video::new(args[1].to_string());
			// progressbar(vid.download_video());

			if args[1..].contains(&"--audio".to_string()) {
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
			download_playlist(&args[1]);
			println!("Done.");

		}
	}

}