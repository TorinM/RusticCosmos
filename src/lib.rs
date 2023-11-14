pub struct RusticCosmos {
    pub name: String,
    pub screen_xsize: f64,
    pub screen_ysize: f64
}
impl RusticCosmos {
    pub fn new(name:String, screen_xsize: f64, screen_ysize: f64) -> RusticCosmos {
        RusticCosmos { name, screen_xsize, screen_ysize }
    }
}