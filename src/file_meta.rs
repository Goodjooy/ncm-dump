use std::{
    io::{Read, Write}, marker::PhantomData,
};

use id3::{Version, TagLike};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AlbumPicDocId {
    Str(String),
    Num(i32),
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Mp3,
    Flac,
}

impl Format {
    pub(crate) fn to_extension(&self) -> &'static str {
        match self {
            Format::Mp3 => "mp3",
            Format::Flac => "flac",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInfo {
    // MusicID       int             `json:"musicId"`
    pub music_id: i32,
    // MusicName     string          `json:"musicName"`
    pub music_name: String,
    // Artist        [][]interface{} `json:"artist"` // [[string,int],]
    pub artist: Vec<(String, i32)>,
    // AlbumID       int             `json:"albumId"`
    pub album_id: i32,
    // Album         string          `json:"album"`
    pub album: String,
    // AlbumPicDocID interface{}     `json:"albumPicDocId"` // string or int
    pub album_pic_doc_id: AlbumPicDocId,
    // AlbumPic      string          `json:"albumPic"`
    pub album_pic: String,
    // BitRate       int             `json:"bitrate"`
    pub bitrate: i32,
    // Mp3DocID      string          `json:"mp3DocId"`
    pub mp3_doc_id: String,
    // Duration      int             `json:"duration"`
    pub duration: i32,
    // MvID          int             `json:"mvId"`
    pub mv_id: i32,
    // Alias         []string        `json:"alias"`
    pub alias: Vec<String>,
    // TransNames    []interface{}   `json:"transNames"`
    pub trans_names: Vec<Value>,
    // Format        string          `json:"format"`
    pub format: Format,
    #[serde(skip)]
    pub(crate) __phantom: PhantomData<()>,
}
pub(crate) trait FileMeta {
    type Error;
    fn set_meta<F: Write + Read>(
        file: &mut F,
        image: Option<&[u8]>,
        meta: MetaInfo,
    ) -> Result<(), Self::Error>;
}
pub struct Mp3;

impl FileMeta for Mp3 {
    type Error = id3::Error;

    fn set_meta<F: Write + Read>(
        file: &mut F,
        image: Option<&[u8]>,
        meta: MetaInfo,
    ) -> Result<(), Self::Error> {
        let mut tag = id3::Tag::read_from(&mut *file)?;

        if let Some(img) = image{
            let _pic_mime = pic_mime(img);
            // TODO: add mp3 tag info
        }
        tag.set_album(meta.album);

        tag.write_to(file, Version::default())?;
        Ok(())
    }
}

fn pic_mime(img:&[u8])->&'static str{
    if img.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]){
        "image/png"
    }else{
        "image/jpeg"
    }
}