use std::{borrow::Cow, process::ExitCode};
use arboard::Clipboard;
use std::fmt;
use url::Url;

struct BreadCrumb(Url,String);
struct BreadCrumbList {
    original: Url,
    crumbs: Vec<BreadCrumb>,
}

impl BreadCrumbList {
    fn new(original: Url) -> Self {
        BreadCrumbList {
            original,
            crumbs: vec![],
        }
    }
}

impl fmt::Display for BreadCrumbList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.original.as_str())?;
        Ok(())
    }
}

// let path_to_xlsx = "https://tenant.sharepoint.com/:x:/r/sites/AAAServer/Shared%20Documents/%E6%9C%80%E4%B8%8A%E4%BD%8D%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E3%81%9D%E3%81%AE%E4%B8%8B%E3%81%AE%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E6%9B%B4%E3%81%AB%E3%81%9D%E3%81%AE%E4%B8%8B/%E8%A1%A8%E8%A8%88%E7%AE%97.xlsx/d=we21a3fac93275f3e9339ea4257ff9e8c&csf=1&web=1&e=gaDicy";
// let path_to_org_mut_xlsx = "https://tenant.sharepoint.com/:x:/s/AAAServer/EaW_GuInkz5fkznqQlf_nowB1LhlwROFhtXyXBjt7N-XA?e=3esCZI";
// let path_to_folder = "https://tenant.sharepoint.com/:f:/r/sites/AAAServer/Shared%20Documents/%E6%9C%80%E4%B8%8A%E4%BD%8D%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E3%81%9D%E3%81%AE%E4%B8%8B%E3%81%AE%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E6%9B%B4%E3%81%AB%E3%81%9D%E3%81%AE%E4%B8%8B?csf=1&web=1&e=a9JxgE";
// let path_to_pptx = "https://tenant.sharepoint.com/:p:/r/sites/AAAServer/Shared%20Documents/%E6%9C%80%E4%B8%8A%E4%BD%8D%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E3%81%9D%E3%81%AE%E4%B8%8B%E3%81%AE%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E6%9B%B4%E3%81%AB%E3%81%9D%E3%81%AE%E4%B8%8B/%E8%A1%A8%E8%A8%88%E7%AE%97.pptx/d=we21a3fac93275f3e9339ea4257ff9e8c&csf=1&web=1&e=gaDicy";
// let path_to_pdf = "https://tenant.sharepoint.com/:b:/r/sites/AAAServer/Shared%20Documents/%E6%9C%80%E4%B8%8A%E4%BD%8D%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E3%81%9D%E3%81%AE%E4%B8%8B%E3%81%AE%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E6%9B%B4%E3%81%AB%E3%81%9D%E3%81%AE%E4%B8%8B/%E8%A1%A8%E8%A8%88%E7%AE%97.pdf/d=we21a3fac93275f3e9339ea4257ff9e8c&csf=1&web=1&e=gaDicy";
// let onedrive_path = "https://tenant-my.sharepoint.com/:p:/g/personal/aaaabbbb_tenant_com/Eq-Lh2ocHvBaruDafMI/EedAb1ndgjsadfjfew?e=p9yjaf";


fn main() -> ExitCode {
    // let mut clipboard = Clipboard::new().unwrap();
    // let s = clipboard.get_text().unwrap();
    // println!("Clipboard test was: {}",&s);

    // let html_text = format!(r#"<html><body><a href="{}">{}</a></body></html>"#,s,s);
    // println!("{html_text}");
    // clipboard.set_html(html_text,None).unwrap();
    // println!("Clipboard test is now: {}",clipboard.get_text().unwrap());

    let path_to_xlsx:String = "https://tenant.sharepoint.com/:x:/r/sites/AAAServer/Shared%20Documents/%E6%9C%80%E4%B8%8A%E4%BD%8D%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E3%81%9D%E3%81%AE%E4%B8%8B%E3%81%AE%E3%83%95%E3%82%A9%E3%83%AB%E3%83%80/%E6%9B%B4%E3%81%AB%E3%81%9D%E3%81%AE%E4%B8%8B/%E8%A1%A8%E8%A8%88%E7%AE%97.xlsx/d=we21a3fac93275f3e9339ea4257ff9e8c&csf=1&web=1&e=gaDicy".into();
    let s = path_to_xlsx;
    let url = Url::parse(&s).unwrap();
    let bread_crumbs = BreadCrumbList::new(url);

    println!("{}",bread_crumbs);

    // let path_segments = url.path_segments().unwrap();
    // let path_segments:Vec<&str> = path_segments.collect();
    // dbg!(path_segments);

    ExitCode::SUCCESS
}

