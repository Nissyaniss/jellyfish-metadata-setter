Tool to retrieve metadata + covers + music file and generate the nfo files and folder for Jellyfish.<br>
THIS IS NOT USABLE **IN** JELLYFISH MAY BE PLANNED LATER BUT I HATE C#

Description is NOT good, i haven't found a way to "search" wikipedia without scraping (and i don't want to do that)<br>
So for example the band [Akara](https://www.youtube.com/channel/UCeJqhwrIBg_sTsqYQrXAk3Q) will have the [Akara street food](https://en.wikipedia.org/wiki/Akara) description because the band has no Wikipdia page but the name is already in use in the Wikipedia database.<br>
Also if there are multiple results for an artist name the descritpion will be the like of "... can be ... or ...".<br>
My implementation is very basic.

Now that yt-dlp is implemented I need to clean up the code (which is a mess right now) and add little functionnality (progress bars, better error handling...)

Big thanks to [Acoustid](https://github.com/acoustid) for fpcalc which this project is heavily using.<br>
Also big thanks to [yt-dlp](https://github.com/yt-dlp/yt-dlp) for the amazing CLI.<br>
Thanks to [MusicBrainz](https://musicbrainz.org/) and their amazing database.<br>
Thanks to [Wikipedia](https://en.wikipedia.org) for the free API to retrieve descriptions.<br>
And thanks [Deezer](https://www.deezer.com/en/) for the free API to retrieve cover.

And thanks to all the crates that are incredibly useful:
- [musicbrainz_rs](https://github.com/RustyNova016/musicbrainz_rs) for the wrapper around MusicBrainz API
- [mp4ameta](https://github.com/saecki/mp4ameta) for the tagging system

And the big crates that are already well-known:
- [chrono](github.com/chronotope/chrono)
- [clap](https://github.com/clap-rs/clap)
- [dotenv](https://github.com/dotenv-rs/dotenv)
- [image](https://github.com/image-rs/image)
- [inquire](https://github.com/mikaelmello/inquire)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [serde](https://github.com/serde-rs/serde)
- [serde-json](https://github.com/serde-rs/json)
- [tokio](https://github.com/tokio-rs/tokio)
