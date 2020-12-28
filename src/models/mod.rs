/// A title is a streamable video.
struct Title {
    /// The representation in bytes of the title's image.
    img_bytes: Vec<u8>,
}

/// A collection is a related set of streaming titles.
struct Collection {
    /// The name of the collection for users.
    display_name: String,
    /// The streamable titles belonging to this collection.
    titles: Vec<Title>,
}

/// A set of collections.
struct Library {
    /// The set of collections belonging to this library.
    collections: Vec<Collection>,
}
