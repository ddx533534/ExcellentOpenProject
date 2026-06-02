#[cfg(test)]
mod test {
    use vulkano_lab::vulkano_image::use_vulkano_create_image;

    #[test]
    pub fn try_vulkano() {
        use_vulkano_create_image();
    }
}