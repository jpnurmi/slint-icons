use itertools::Itertools;
use std::{fs::File, io::Write, path::PathBuf};

macro_rules! include_code_points {
    ($file:expr) => {
        include_str!(concat!("../fonts/", $file))
    };
}

macro_rules! write_file {
    ($name:expr, $content:expr) => {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui");
        let mut file = File::create(path.join($name)).unwrap();
        write!(file, "{}", $content).unwrap();
    };
}

fn main() {
    write_file!(
        "material/filled.slint",
        generate_icon_font(
            "MaterialIcons",
            IconFont {
                family: "Material Icons",
                file_name: "material/MaterialIcons-Regular.ttf",
                code_points: include_code_points!("material/MaterialIcons-Regular.codepoints"),
            },
        )
    );
    write_file!(
        "material/outlined.slint",
        generate_icon_font(
            "MaterialIconsOutlined",
            IconFont {
                family: "Material Icons Outlined",
                file_name: "material/MaterialIconsOutlined-Regular.otf",
                code_points: include_code_points!(
                    "material/MaterialIconsOutlined-Regular.codepoints"
                ),
            },
        )
    );
    write_file!(
        "material/round.slint",
        generate_icon_font(
            "MaterialIconsRound",
            IconFont {
                family: "Material Icons Round",
                file_name: "material/MaterialIconsRound-Regular.otf",
                code_points: include_code_points!("material/MaterialIconsRound-Regular.codepoints"),
            },
        )
    );
    write_file!(
        "material/sharp.slint",
        generate_icon_font(
            "MaterialIconsSharp",
            IconFont {
                family: "Material Icons Sharp",
                file_name: "material/MaterialIconsSharp-Regular.otf",
                code_points: include_code_points!("material/MaterialIconsSharp-Regular.codepoints"),
            },
        )
    );
    write_file!(
        "material/two_tone.slint",
        generate_icon_font(
            "MaterialIconsTwoTone",
            IconFont {
                family: "Material Icons Two Tone",
                file_name: "material/MaterialIconsTwoTone-Regular.otf",
                code_points: include_code_points!(
                    "material/MaterialIconsTwoTone-Regular.codepoints"
                ),
            },
        )
    );
}

struct IconFont {
    family: &'static str,
    file_name: &'static str,
    code_points: &'static str,
}

fn generate_icon_font(name: &str, font: IconFont) -> String {
    format!(
        "import {{ Icon, IconData, IconTheme }} from \"../icon.slint\";
export {{ Icon, IconData, IconTheme }}

import \"../../fonts/{}\";

export global {} {{
    {}
}}
",
        font.file_name,
        name,
        generate_code_points(&font).iter().join("\n    "),
    )
}

fn generate_code_points(font: &IconFont) -> Vec<String> {
    let mut lines = Vec::new();
    for line in font.code_points.lines() {
        if let Some((icon_name, code_point)) = line.split_whitespace().collect_tuple() {
            if icon_name == "flourescent" {
                continue;
            }
            let fixed_name = fixup_icon_name(icon_name);
            lines.push(format!(
                "out property<IconData> {}: {{code-point: \"\\u{{{}}}\", font-family: \"{}\"}};",
                fixed_name, code_point, font.family,
            ));
        }
    }
    lines
}

fn fixup_icon_name(name: &str) -> String {
    let number = name
        .chars()
        .take_while(|x| x.is_ascii_digit())
        .collect::<String>();

    if number.is_empty() {
        name.replace("_", "-").to_string()
    } else {
        format!(
            "{}-{}",
            match number.as_str() {
                "1" => "one",
                "2" => "two",
                "3" => "three",
                "4" => "four",
                "5" => "five",
                "6" => "six",
                "7" => "seven",
                "8" => "eight",
                "9" => "nine",
                "10" => "ten",
                "11" => "eleven",
                "12" => "twelve",
                "13" => "thirteen",
                "14" => "fourteen",
                "15" => "fifteen",
                "16" => "sixteen",
                "17" => "seventeen",
                "18" => "eighteen",
                "19" => "nineteen",
                "20" => "twenty",
                "21" => "twenty-one",
                "22" => "twenty-two",
                "23" => "twenty-three",
                "24" => "twenty-four",
                "30" => "thirty",
                "123" => "one-twenty-three",
                "60" => "sixty",
                "360" => "three-sixty",
                _ => panic!("{}", number),
            },
            name[number.len()..]
                .replace("_", "-")
                .trim_start_matches('-')
        )
    }
}
