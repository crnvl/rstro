use web_sys::Document;

pub fn set_meta_tags(document: Document, tag_title: &str, tag_desc: &str, tag_img: &str) {
    let head = document.head().expect("failed to get head element");

    let title = document.create_element("title").expect("failed to create title element");
    title.set_inner_html(tag_title);
    head.append_child(&title).expect("failed to append title element");

    let description = document.create_element("meta").expect("failed to create meta element");
    description.set_attribute("name", "description").expect("failed to set name attribute");
    description.set_attribute("content", tag_desc).expect("failed to set content attribute");
    head.append_child(&description).expect("failed to append meta element");

    let og_type = document.create_element("meta").expect("failed to create meta element");
    og_type.set_attribute("property", "og:type").expect("failed to set property attribute");
    og_type.set_attribute("content", "website").expect("failed to set content attribute");
    head.append_child(&og_type).expect("failed to append meta element");

    let og_url = document.create_element("meta").expect("failed to create meta element");
    og_url.set_attribute("property", "og:url").expect("failed to set property attribute");
    og_url.set_attribute("content", "https://social.qwq.sh/").expect("failed to set content attribute");
    head.append_child(&og_url).expect("failed to append meta element");

    let og_title = document.create_element("meta").expect("failed to create meta element");
    og_title.set_attribute("property", "og:title").expect("failed to set property attribute");
    og_title.set_attribute("content", tag_title).expect("failed to set content attribute");
    head.append_child(&og_title).expect("failed to append meta element");

    let og_description = document.create_element("meta").expect("failed to create meta element");
    og_description.set_attribute("property", "og:description").expect("failed to set property attribute");
    og_description.set_attribute("content", tag_desc).expect("failed to set content attribute");
    head.append_child(&og_description).expect("failed to append meta element");

    let og_image = document.create_element("meta").expect("failed to create meta element");
    og_image.set_attribute("property", "og:image").expect("failed to set property attribute");
    og_image.set_attribute("content", tag_img).expect("failed to set content attribute");
    head.append_child(&og_image).expect("failed to append meta element");

    let twitter_card = document.create_element("meta").expect("failed to create meta element");
    twitter_card.set_attribute("property", "twitter:card").expect("failed to set property attribute");
    twitter_card.set_attribute("content", "summary_large_image").expect("failed to set content attribute");
    head.append_child(&twitter_card).expect("failed to append meta element");

    let twitter_url = document.create_element("meta").expect("failed to create meta element");
    twitter_url.set_attribute("property", "twitter:url").expect("failed to set property attribute");
    twitter_url.set_attribute("content", "https://social.qwq.sh/").expect("failed to set content attribute");
    head.append_child(&twitter_url).expect("failed to append meta element");

    let twitter_title = document.create_element("meta").expect("failed to create meta element");
    twitter_title.set_attribute("property", "twitter:title").expect("failed to set property attribute");
    twitter_title.set_attribute("content", tag_title).expect("failed to set content attribute");
    head.append_child(&twitter_title).expect("failed to append meta element");

    let twitter_description = document.create_element("meta").expect("failed to create meta element");
    twitter_description.set_attribute("property", "twitter:description").expect("failed to set property attribute");
    twitter_description.set_attribute("content", tag_desc).expect("failed to set content attribute");
    head.append_child(&twitter_description).expect("failed to append meta element");

    let twitter_image = document.create_element("meta").expect("failed to create meta element");
    twitter_image.set_attribute("property", "twitter:image").expect("failed to set property attribute");
    twitter_image.set_attribute("content", tag_img).expect("failed to set content attribute");
    head.append_child(&twitter_image).expect("failed to append meta element");

}