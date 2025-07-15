//! Color modifiers
//! These are functions for [`Myrgb`] type used only in this template module, so it doesn't interact with other modules.
use crate::colors::Myrgb;

/// methods used only by the templates
/// darken and Lighten are not here
impl Myrgb {

    /// This outputs `235,235,235` as r,g,b
    pub fn rgb(&self) -> String {
        let (r, g, b) = self.to_rgb8();
        format!("{r},{g},{b}")
    }

    /// HEXA output (e.g. `#001122FF`, where FF is the alpha hex)
    /// Alpha needs to be in hex format alrd
    /// Ref: <https://net-informations.com/q/web/trans.html>
    pub fn hexa(&self, alpha: &str) -> String {
        let (r, g, b) = self.to_rgb8();
        format!("#{r}{g}{b}{alpha}")
    }

    /// .rgba output `235,235,235,1.0`
    pub fn rgba(&self, alpha: f32) -> String {
        let (r, g, b) = self.to_rgb8();
        format!("rgba({r},{g},{b},{alpha})")
    }

    /// xrgba outputs `ee/ee/ee/ff` as r/g/b/alpha in hex but using `/` as a separator
    /// Alpha needs to be in hex format alrd
    pub fn xrgba(&self, alpha: &str) -> String {
        let (r, g, b) = self.to_rgb8();
        format!("{r:02x}/{g:02x}/{b:02x}/{}", alpha.to_ascii_lowercase())
    }

    /// - xrgba outputs `ee/ee/ee/ff` as r/g/b/alpha in hex but using `/` as a separator
    /// - xrgba but without alpha
    /// - alpha is a variable itself, not contained in Colors. so it could be formatted standalone.
    /// > Sample: `{{color0 | xrgb}}{{"/"}}{{alpha_hex}}`
    pub fn xrgb(&self) -> String {
        let (r, g, b) = self.to_rgb8();
        format!("{r:02x}/{g:02x}/{b:02x}")
    }

    /// This only "strips" the `#` from the usual output, leaving the following: `EEEEEE`
    pub fn strip(&self) -> String {
        let (r, g, b) = self.to_rgb8();
        format!("{r:02X}{g:02X}{b:02X}")
    }

    // Red green and blue values as u8s
    // XXX maybe also make red green and blue for hex values?
    pub fn red(&self) -> String {
        let (r, _, _) = self.to_rgb8();
        format!("{r}")
    }
    pub fn green(&self) -> String {
        let (_, g, _) = self.to_rgb8();
        format!("{g}")
    }
    pub fn blue(&self) -> String {
        let (_, _, b) = self.to_rgb8();
        format!("{b}")
    }

    pub fn rgbf(&self) -> String {
        let (r, g, b) = self.0.into_components();
        format!("{r:.4}, {g:.4}, {b:.4}")
    }

    pub fn redf(&self) -> String { format!("{:.4}", self.0.red) }
    pub fn bluef(&self) -> String { format!("{:.4}", self.0.blue) }
    pub fn greenf(&self) -> String { format!("{:.4}", self.0.green) }

}
