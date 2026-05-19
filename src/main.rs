use std::{fs::File, io::BufWriter};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let re = regex::Regex::new(r"IP address of ([\w.-]+) is ([\d.]+)").unwrap();

    let domains = vec![
        "alive.github.com",
        "live.github.com",
        "github.githubassets.com",
        "central.github.com",
        "desktop.githubusercontent.com",
        "assets-cdn.github.com",
        "camo.githubusercontent.com",
        "github.map.fastly.net",
        "github.global.ssl.fastly.net",
        "gist.github.com",
        "github.io",
        "github.com",
        "github.blog",
        "api.github.com",
        "raw.githubusercontent.com",
        "user-images.githubusercontent.com",
        "favicons.githubusercontent.com",
        "avatars5.githubusercontent.com",
        "avatars4.githubusercontent.com",
        "avatars3.githubusercontent.com",
        "avatars2.githubusercontent.com",
        "avatars1.githubusercontent.com",
        "avatars0.githubusercontent.com",
        "avatars.githubusercontent.com",
        "codeload.github.com",
        "github-cloud.s3.amazonaws.com",
        "github-com.s3.amazonaws.com",
        "github-production-release-asset-2e65be.s3.amazonaws.com",
        "github-production-user-asset-6210df.s3.amazonaws.com",
        "github-production-repository-file-5c1aeb.s3.amazonaws.com",
        "githubstatus.com",
        "github.community",
        "github.dev",
        "collector.github.com",
        "pipelines.actions.githubusercontent.com",
        "media.githubusercontent.com",
        "cloud.githubusercontent.com",
        "objects.githubusercontent.com",
    ];

    let selector = scraper::Selector::parse("#divString0").unwrap();

    let host_file = File::create("hosts").unwrap();
    let mut bufwriter = BufWriter::new(host_file);


    let mut count = (0,0);

    for domain in domains {
        let body_res = reqwest::blocking::get(format!("https://tools.tutorialspoint.com/ip_lookup_ajax.php?host={}",domain))?.text();
        if let Ok(body) = &body_res {
            if let Some(content) = scraper::Html::parse_document(body.as_str()).select(&selector).next() {
                if let Some(domain_ip) = parse_ip_domain(content.text().collect::<String>().as_str(),&re) {
                    writeln!(bufwriter,"{}", format!("{} {}",domain_ip.1,domain_ip.0)).unwrap();
                    count.0+=1;
                }
            }else {
                count.1+=1;
                writeln!(bufwriter,"#fail {}", domain).unwrap();
            }
        }
    }
    bufwriter.flush().unwrap();
    println!("{}ok,{}fail",count.0,count.1);
    Ok(())
}


fn parse_ip_domain<'a>(text: &'a str, re: &regex::Regex) -> Option<(&'a str, &'a str)> {
    if let Some(caps) = re.captures(text) {
        let domain = caps.get(1).unwrap().as_str();
        let ip = caps.get(2).unwrap().as_str();
        Option::Some( (domain ,ip) )
    }else {
        Option::None
    }
}