// Based on colors in https://github.com/Mofiqul/vscode.nvim

use std::collections::HashMap;

use masonry::Color;

pub fn get_colors() -> Vec<(&'static str, Color)> {
    let mut colormap = HashMap::new();
    colormap.insert("vscFront", "#D4D4D4");
    colormap.insert("vscBack", "#1F1F1F");

    colormap.insert("vscTabCurrent", "#1F1F1F");
    colormap.insert("vscTabOther", "#2D2D2D");
    colormap.insert("vscTabOutside", "#252526");

    colormap.insert("vscLeftDark", "#252526");
    colormap.insert("vscLeftMid", "#373737");
    colormap.insert("vscLeftLight", "#636369");

    colormap.insert("vscPopupFront", "#BBBBBB");
    colormap.insert("vscPopupBack", "#272727");
    colormap.insert("vscPopupHighlightBlue", "#004b72");
    colormap.insert("vscPopupHighlightGray", "#343B41");

    colormap.insert("vscSplitLight", "#898989");
    colormap.insert("vscSplitDark", "#444444");
    colormap.insert("vscSplitThumb", "#424242");

    colormap.insert("vscCursorDarkDark", "#222222");
    colormap.insert("vscCursorDark", "#51504F");
    colormap.insert("vscCursorLight", "#AEAFAD");
    colormap.insert("vscSelection", "#264F78");
    colormap.insert("vscLineNumber", "#5A5A5A");

    colormap.insert("vscDiffRedDark", "#4B1818");
    colormap.insert("vscDiffRedLight", "#6F1313");
    colormap.insert("vscDiffRedLightLight", "#FB0101");
    colormap.insert("vscDiffGreenDark", "#373D29");
    colormap.insert("vscDiffGreenLight", "#4B5632");
    colormap.insert("vscSearchCurrent", "#515c6a");
    colormap.insert("vscSearch", "#613315");

    colormap.insert("vscGitAdded", "#81b88b");
    colormap.insert("vscGitModified", "#e2c08d");
    colormap.insert("vscGitDeleted", "#c74e39");
    colormap.insert("vscGitRenamed", "#73c991");
    colormap.insert("vscGitUntracked", "#73c991");
    colormap.insert("vscGitIgnored", "#8c8c8c");
    colormap.insert("vscGitStageModified", "#e2c08d");
    colormap.insert("vscGitStageDeleted", "#c74e39");
    colormap.insert("vscGitConflicting", "#e4676b");
    colormap.insert("vscGitSubmodule", "#8db9e2");

    colormap.insert("vscContext", "#404040");
    colormap.insert("vscContextCurrent", "#707070");

    colormap.insert("vscFoldBackground", "#202d39");

    colormap.insert("vscGray", "#808080");
    colormap.insert("vscViolet", "#646695");
    colormap.insert("vscBlue", "#569CD6");
    colormap.insert("vscAccentBlue", "#4FC1FF");
    colormap.insert("vscDarkBlue", "#223E55");
    colormap.insert("vscMediumBlue", "#18a2fe");
    colormap.insert("vscDisabledBlue", "#729DB3");
    colormap.insert("vscLightBlue", "#9CDCFE");
    colormap.insert("vscGreen", "#6A9955");
    colormap.insert("vscBlueGreen", "#4EC9B0");
    colormap.insert("vscLightGreen", "#B5CEA8");
    colormap.insert("vscRed", "#F44747");
    colormap.insert("vscOrange", "#CE9178");
    colormap.insert("vscLightRed", "#D16969");
    colormap.insert("vscYellowOrange", "#D7BA7D");
    colormap.insert("vscYellow", "#DCDCAA");
    colormap.insert("vscDarkYellow", "#FFD602");
    colormap.insert("vscPink", "#C586C0");
    colormap.insert("vscDimHighlight", "#51504F");

    let colormap: HashMap<_, _> = colormap
        .iter()
        .map(|(&i, v)| (i, Color::parse(v).unwrap()))
        .collect();

    vec![
        ("error", colormap["vscRed"]),
        ("punctuation.bracket", colormap["vscFront"]),
        ("punctuation.special", colormap["vscFront"]),
        ("punctuation.delimiter", colormap["vscFront"]),
        ("comment", colormap["vscGreen"]),
        ("comment.note", colormap["vscBlueGreen"]),
        ("comment.warning", colormap["vscYellowOrange"]),
        ("comment.error", colormap["vscRed"]),
        ("constant", colormap["vscAccentBlue"]),
        ("constant.builtin", colormap["vscBlue"]),
        ("constant.macro", colormap["vscBlueGreen"]),
        ("string.regexp", colormap["vscOrange"]),
        ("string", colormap["vscOrange"]),
        ("character", colormap["vscOrange"]),
        ("number", colormap["vscLightGreen"]),
        ("number.float", colormap["vscLightGreen"]),
        ("boolean", colormap["vscBlue"]),
        ("annotation", colormap["vscYellow"]),
        ("attribute", colormap["vscYellow"]),
        ("attribute.builtin", colormap["vscBlueGreen"]),
        ("module", colormap["vscBlueGreen"]),
        ("function", colormap["vscYellow"]),
        ("function.builtin", colormap["vscYellow"]),
        ("function.macro", colormap["vscYellow"]),
        ("function.method", colormap["vscYellow"]),
        ("variable", colormap["vscLightBlue"]),
        ("variable.builtin", colormap["vscBlue"]),
        ("variable.parameter", colormap["vscLightBlue"]),
        ("variable.parameter.reference", colormap["vscLightBlue"]),
        ("variable.member", colormap["vscLightBlue"]),
        ("property", colormap["vscLightBlue"]),
        ("constructor", colormap["vscBlue"]),
        ("label", colormap["vscLightBlue"]),
        ("keyword", colormap["vscBlue"]),
        ("keyword.conditional", colormap["vscPink"]),
        ("keyword.repeat", colormap["vscPink"]),
        ("keyword.return", colormap["vscPink"]),
        ("keyword.exception", colormap["vscPink"]),
        ("keyword.import", colormap["vscPink"]),
        ("operator", colormap["vscFront"]),
        ("type", colormap["vscBlueGreen"]),
        ("type.qualifier", colormap["vscBlue"]),
        ("structure", colormap["vscLightBlue"]),
        ("tag", colormap["vscBlue"]),
        ("tag.builtin", colormap["vscBlue"]),
        ("tag.delimiter", colormap["vscGray"]),
        ("tag.attribute", colormap["vscLightBlue"]),
    ]
}
