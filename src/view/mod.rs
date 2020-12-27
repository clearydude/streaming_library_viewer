pub(crate) fn render() {
    // RGBA
    unsafe { gl::ClearColor(0.2, 0.3, 0.3, 1.0) }
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }
}
