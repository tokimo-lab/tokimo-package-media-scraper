use serde::{Deserialize, Serialize};

/// Adult metadata shared across `JavBus`, `JavDB`, `StashDB`, TPDB clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdultMetadata {
    pub video_id: String,
    pub title: Option<String>,
    pub poster_url: Option<String>,
    pub cover_url: Option<String>,
    pub source_url: Option<String>,
    pub actors: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub release_date: Option<String>,
    pub studio: Option<String>,
    pub duration: Option<u32>,
    pub rating: Option<f64>,
    pub source: String,
}

/// Adult series video item from prefix search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdultSeriesVideo {
    pub video_id: String,
    pub title: Option<String>,
    pub poster_url: Option<String>,
    pub release_date: Option<String>,
}

/// Individual artist credit (name + `MusicBrainz` artist ID).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistCredit {
    pub name: String,
    pub mb_id: String,
}

/// Music release match candidate from `MusicBrainz` search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMatchCandidate {
    pub mb_release_id: String,
    pub title: String,
    pub artist: String,
    pub year: Option<i32>,
    pub track_count: Option<i32>,
    pub country: Option<String>,
    pub format: Option<String>,
    pub score: Option<i32>,
}

/// Music release detail from `MusicBrainz`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMatchDetail {
    pub mb_release_id: String,
    pub mb_release_group_id: Option<String>,
    pub title: String,
    pub artist: String,
    pub artist_mb_id: Option<String>,
    pub year: Option<i32>,
    pub release_date: Option<String>,
    pub album_type: Option<String>,
    pub genres: Option<Vec<String>>,
    pub total_tracks: Option<i32>,
    pub total_discs: Option<i32>,
    pub cover_url: Option<String>,
    pub overview: Option<String>,
    pub spotify_id: Option<String>,
    pub tracks: Option<Vec<MusicTrack>>,
    /// Individual artist credits — one entry per artist.
    pub artist_credits: Vec<ArtistCredit>,
}

/// Track info within a music release.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicTrack {
    pub number: i32,
    pub title: String,
    pub duration: Option<i32>,
}

/// Lyrics result from LRCLIB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricsResult {
    /// Synced lyrics in .lrc format (with timestamps).
    pub synced_lyrics: Option<String>,
    /// Plain text lyrics.
    pub plain_lyrics: Option<String>,
    /// Whether this is an instrumental track.
    pub instrumental: bool,
}
