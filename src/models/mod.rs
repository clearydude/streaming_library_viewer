/// A a streamable video.
struct VideoModel {
    /// The representation in bytes of the title's image.
    img_bytes: Vec<u8>,
}

/// A collection is a related set of streaming titles.
struct CollectionModel {
    /// The name of the collection for users.
    display_name: String,
    /// The streamable titles belonging to this collection.
    titles: Vec<VideoModel>,
}

/// A set of collections.
pub struct LibraryModel {
    /// The set of collections belonging to this library.
    collections: Vec<CollectionModel>,
}

impl LibraryModel {
    pub(crate) fn new() -> Self {
        Self {
            collections: vec![],
        }
    }
}

impl BrowsableLibraryModel for LibraryModel {
    fn next_title(&self) {
        println!("Selecting next title from collection");
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

    fn current_title(&self) {
        println!("Current title");
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
    fn current_title(&self);
}
