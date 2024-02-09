#![allow(non_upper_case_globals)]
use clap::CommandFactory;

/// DESCRIPTION section
const description: &str =
r#"
.SH "DESCRIPTION"
.TS
tab(;) left,box;

l   |   l |.
\fBMethods\fP;\fBDescription\fP
_
\fBBackends\fP;How to extract the colors from the image (e.g. pywal uses convert).
_
\fBColorspace\fP;Get the most prominent color, and sort them according to the \fBpalette\fP, configurable with a threshold.
_
\fBPalette\fP;Makes a scheme palette with the gathered colors (e.g. sets a light background).
.TE
"#;

/// This goes below options and subcommands, miscellaneous stuff
const misc_wallust: &str = r#"
.SH "TERMINAL COLORS"
By default,
.I wallust
will send these sequences to all open terminals:
.IP \(bu
.I /dev/pts/
on Linux
.IP \(bu
.I /dev/ttys00
on MacOS.
.IP \(bu
.I "ps to search active terminals"
(ref: https://github.com/dylanaraps/pywal/pull/510) on OpenBSD
.IP \(bu
Updates `settings.json` on Windows Terminal, to enable this scheme for the first time you will have to selected it manually

.RE
.sp
You can skip this with the `-s` or `--skip-sequences` flag.
.br
When opening new terminals you will notice that the color sequences are not applied. To solve this you can send the sequences yourself when your shell opens. `wallust` will store the sequences in the cache directory as a file called `sequences`, the usual way is to `cat ~/.cache/wallust/sequences` in your `.zshrc`, `.bashrc`, etc.

.SH "TEMPLATE VARIABLES"
The following are the avaliable variable template names that one can use: where
.B var
can be colors from
.I "color0"
to
.I color15
,
.I background
,
.I foreground
and
.I cursor
.

.TP
.B wallpaper
The full path to the current wallpaper, colorscheme file or the name of the theme in use.
.TP
.B backend
Current backend being used.
.TP
.B colorspace
Current **colorspace** being used.
.TP
.B palette
Current **palette** being used.
.TP
.B alpha
Default to 100, can be modified in the config file or with `--alpha`/`-a`.
.TP
.B alpha_dec
Instead of 0 to 100, displays it from 0.00 to 1.00.
.TP
.B var
Output the color in `hex`.
.TP
.B var.rgb
Output the color in `rgb`.
.TP
.B var.rgba
Output the color in `rgba`.
.TP
.B var.xrgba
Output the color in `xrgb`.
.TP
.B var.strip
Output the color in `hex` (without a `#`).
.TP
.B var.red
Output the red value.
.TP
.B var.green
Output the green value.
.TP
.B var.blue
Output the blue value.

.SH "TEMPLATE EXAMPLES"
You can use
.B wallust
generated colors in a program by
.I templating
the colors in it's config file, like the following example:

.RS
.nf
\fC
# zathurarc config file

#keybindings
...

# colors
set default-bg     "{color2}"
set default-fg     "{foreground}"
set statusbar-bg   "{color4}"
set statusbar-fg   "{color6}"
set inputbar-bg    "{color1}"
\fP
.fi

.RE
.sp
Then you can add this file to
.I ~/.config/wallust/
and use the config file to template it. For example,
.I "zathura.template = ~/.config/wallust/zathurarc"
, and then define a
.I target
field, see
.BR wallust (5).
"#;

/// Usually how to end the man page
const footer:&str = r#"
.SH "SEE ALSO"
.BR wallust (5),
.BR wallust-run (1),
.BR wallust-cs (1),
.BR wallust-theme (1),
.BR wallust-themes [1]
.br
.SH "NOTES"
.nr step 1
.IP " \n+[step]." 4
Suggestions for new colorschemes returned by the
.B themes
subcommand should be filled here.
.RS 4
.I https://codeberg.org/explosion-mental/wallust-themes
.RE
.SH "BUGS"
.I https://codeberg.org/explosion-mental/wallust
"#;

/// Maybe consider making a makefile? (just like the old times :3)
#[test]
fn mk_man() {
    use std::path::Path;
    use std::fs::File;
    use std::io::Write;

    let cmd = wallust::args::Subcmds::command();

    let v = clap::crate_version!().chars().collect::<Vec<_>>();
    let mut version = String::new();
    let mut num = 0;
    for i in v {
        if i == '.' { num += 1; } // count dots
        if num == 2 { break; }    // only allow Major.Minor
        version.push(i);
    }

    // First the overall man-page: wallust
    let dir = Path::new("./man/");
    let full_name = "wallust.1";
    let mut out = File::create(dir.join(&full_name)).unwrap();
    //let app = cmd.get_subcommands().filter(|x| x.get_name() == "wallust").collect::<Vec<_>>()[0];
    let app = clap_mangen::Man::new(cmd.clone())
        .source(format!("wallust-{version}"));

    // This is the only reason we use clap_mangen, to autogenerate cli flags descriptions.
    // 1. header
    // 2. synopsis
    // 3. description
    // 4. options
    // 4. footer (issues, link, see also..)
    app.render_title(&mut out).unwrap();
    app.render_name_section(&mut out).unwrap();
    app.render_synopsis_section(&mut out).unwrap();
    write!(out, "{description}").unwrap();
    app.render_options_section(&mut out).unwrap();
    app.render_subcommands_section(&mut out).unwrap(); //subcommand only at wallust
    write!(out, "{misc_wallust}").unwrap();
    write!(out, "{footer}").unwrap();
    out.flush().unwrap();

    // wallust-theme
    let full_name = "wallust-theme.1";
    let mut out = File::create(dir.join(&full_name)).unwrap();
    let app = cmd.get_subcommands().find(|&x| x.get_name() == "theme").unwrap(); // .get_name doesn't use `wallust-theme` but rather just `theme`
    let app = clap_mangen::Man::new(app.clone())
        .title("wallust-theme")
        .manual("wallust-theme")
        .source(format!("wallust-{version}")) //little string footer at the end
        ;

    app.render_title(&mut out).unwrap();
    app.render_name_section(&mut out).unwrap();
    app.render_synopsis_section(&mut out).unwrap();
    write!(out, "{description}").unwrap();
    app.render_options_section(&mut out).unwrap();
    write!(out, "{footer}").unwrap();
    out.flush().unwrap();

    // wallust-cs
    let full_name = "wallust-cs.1";
    let mut out = File::create(dir.join(&full_name)).unwrap();
    let app = cmd.get_subcommands().find(|&x| x.get_name() == "cs").unwrap(); // .get_name doesn't use `wallust-theme` but rather just `theme`
    let app = clap_mangen::Man::new(app.clone())
        .title("wallust-cs")
        .manual("wallust-cs")
        .source(format!("wallust-{version}"))
        ;

    app.render_title(&mut out).unwrap();
    app.render_name_section(&mut out).unwrap();
    app.render_synopsis_section(&mut out).unwrap();
    write!(out, "{description}").unwrap();
    app.render_options_section(&mut out).unwrap();
    write!(out, "{footer}").unwrap();
    out.flush().unwrap();
}
