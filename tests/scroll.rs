use std::ops::RangeInclusive;

mod helpers;

#[test]
fn scroll_regions() {
    helpers::fixture("decstbm");
}

#[test]
fn origin_mode() {
    helpers::fixture("origin_mode");
}

#[test]
fn scrollback() {
    let mut parser = vt100::Parser::new(24, 80, 10);

    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24"
    );

    parser.process(b"\r\n25\r\n26\r\n27\r\n28\r\n29\r\n30");
    assert_eq!(
        parser.screen().contents(),
        "7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30"
    );

    parser.screen_mut().set_scrollback(0);
    assert_eq!(parser.screen().scrollback(), 0);
    assert_eq!(
        parser.screen().contents(),
        "7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30"
    );

    parser.screen_mut().set_scrollback(1);
    assert_eq!(parser.screen().scrollback(), 1);
    assert_eq!(
        parser.screen().contents(),
        "6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29"
    );

    parser.screen_mut().set_scrollback(3);
    assert_eq!(parser.screen().scrollback(), 3);
    assert_eq!(
        parser.screen().contents(),
        "4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27"
    );

    parser.screen_mut().set_scrollback(6);
    assert_eq!(parser.screen().scrollback(), 6);
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24"
    );

    parser.screen_mut().set_scrollback(7);
    assert_eq!(parser.screen().scrollback(), 6);
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24"
    );

    parser.screen_mut().set_scrollback(0);
    assert_eq!(parser.screen().scrollback(), 0);
    assert_eq!(
        parser.screen().contents(),
        "7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30"
    );

    parser.screen_mut().set_scrollback(7);
    assert_eq!(parser.screen().scrollback(), 6);
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24"
    );

    parser.process(b"\r\n31");
    assert_eq!(parser.screen().scrollback(), 7);
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24"
    );

    parser.process(b"\r\n32");
    assert_eq!(parser.screen().scrollback(), 8);
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24"
    );

    parser.process(b"\r\n33");
    assert_eq!(parser.screen().scrollback(), 9);
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24"
    );

    parser.process(b"\r\n34");
    assert_eq!(parser.screen().scrollback(), 10);
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24"
    );

    parser.process(b"\r\n35");
    assert_eq!(parser.screen().scrollback(), 10);
    assert_eq!(
        parser.screen().contents(),
        "2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25"
    );

    parser.process(b"\r\n36");
    assert_eq!(parser.screen().scrollback(), 10);
    assert_eq!(
        parser.screen().contents(),
        "3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26"
    );

    parser.screen_mut().set_scrollback(12);
    assert_eq!(parser.screen().scrollback(), 10);
    assert_eq!(
        parser.screen().contents(),
        "3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26"
    );

    parser.screen_mut().set_scrollback(0);
    assert_eq!(parser.screen().scrollback(), 0);
    assert_eq!(
        parser.screen().contents(),
        "13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30\n31\n32\n33\n34\n35\n36"
    );

    parser.process(b"\r\n37\r\n38");
    assert_eq!(parser.screen().scrollback(), 0);
    assert_eq!(
        parser.screen().contents(),
        "15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30\n31\n32\n33\n34\n35\n36\n37\n38"
    );

    parser.screen_mut().set_scrollback(5);
    assert_eq!(parser.screen().scrollback(), 5);
    assert_eq!(
        parser.screen().contents(),
        "10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30\n31\n32\n33"
    );

    parser.process(b"\r\n39\r\n40");
    assert_eq!(parser.screen().scrollback(), 7);
    assert_eq!(
        parser.screen().contents(),
        "10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30\n31\n32\n33"
    );
}

#[test]
fn edge_of_screen() {
    let mut parser = vt100::Parser::default();
    let screen = parser.screen().clone();

    parser.process(b"\x1b[31m\x1b[24;75Hfooba\x08r\x08\x1b[1@a");
    assert_eq!(parser.screen().cursor_position(), (23, 79));
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n                                                                          foobar"
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        &b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[24;75H\x1b[31mfoobar\x1b[24;80H"[..]
    );
    assert_eq!(
        parser.screen().contents_diff(&screen),
        b"\x1b[24;75H\x1b[31mfoobar\x1b[24;80H"
    );
}

#[test]
fn scrollback_larger_than_rows() {
    let mut parser = vt100::Parser::new(3, 20, 10);

    parser.process(gen_nums(1..=10, "\r\n").as_bytes());

    // 1. Extra rows returned
    parser.screen_mut().set_scrollback(4);
    assert_eq!(parser.screen().contents(), gen_nums(4..=6, "\n"));

    // 2. Subtraction overflow
    parser.screen_mut().set_scrollback(10);
    assert_eq!(parser.screen().contents(), gen_nums(1..=3, "\n"));
}

#[cfg(test)]
fn gen_nums(range: RangeInclusive<u8>, join: &str) -> String {
    range
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(join)
}

#[test]
fn scrollback_row_count() {
    // No scrollback buffer configured
    let mut parser = vt100::Parser::new(3, 20, 0);
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5");
    assert_eq!(parser.screen().scrollback_row_count(), 0);

    // Scrollback buffer configured but not yet filled
    let mut parser = vt100::Parser::new(3, 20, 10);
    parser.process(b"1\r\n2\r\n3");
    assert_eq!(parser.screen().scrollback_row_count(), 0);

    // Scrollback buffer partially filled
    parser.process(b"\r\n4\r\n5");
    assert_eq!(parser.screen().scrollback_row_count(), 2);

    // Scrollback buffer continues to fill
    parser.process(b"\r\n6\r\n7\r\n8");
    assert_eq!(parser.screen().scrollback_row_count(), 5);

    // Scrollback buffer at max capacity
    let mut parser = vt100::Parser::new(3, 20, 5);
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10");
    assert_eq!(parser.screen().scrollback_row_count(), 5);
}

#[test]
fn scrollback_rows() {
    let mut parser = vt100::Parser::new(3, 20, 10);

    // Fill scrollback with some rows
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8");

    // Should have 5 rows in scrollback (rows 1-5), screen shows 6-8
    assert_eq!(parser.screen().scrollback_row_count(), 5);

    // Verify scrollback contents via iterator
    let scrollback: Vec<String> = parser
        .screen()
        .scrollback_rows()
        .map(|row| {
            let mut contents = String::new();
            row.write_contents(&mut contents, 0, 20, false);
            contents.trim_end().to_string()
        })
        .collect();

    assert_eq!(scrollback, vec!["1", "2", "3", "4", "5"]);

    // Verify we can access cells in scrollback rows
    let first_row = parser.screen().scrollback_rows().next().unwrap();
    let cell = first_row.get(0).unwrap();
    assert_eq!(cell.contents(), "1");
}

#[test]
fn clear_scrollback() {
    let mut parser = vt100::Parser::new(3, 20, 10);

    // Fill scrollback
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8");
    assert_eq!(parser.screen().scrollback_row_count(), 5);
    assert_eq!(parser.screen().scrollback(), 0);

    // Set a scrollback offset
    parser.screen_mut().set_scrollback(3);
    assert_eq!(parser.screen().scrollback(), 3);

    // Clear scrollback
    parser.screen_mut().clear_scrollback();

    // Verify scrollback is cleared
    assert_eq!(parser.screen().scrollback_row_count(), 0);
    assert_eq!(parser.screen().scrollback_rows().count(), 0);

    // Verify scrollback offset is also reset
    assert_eq!(parser.screen().scrollback(), 0);

    // Screen contents should be unaffected
    assert_eq!(parser.screen().contents(), "6\n7\n8");
}
