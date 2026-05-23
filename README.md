# tokimo-package-media-scraper

Metadata scraping clients and utility modules for [TokimoOS](https://github.com/tokimo-lab/tokimo).

## Metadata Providers

| Service | Module | Description |
|---------|--------|-------------|
| TMDB | `metadata_providers::tmdb` | Movies, TV shows, people, images |
| OMDb | `metadata_providers::omdb` | Movie / series lookup by IMDb ID |
| Douban | `metadata_providers::douban` | Chinese movie / book reviews |
| Bangumi | `metadata_providers::bangumi` | Anime / game tracking |
| MusicBrainz | `metadata_providers::musicbrainz` | Music metadata (artists, albums, recordings) |
| TheTVDB | `metadata_providers::thetvdb` | TV series / episode metadata |
| FanArt | `metadata_providers::fanart` | Fan-created artwork for media |
| Spotify | `metadata_providers::spotify` | Spotify catalog search |
| Deezer | `metadata_providers::deezer` | Deezer catalog search |
| LRCLib | `metadata_providers::lrclib` | Synced lyrics lookup |
| JavBus | `metadata_providers::javbus` | JAV metadata |
| JavDB | `metadata_providers::javdb` | JAV metadata |
| StashDB | `metadata_providers::stashdb` | Stash metadata |
| TPDB | `metadata_providers::tpdb` | ThePornDB metadata |
| Wikipedia | `metadata_providers::wikipedia` | Article summaries |
| Qidian | `metadata_providers::qidian` | Chinese web novels |

## Standalone Modules

| Module | Description |
|--------|-------------|
| `assrt` | ASS subtitle translation via assrt.net |
| `geocoding` | Address geocoding |
| `github_releases` | GitHub release asset download |
| `model_downloader` | AI model file download with progress tracking |
| `nominatim` | OpenStreetMap geocoding |
| `open_meteo` | Weather data |
| `timor_holiday` | Chinese public holiday calendar |
| `weclaw` | WeClaw (iLink / ClawBot) messaging API |
| `pan115_auth` | 115 cloud drive authentication |

## Shared Utilities

- `cache::RequestCache` — TTL-based HTTP response cache
- `cloudflare` — Cloudflare challenge detection helpers
- `error::ClientError` — unified error type

## Usage

```toml
[dependencies]
tokimo-media-scraper = { git = "https://github.com/tokimo-lab/tokimo-package-media-scraper.git", rev = "b124c1c" }
```

```rust
use tokimo_media_scraper::metadata_providers::tmdb::{TmdbClient, TmdbConfig};

let client = TmdbClient::new(TmdbConfig { api_key: "...".into() });
let results = client.search_movie("Inception").await?;
```

## License

MIT OR Apache-2.0
