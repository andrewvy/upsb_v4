use lazy_static::lazy_static;
use regex::{Captures, Regex};

fn code(input: &str) -> String {
    let mut output = input.to_owned();
    lazy_static! {
        static ref CODE: Regex =
            Regex::new(r"(?s)\[(code|preformatted)\](.*?)\[/(code|preformatted)\]").unwrap();
    }

    output = CODE.replace_all(&output, code_replacer).to_string();
    output
}

fn code_replacer(captures: &Captures) -> String {
    let mut replaced = captures.get(2).unwrap().as_str().to_string();
    for &(input, output) in [("[", "&#91;"), ("]", "&#93;"), ("<br />", "\n")].iter() {
        replaced = replaced.replace(&input, &output);
    }
    format!("<pre><code>{}</code></pre>", replaced)
}

pub fn patterns() -> &'static [(Regex, &'static str); 28] {
    lazy_static! {
      static ref  PATTERNS: [(Regex, &'static str); 28] = [
        // Font changes
        (Regex::new(r"(?s)\[b\](.*?)\[/b\]").unwrap(), "<strong>$1</strong>"),
        (Regex::new(r"(?si)\[i\](.*?)\[/i\]").unwrap(), "<em>$1</em>"),
        (Regex::new(r"(?si)\[u\](.*?)\[/u\]").unwrap(), "<u>$1</u>"),
        (Regex::new(r"(?si)\[s\](.*?)\[/s\]").unwrap(), "<strike>$1</strike>"),
        (Regex::new(r"(?si)\[size=(\d+)](.*?)\[/size\]").unwrap(),
          "<span style=\"font-size: ${1}px;\">$2</span>"),
        (Regex::new(r#"(?si)\[color="?(#[a-zA-Z0-9]+)"?](.*?)\[/color\]"#).unwrap(),
          "<span style=\"color: $1;\">$2</span>"),
        // Alignment
        (Regex::new(r"(?si)\[center\](.*?)\[/center\]").unwrap(),
          "<div style=\"text-align: center;\">$1</div>"),
        (Regex::new(r"(?si)\[left\](.*?)\[/left\]").unwrap(),
          "<div style=\"text-align: left;\">$1</div>"),
        (Regex::new(r"(?si)\[right\](.*?)\[/right\]").unwrap(),
          "<div style=\"text-align: right;\">$1</div>"),
        // Tables
        (Regex::new(r"(?si)\[table\](.*?)\[/table\]").unwrap(), "<table>$1</table>"),
        (Regex::new(r"(?si)\[td\](.*?)\[/td\]").unwrap(), "<td>$1</td>"),
        (Regex::new(r"(?si)\[tr\](.*?)\[/tr\]").unwrap(), "<tr>$1</tr>"),
        (Regex::new(r"(?si)\[th\](.*?)\[/th\]").unwrap(), "<th>$1</th>"),
        // Links
        (Regex::new(r"(?si)\[url\](.*?)\[/url\]").unwrap(),
          "<a href=\"$1\" rel=\"nofollow\" target=\"_new\">$1</a>"),
        (Regex::new(r#"(?si)\[url="?([^\]]+)\]"?(.*?)\[/url\]"#).unwrap(),
          "<a href=\"$1\" rel=\"nofollow\" target=\"_new\">$2</a>"),
        // Quotes
        (Regex::new(r"(?si)\[quote\](.*?)\[/quote\]").unwrap(),
          "<blockquote class=\"quote\">$1</blockquote>"),
        (Regex::new(r"(?si)\[mention=(.+)\](.*?)\[/mention\]").unwrap(), "<span class=\"mention\">@$2</span>"),
        (Regex::new(r"(?si)\[quote=(.+)(;\d+)\](.*?)\[/quote\]").unwrap(),
          "<blockquote class=\"quote\"><strong>$1 wrote:</strong>\n$3</blockquote>"),
        // Images
        (Regex::new(r"(?si)\[img=(\d+)x(\d+)(\b.*)?\](.*?)\[/img\]").unwrap(),
          "<img src=\"$4\" width=\"$1\" height=\"$2\"$3 />"),
        (Regex::new(r"(?si)\[img=(.+)(\b.*)?\](.*?)\[/img\]").unwrap(),
          "<img src=\"$3\" alt=\"$1\"$2 />"),
        (Regex::new(r"(?si)\[img?\](.*?)\[/img\]").unwrap(),
          "<img src=\"$1\" />"),
        // Lists
        (Regex::new(r"(?si)\[ol\](.*?)\[/ol\]").unwrap(), "<ol>$1</ol>"),
        (Regex::new(r"(?si)\[ul\](.*?)\[/ul\]").unwrap(), "<ul>$1</ul>"),
        (Regex::new(r"(?si)\[list\](.*?)\[/list\]").unwrap(), "<ul>$1</ul>"),
        // Youtube
        (Regex::new(r"(?si)\[youtube\](.*?)\[/youtube\]").unwrap(),
          "<object data=\"http://www.youtube.com/embed/$1\"></object>"),
        (Regex::new(r"(?si)\[video\](.*?)\[/video\]").unwrap(),
          "<object data=\"http://www.youtube.com/embed/$1\"></object>"),
        (Regex::new(r"(?si)\[youtube=(\d+)x(\d+)\](.*?)\[/youtube\]").unwrap(),
        "<object width=\"$1\" height=\"$2\" data=\"http://www.youtube.com/embed/$3\"></object>"),
        // List Items
        (Regex::new(r"(?si)\[li\](.*?)\[/li\]").unwrap(), "<li>$1</li>"),
        ];

    }
    &PATTERNS
}

pub fn str_to_html(input: &str) -> String {
    let mut output = code(&input);
    for &(ref pattern, replace) in patterns() {
        output = pattern.replace_all(&output, replace).into_owned();
    }
    output
}
