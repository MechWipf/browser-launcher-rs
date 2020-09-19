use crate::{config, err::LauncherError};
use config::objects::{Match, MatchKind};
use regex::Regex;
use std::process::Command;

pub(crate) fn launch_browser(url: &str) -> Result<(), LauncherError> {
    let config_root = match config::read_config() {
        Err(err_msg) => {
            let msg = "Unable to load config.toml";
            log::error!("{} Err: {}", msg, err_msg);
            panic!("{}", msg);
        }
        Ok(r) => r,
    };

    let mut browsers: Vec<_> = config_root.browser.iter().collect();
    browsers.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));
    browsers.reverse();

    for (_, browser) in browsers {
        if is_match(url, &browser.matching) {
            let browser_exe = browser.path.replacen("{url}", url, 1);
            let args: Vec<_> = browser
                .args
                .iter()
                .map(|a| a.replacen("{url}", url, 1))
                .collect();

            execute(
                &browser_exe,
                &args.iter().map(|s| s.as_ref()).collect::<Vec<&str>>()[..],
            );
            break;
        }
    };

    Ok(())
}

fn is_match(url: &str, matches: &[Match]) -> bool {
    let mut res = false;

    for m in matches {
        res = match m.kind {
            MatchKind::SimpleMatch => match_simple(url, &m.pattern),
            MatchKind::Regex => match_regex(url, &m.pattern),
        };

        if res {
            break;
        }
    }

    res
}

fn match_simple(subject: &str, pattern: &str) -> bool {
    let pattern = format!(
        "^(https?|ftp)://{}.*",
        pattern
            .to_owned()
            .replacen("+", "\\+", 99)
            .replacen("-", "\\-", 99)
            .replacen("?", "\\?", 99)
            .replacen(".", "\\.", 99)
            .replacen("*", ".*", 99)
    );

    match_regex(subject, &pattern)
}

fn match_regex(subject: &str, pattern: &str) -> bool {
    let rx = Regex::new(pattern).expect("Regex failed to compile.");
    rx.is_match(subject)
}

fn execute(exe: &str, args: &[&str]) {
    if let Err(msg) = Command::new(exe).args(args).spawn() {
        log::error!("failed to spawn process {}", msg)
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn test_patterns_simple() {
        assert!(super::match_simple("https://www.example.com", "*"));

        assert!(super::match_simple(
            "https://www.example.com",
            "*.example.com"
        ));
        assert!(super::match_simple(
            "http://www.example.com",
            "*.example.com"
        ));
        assert!(!super::match_simple(
            "http://www.example.com",
            "example.com"
        ));
        assert!(super::match_simple("http://example.com", "example.com"));
    }

    #[test]
    fn test_patterns_paramter() {
        assert!(super::match_simple(
            "http://store.steampowered.com/app/203770/Crusader_Kings_II/",
            "*.steampowered.com"
        ));
        assert!(!super::match_simple(
            "http://store.steampowered.com/app/203770/Crusader_Kings_II/",
            "*.steamcommunity.com"
        ));
        assert!(super::match_simple(
            "https://steamcommunity.com/sharedfiles/filedetails/?id=1526918750",
            "steamcommunity.com"
        ));
        assert!(super::match_simple(
            "https://steamcommunity.com/sharedfiles/filedetails/?id=1526918750&asd=kas1235",
            "steamcommunity.com"
        ));
    }
}
