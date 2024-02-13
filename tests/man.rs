#![cfg(feature = "buildgen")]
#![allow(non_upper_case_globals)]
use clap::{Command, CommandFactory};

/// DESCRIPTION section
const description_main: &str =
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
.RS
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

.TP
.B COLORS
.br
These types are formated like as HEX (e.g. '#0A0B0C')

.BR color0 ,
.BR color1 ,
.BR color2 ,
.BR color3 ,
.BR color4 ,
.BR color5 ,
.BR color6 ,
.BR color7 ,
.BR color8 ,
.BR color9 ,
.BR color10 ,
.BR color11 ,
.BR color12 ,
.BR color13 ,
.BR color14 ,
.BR color15 ,
.BR background ,
.BR foreground " and"
.BR cursor .

.TP
.B MISCELLANEOUS
.RS
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
.B alpha_hex
Displays alpha value as hexadecimal color code, (e.g "FF")
.br
see <https://gist.github.com/lopspower/03fb1cc0ac9f32ef38f4>
.RE

.SH "TEMPLATE FUNCTIONS"

.TP
.B COLORS
.br
Functions that only work with colors. Here
.I
var
represent a color, see TEMPLATE VARIABLES.
.RS
.TP
.B var.rgb
Output the color in `rgb`, separated by comas. (e.g. "10,11,12")
.TP
.B var.rgba
Output the color in `rgba`.
DEPRECATE THIS
.TP
.B var.xrgb
Output the color in `xrgb`, separated by slashes. (e.g "0A/0B/0C")
.TP
.B var.strip
Output the color in `hex`, just like by default, but removes the leading `#`. (e.g. "0A0B0C")
.TP
.B var.red
Outputs only the red value. (e.g. "10")
.TP
.B var.green
Outputs only the green value. (e.g. "11")
.TP
.B var.blue
Outputs only the blue value. (e.g. "12")
.RE

.SH "TEMPLATE SYNTAX"
You reference variables in the following syntax:

.RS
.nf
\fC
{{color0}}
\fP
.fi
.RE

For applying a function (technically named a
.I "filter"
) you apply it like this:

.RS
.nf
\fC
{{background | strip}}
\fP
.fi
.RE

Keep in mind that
.B color functions
require
.B color arguments.

If you need to write a literal `{{`, that doesn't references any variable, you can write literals inside the delimiters:

.RS
.nf
\fC
{{ "{{" }} {{ "}}" }}
\fP
.fi
.RE

The syntax comes from the library being used, which is
.I minijinja
, a subset of the template engine `Jinja2'.

You can read more at:
.I
<https://github.com/mitsuhiko/minijinja/blob/main/COMPATIBILITY.md>


.SH "TEMPLATE EXAMPLE"
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
set default-bg     "{{color2}}"
set default-fg     "{{foreground}}"
set statusbar-bg   "{{color4}}"
set statusbar-fg   "{{color6}}"
set inputbar-bg    "{{color1}}"
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

.SH PYWAL TEMPLATE COMPATIBILITY
You can enable pywal like syntax in the config file with `pywal = true',
see
.BR wallust (5).

.br
The syntax is simple, but more variables are added given that it's engine and spec doesn't support runtime evaluation functions.

.br
.I
While the implementation is simple enough to be added in wallust, it's use is discoraged.

.TP
.B Variables
.BR color0 ,
.BR color1 ,
.BR color2 ,
.BR color3 ,
.BR color4 ,
.BR color5 ,
.BR color6 ,
.BR color7 ,
.BR color8 ,
.BR color9 ,
.BR color10 ,
.BR color11 ,
.BR color12 ,
.BR color13 ,
.BR color14 ,
.BR color15 ,
.BR background ,
.BR foreground ,
.BR cursor ,
and it's
.BR .rgb ,
.BR .rgba ,
.BR .xrgba ,
.BR .strip ,
.BR .red ,
.BR .green " and"
.BR .blue
variants, just append it to the variable name (e.g. "color0.rgb", "background.blue" ...).

.br

.BR wallpaper ,
.BR alpha ,
.BR alpha_dec " and"
.BR alpha_hex
are also avaliable, these don't support the variants from above.

.TP
.B Syntax
.br
The syntax logic is simply "Find and Replace" like:

.RS
.nf
\fC
somevariable = {color2}
anothervariable = {color8.rgb}
\fP
.fi
.RE

For the full pywal spec see
.I <https://github.com/dylanaraps/pywal/wiki/User-Template-Files>
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

const subcommands: &str =
r#"
.SH SUBCOMMANDS
.TP
wallust\-run(1)
Generate a palette from an image
.TP
wallust\-cs(1)
Apply a certain colorscheme
.TP
wallust\-theme(1)
Apply a custom built in theme
.TP
wallust\-migrate
Migrate v2 config to v3
.TP
wallust\-debug
Print information about the program and the enviroment it uses
.TP
wallust\-help
Print this message or the help of the given subcommand(s)
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
    let wallust_v = &format!("wallust-{version}");

    // First the overall man-page: wallust
    let dir = Path::new("./man/");
    let full_name = "wallust.1";
    let mut out = File::create(dir.join(&full_name)).unwrap();
    //let app = cmd.get_subcommands().filter(|x| x.get_name() == "wallust").collect::<Vec<_>>()[0];
    let app = clap_mangen::Man::new(cmd.clone())
        .source(wallust_v);

    // This is the only reason we use clap_mangen, to autogenerate cli flags descriptions.
    // 1. header
    // 2. synopsis
    // 3. description
    // 4. options
    // 4. footer (issues, link, see also..)
    app.render_title(&mut out).unwrap();
    app.render_name_section(&mut out).unwrap();
    app.render_synopsis_section(&mut out).unwrap();
    write!(out, "{description_main}").unwrap();
    app.render_options_section(&mut out).unwrap();
    //app.render_subcommands_section(&mut out).unwrap(); //subcommand only at wallust
    write!(out, "{subcommands}").unwrap();
    write!(out, "{misc_wallust}").unwrap();
    write!(out, "{footer}").unwrap();
    out.flush().unwrap();

    subcmd("wallust-run"  , &cmd, dir, wallust_v, None, Some(footer)).unwrap();
    subcmd("wallust-theme", &cmd, dir, wallust_v, None, Some(footer)).unwrap();
    subcmd("wallust-cs"   , &cmd, dir, wallust_v, None, Some(footer)).unwrap();
}

/// This is the only reason we use clap_mangen, to autogenerate cli flags descriptions.
/// 1. header
/// 2. synopsis
/// 3. description
/// 4. options
/// 4. footer (issues, link, see also..)
fn subcmd(name: &'static str, cmd: &Command, dirout: &std::path::Path, version: &str, description: Option<&str>, foot: Option<&str>) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;
    let manname  = format!("{name}.1");
    let mut out = File::create(dirout.join(&manname))?;
    // renaming the `app` so that SYNOPSIS and other places the program name is `program-subcommand`
    let app = cmd.get_subcommands().find(|&x| x.get_name() == name.split('-').collect::<Vec<&str>>()[1]).unwrap(); // .get_name doesn't use `wallust-theme` but rather just `theme`
    let app = clap_mangen::Man::new(app.clone().name(&name))
        .title(name)
        .manual(name)
        .source(version);

    app.render_title(&mut out)?;
    app.render_name_section(&mut out)?;
    app.render_synopsis_section(&mut out)?;
    if let Some(des) = description { write!(out, "{des}")?; }
    app.render_options_section(&mut out)?;
    if let Some(f) = foot { write!(out, "{f}")?; }

    out.flush()
}
