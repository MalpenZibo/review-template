use review::Tag::{Img, Main, Span, H1, I};
use review::{children, component, ElementBuilder};

#[component(App)]
pub fn app() -> VNode {
    Main.with_children(children!(
        Img.with_attribute("class", "logo")
            .with_attribute("src", "/assets/logo.png")
            .with_attribute("alt", "reView logo"),
        H1.with_child("Hello World!"),
        Span.with_attribute("class", "subtitle")
            .with_children(children!(
                "from reView with ",
                I.with_attribute("class", "heart")
            ))
    ))
    .into()
}
