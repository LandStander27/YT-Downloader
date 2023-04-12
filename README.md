## YT Downloader
- A youtube downloader written in Rust

### Building from source
- Install [Rust](https://www.rust-lang.org/)
- Clone the repo
- Run `cargo r --release`
- End binary will be in `.\target\release\yt_down.exe`
### Installing
- Download `yt_down.exe` from the latest release
- Move the executable somewhere other then downloads, something like `C:\YT_Downloader\yt_down.exe`
- Run `yt_down.exe`
### Extra
- Running with no arguments (Ex: From file explorer) initiates interactive mode (Not recommended), with arguments instantly carries out the process
- Run with `--help` to get the arguments options
- Uses [ffmpeg](https://ffmpeg.org/) and [yt-dlp](https://github.com/yt-dlp/yt-dlp)