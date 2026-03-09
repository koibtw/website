const CAT_LEFT: &str = "ᓚ₍ ^. .^₎";
const CAT_RIGHT: &str = "₍^. .^ ₎ᓗ";
const CAT_MID: &str = "";

const MAX_WIDTH: usize = 77;
const PADDING: usize = 2;
const HEADER_LEN: usize = MAX_WIDTH + 2 - 18;

const FG: &str = "\x1b[38;2;234;207;183m";
const FG_ALT: &str = "\x1b[38;2;140;111;83m";
const ACCENT: &str = "\x1b[38;2;175;76;88m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

const BODY: &str = "hello!! welcome to my website :3
this is cool, huh?

my name is june and i write like code and stuff i think?

anyway, i love my amazing girlfriend and also listening to music and stuff
i play cool video games like Deep Rock Galactic,
OneShot, Dead Cells, and Elden Ring!
i also play some guitar and draw, pretty bad at both though :c

oh and btw, i run this site hosting thingy called ebil.club >:3
you should definitely check it out!!

so uhh have a great day? or something";

pub fn render() -> String {
  let header = render_header();
  let footer = render_footer();

  let content = render_body(BODY, MAX_WIDTH, PADDING);

  format!("\n{header}\n{content}\n{footer}\n\n")
}

fn render_header() -> String {
  let mut s = String::new();

  s.push_str(FG);
  s.push_str(BOLD);
  s.push_str(CAT_LEFT);

  for i in 0..HEADER_LEN {
    if i == HEADER_LEN / 2 {
      s.push_str(CAT_MID);
    } else {
      s.push(' ');
    }
  }

  s.push_str(CAT_RIGHT);
  s.push_str(RESET);
  s.push('\n');

  s.push_str(FG_ALT);
  s.push_str(BOLD);
  s.push('╭');
  for _ in 0..MAX_WIDTH {
    s.push('─');
  }
  s.push('╮');
  s.push_str(RESET);

  s
}

fn render_footer() -> String {
  let mut s = String::new();
  s.push_str(FG_ALT);
  s.push_str(BOLD);
  s.push('╰');
  for _ in 0..MAX_WIDTH {
    s.push('─');
  }
  s.push('╯');
  s.push_str(RESET);
  s
}

fn render_body(text: &str, max_width: usize, padding: usize) -> String {
  let inner_width = max_width.saturating_sub(padding);
  let mut out = String::new();

  for raw_line in text.lines() {
    let mut line = raw_line.trim();

    if line.is_empty() {
      push_body_line(&mut out, "", padding);
      continue;
    }

    while line.len() > inner_width {
      let (head, tail) = split_at_wrap_boundary(line, inner_width);
      push_body_line(&mut out, head.trim_end(), padding);
      line = tail.trim_start();
    }

    push_body_line(&mut out, line, padding);
  }

  if out.ends_with('\n') {
    out.pop();
  }

  out
}

fn push_body_line(out: &mut String, content: &str, padding: usize) {
  for _ in 0..padding {
    out.push(' ');
  }
  out.push_str(FG);
  out.push_str(content);
  out.push_str(RESET);
  out.push('\n');
}

fn split_at_wrap_boundary(s: &str, limit: usize) -> (&str, &str) {
  if s.len() <= limit {
    return (s, "");
  }

  let head = &s[..limit];
  if let Some(space_idx) = head.rfind(' ') {
    let left = &s[..space_idx];
    let right = &s[space_idx + 1..];
    (left, right)
  } else {
    let left = &s[..limit];
    let right = &s[limit..];
    (left, right)
  }
}
