mod streaming_library_service;

/// A a streamable video.
#[derive(Debug, PartialEq)]
pub struct VideoModel {
    /// The user-friendly display name of the video.
    display_name: String,
    /// The representation in bytes of the title's image.
    img_bytes: Vec<u8>,
}

/// A collection is a related set of streaming titles.
#[derive(Debug)]
struct CollectionModel {
    /// The user-friendly display name of the collection.
    display_name: String,
    /// The index of the currently selected video.
    current_video_index: usize,
    /// The streamable titles belonging to this collection.
    videos: Vec<VideoModel>,
}

impl CollectionModel {
    fn current_video(&self) -> &VideoModel {
        self.videos.get(self.current_video_index).unwrap()
    }

    fn next_video(&self) {
        // if self.current_video_index + 1 < self.videos.len() {
        //     self.current_video_index += 1;
        // }
        unimplemented!()
    }
}

/// A set of collections.
#[derive(Debug)]
pub struct LibraryModel {
    /// The index of the currently selected collection.
    current_collection_index: usize,
    /// The set of collections belonging to this library.
    collections: Vec<CollectionModel>,
}

impl LibraryModel {
    pub(crate) fn new() -> Self {
        Self {
            current_collection_index: 0,
            collections: vec![],
        }
    }

    fn current_collection(&self) -> &CollectionModel {
        self.collections.get(self.current_collection_index).unwrap()
    }
}

impl BrowsableLibraryModel for LibraryModel {
    fn next_title(&self) {
        self.current_collection().next_video()
    }

    fn previous_title(&self) {
        println!("Selecting previous title from collection");
    }

    fn next_collection(&self) {
        println!("Selecting next collection from library");
    }

    fn previous_collection(&self) {
        println!("Selecting previous title from collection");
    }

    fn current_title(&self) -> &VideoModel {
        self.current_collection().current_video()
    }
}

/// Interface for library models.
pub trait BrowsableLibraryModel {
    /// Move to the next title in the collection.
    fn next_title(&self);

    /// Move to the previous title in the collection.
    fn previous_title(&self);

    /// Move to the next collection in the library.
    fn next_collection(&self);

    /// Move to the previous collection in the library.
    fn previous_collection(&self);

    /// Get more information about the currently selected title.
    fn current_title(&self) -> &VideoModel;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_populated_library() -> LibraryModel {
        LibraryModel {
            current_collection_index: 0,
            collections: vec![
                CollectionModel {
                    current_video_index: 0,
                    display_name: "Happy Holidays".to_string(),
                    videos: vec![
                        VideoModel {
                            display_name: "Home Alone".to_string(),
                            img_bytes: vec![],
                        },
                        VideoModel {
                            display_name: "The Grinch Who Stole Christmas".to_string(),
                            img_bytes: vec![],
                        },
                    ],
                },
                CollectionModel {
                    current_video_index: 0,
                    display_name: "Pixar".to_string(),
                    videos: vec![
                        VideoModel {
                            display_name: "Soul".to_string(),
                            img_bytes: vec![],
                        },
                        VideoModel {
                            display_name: "Inside Out".to_string(),
                            img_bytes: vec![],
                        },
                        VideoModel {
                            display_name: "Coco".to_string(),
                            img_bytes: vec![],
                        },
                    ],
                },
                CollectionModel {
                    current_video_index: 0,
                    display_name: "Spooky Season".to_string(),
                    videos: vec![VideoModel {
                        display_name: "The Witches".to_string(),
                        img_bytes: vec![],
                    }],
                },
            ],
        }
    }

    #[test]
    fn gets_current_video() {
        // Given a library with some collections and videos
        let library = get_populated_library();

        // When it returns the current selection
        let current_selection = library.current_title();

        // Then it matches the first movie in the first collection
        assert_eq!(
            current_selection,
            library.collections.first().unwrap().videos.first().unwrap()
        );
    }

    // Comment this out for now because I'm thinking stateful is a bad way to go
    #[test]
    #[ignore]
    fn next_video_advances_current_selection() {
        // Given a library with some collections and videos
        let library = get_populated_library();

        // When the collection advances to the next video
        library.next_title();

        // Then the current selection is the second video in the first collection
        assert_eq!(
            library.current_title(),
            library.collections.first().unwrap().videos.get(1).unwrap()
        );
    }

    // if we're at the last video next does nothing

    // prev video
    // if we're at the first video prev does nothing

    // next/prev on collection with one video does nothing
}
