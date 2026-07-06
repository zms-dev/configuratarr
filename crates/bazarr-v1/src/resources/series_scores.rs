use core_macros::nested;

/// Series subtitle-match scoring weights (`settings-series_scores-*`).
///
/// Each field is the score a match on that attribute contributes; the total is
/// compared against `general.minimum_score` to decide whether a subtitle is good
/// enough to download.
#[nested(case = snake)]
pub struct SeriesScores {
    /// Points for a release-hash match (exact file match).
    pub hash: Option<i32>,
    /// Points for a matching series title.
    pub series: Option<i32>,
    /// Points for a matching year.
    pub year: Option<i32>,
    /// Points for a matching season number.
    pub season: Option<i32>,
    /// Points for a matching episode number.
    pub episode: Option<i32>,
    /// Points for a matching release group.
    pub release_group: Option<i32>,
    /// Points for a matching source (BluRay, WEB-DL, …).
    pub source: Option<i32>,
    /// Points for a matching resolution.
    pub resolution: Option<i32>,
    /// Points for a matching video codec.
    pub video_codec: Option<i32>,
    /// Points for a matching audio codec.
    pub audio_codec: Option<i32>,
    /// Points for a matching streaming service.
    pub streaming_service: Option<i32>,
    /// Points for a matching hearing-impaired flag.
    pub hearing_impaired: Option<i32>,
}
