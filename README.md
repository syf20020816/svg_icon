# SVG_ICON

use `nom` to parse svg tag or file

## Example

<?xml version="1.0" encoding="UTF-8"?><svg width="24" height="24" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M24 19V4" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 22L24 19L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M28 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M44 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M20 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M4 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M12 44C16.4183 44 20 40.4183 20 36H4C4 40.4183 7.58172 44 12 44Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M36 38C40.4183 38 44 34.4183 44 30H28C28 34.4183 31.5817 38 36 38Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/></svg>

```html
<?xml version="1.0" encoding="UTF-8"?><svg width="24" height="24" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M24 19V4" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 22L24 19L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M28 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M44 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M20 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M4 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M12 44C16.4183 44 20 40.4183 20 36H4C4 40.4183 7.58172 44 12 44Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M36 38C40.4183 38 44 34.4183 44 30H28C28 34.4183 31.5817 38 36 38Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/></svg>
```

```rust
// parse from file path
let svg1 = Svg::from_path("E:/Rust/try/makepad/Gen-UI/gen/middleware/svg_icon/a.svg").unwrap();
// parse from str
let svg_str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" fill="none"></svg>"#;
let svg2: Svg = svg_str.parse().unwrap();
// use macro
let svg = svg!{
r##"<?xml version="1.0" encoding="UTF-8"?><svg width="24" height="24" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M24 19V4" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 22L24 19L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M28 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M44 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M20 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M4 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M12 44C16.4183 44 20 40.4183 20 36H4C4 40.4183 7.58172 44 12 44Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M36 38C40.4183 38 44 34.4183 44 30H28C28 34.4183 31.5817 38 36 38Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
};
```

### Result

```rust
svg = Svg {
    header: Some(
        Header {
            version: Some(
                "1.0",
            ),
            encoding: Some(
                UTF8,
            ),
        },
    ),
    x: None,
    y: None,
    height: Auto(
        Some(
            24.0,
        ),
    ),
    width: Auto(
        Some(
            24.0,
        ),
    ),
    view_box: Some(
        (
            0,
            0,
            48,
            48,
        ),
    ),
    fill: "none",
    xmlns: "http://www.w3.org/2000/svg",
    children: [
        Path(
            Path {
                common: CommonKVs {
                    x: None,
                    y: None,
                    fill: None,
                    stroke: Some(
                        "#008000",
                    ),
                    stroke_width: Some(
                        4.0,
                    ),
                    stroke_linecap: Some(
                        Round,
                    ),
                    stroke_linejoin: Some(
                        Round,
                    ),
                    stroke_dasharray: None,
                    stroke_dashoffset: None,
                    stroke_opacity: None,
                    stroke_miterlimit: None,
                },
                d: Some(
                    "M24 19V4",
                ),
            },
        ),
        Path(
            Path {
                common: CommonKVs {
                    x: None,
                    y: None,
                    fill: None,
                    stroke: Some(
                        "#008000",
                    ),
                    stroke_width: Some(
                        4.0,
                    ),
                    stroke_linecap: Some(
                        Round,
                    ),
                    stroke_linejoin: Some(
                        Round,
                    ),
                    stroke_dasharray: None,
                    stroke_dashoffset: None,
                    stroke_opacity: None,
                    stroke_miterlimit: None,
                },
                d: Some(
                    "M12 22L24 19L36 16",
                ),
            },
        ),
        Path(
            Path {
                common: CommonKVs {
                    x: None,
                    y: None,
                    fill: None,
                    stroke: Some(
                        "#008000",
                    ),
                    stroke_width: Some(
                        4.0,
                    ),
                    stroke_linecap: Some(
                        Round,
                    ),
                    stroke_linejoin: Some(
                        Round,
                    ),
                    stroke_dasharray: None,
                    stroke_dashoffset: None,
                    stroke_opacity: None,
                    stroke_miterlimit: None,
                },
                d: Some(
                    "M28 30L36 16",
                ),
            },
        ),
        Path(
            Path {
                common: CommonKVs {
                    x: None,
                    y: None,
                    fill: None,
                    stroke: Some(
                        "#008000",
                    ),
                    stroke_width: Some(
                        4.0,
                    ),
                    stroke_linecap: Some(
                        Round,
                    ),
                    stroke_linejoin: Some(
                        Round,
                    ),
                    stroke_dasharray: None,
                    stroke_dashoffset: None,
                    stroke_opacity: None,
                    stroke_miterlimit: None,
                },
                d: Some(
                    "M44 30L36 16",
                ),
            },
        ),
        Path(
            Path {
                common: CommonKVs {
                    x: None,
                    y: None,
                    fill: None,
                    stroke: Some(
                        "#008000",
                    ),
                    stroke_width: Some(
                        4.0,
                    ),
                    stroke_linecap: Some(
                        Round,
                    ),
                    stroke_linejoin: Some(
                        Round,
                    ),
                    stroke_dasharray: None,
                    stroke_dashoffset: None,
                    stroke_opacity: None,
                    stroke_miterlimit: None,
                },
                d: Some(
                    "M20 36L12 22",
                ),
            },
        ),
        Path(
            Path {
                common: CommonKVs {
                    x: None,
                    y: None,
                    fill: None,
                    stroke: Some(
                        "#008000",
                    ),
                    stroke_width: Some(
                        4.0,
                    ),
                    stroke_linecap: Some(
                        Round,
                    ),
                    stroke_linejoin: Some(
                        Round,
                    ),
                    stroke_dasharray: None,
                    stroke_dashoffset: None,
                    stroke_opacity: None,
                    stroke_miterlimit: None,
                },
                d: Some(
                    "M4 36L12 22",
                ),
            },
        ),
        Path(
            Path {
                common: CommonKVs {
                    x: None,
                    y: None,
                    fill: Some(
                        "#008000",
                    ),
                    stroke: Some(
                        "#008000",
                    ),
                    stroke_width: Some(
                        4.0,
                    ),
                    stroke_linecap: Some(
                        Round,
                    ),
                    stroke_linejoin: Some(
                        Round,
                    ),
                    stroke_dasharray: None,
                    stroke_dashoffset: None,
                    stroke_opacity: None,
                    stroke_miterlimit: None,
                },
                d: Some(
                    "M12 44C16.4183 44 20 40.4183 20 36H4C4 40.4183 7.58172 44 12 44Z",
                ),
            },
        ),
        Path(
            Path {
                common: CommonKVs {
                    x: None,
                    y: None,
                    fill: Some(
                        "#008000",
                    ),
                    stroke: Some(
                        "#008000",
                    ),
                    stroke_width: Some(
                        4.0,
                    ),
                    stroke_linecap: Some(
                        Round,
                    ),
                    stroke_linejoin: Some(
                        Round,
                    ),
                    stroke_dasharray: None,
                    stroke_dashoffset: None,
                    stroke_opacity: None,
                    stroke_miterlimit: None,
                },
                d: Some(
                    "M36 38C40.4183 38 44 34.4183 44 30H28C28 34.4183 31.5817 38 36 38Z",
                ),
            },
        ),
    ],
}
```