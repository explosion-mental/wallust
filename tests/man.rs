#![cfg(feature = "buildgen")]
#![allow(non_upper_case_globals)]
use clap::{Command, CommandFactory};

/// DESCRIPTION section
const description_main: &str =
r#"
.SH "DESCRIPTION"
.ad l
.TS
box tab(!);
cB | cB
lwB | lw.
Methods!Description
_
Backends!T{
How to extract the colors from the image
(e.g. pywal uses convert).
T}
_
Color Space!T{
Get the most prominent color, and sort them
according to the
.B palette
, configurable with a threshold.
T}
_
Palette!T{
Makes a scheme palette with the gathered colors
(e.g. sets a light background).
T}
.TE
.ad b


.B Reminder
The options below can be used after the subcommand, for example:
"wallust --quiet run image.png" is the same as "wallust run --quiet image.png"
"#;

/// This goes below options and subcommands, miscellaneous stuff
const misc_wallust: &str = r##"
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
.BR "ps to search active terminals" [1]
on OpenBSD
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
These types are formated like as HEX rgb (e.g. '#0A0B0C') by default.
However a
.B "color literal"
can be represented in multiple ways, like HEXA rgba (e.g. '#0A0B0CFF', where 'FF' is the
transparency value) or HEX rgb without the leading '#' ('0a0b0c').

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

.B colors
Additionally, this variable returns a vector of all the presented colors in the following order:
starts with
.I color0
to
.IR color15 ,
.IR background ,
.IR foreground and
at the end, (index 18 if starting from 0),
.IR  cursor .
.I See TEMPLATE SYNTAX for a practical guide.

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
.RE

.SH "TEMPLATE FILTERS"
.PP
The Jinja2 format calls them 'filters', making a distincion from 'functions'.
.br
Currently I haven't implemented any
.B function
because I haven't found a usecase (yet?).

.TP
.B UNSIGNED INT
.RS
.TP
.B alpha_hexa
Displays alpha value as
.BR "hexadecimal color code" [2]
(e.g "{{ 100 | alpha_hexa }}" outputs 'FF').
This can only be used with numbers from 0 to 100, so you are free to use the variable
.I alpha
with this filter.
.RE

.TP
.B COLORS
.br
Functions that only work with colors. These can be applied to a
.I color
, which can be the COLOR variables listed above, see TEMPLATE VARIABLES, or a literal color like
"#0A0B0C". These functions return a color in the mentioned format (hex rgb, like "#000000"), unless
written otherwise (like rgb, rgba, the other filters that explicitly say it's output format). This
allows to apply multiple filters at a time.

.I Note
If an 'alpha' value is mentioned, it's defined in the config file, as a cli flag and by default it's value is '100'.
.RS
.TP
.B hexa
Outputs the color in `hexa` format: e.g "#0A0B0CFF", where 'FF' is the alpha value.
.I Note
This, internally uses `alpha_hexa` filter from above.
.TP
.B rgb
Output the color in `rgb`, separated by comas. (e.g. "10,11,12")
.TP
.B xrgb
Output the color in `xrgb`, separated by slashes. (e.g "0A/0B/0C")
.TP
.B strip
Output the color in `hex`, just like by default, but removes the leading `#`. (e.g. "0A0B0C")
.TP
.B red
Outputs only the red value. (e.g. "10")
.TP
.B green
Outputs only the green value. (e.g. "11")
.TP
.B blue
Outputs only the blue value. (e.g. "12")
.TP
.B complementary
Returns the respective complementary color.
.TP
.BI blend " COLOR"
Takes another
.B color
as input, to blend it for the filtered color.
.TP
.BI lighten " amount"
Takes a
.I float
(decimal value) as input,
.B "from 0.1 to 1.0"
, that corresponds to the amount to lighten the color by.
.TP
.BI darken " amount"
Takes a
.I float
(decimal value) as input,
.B "from 0.1 to 1.0"
, that corresponds to the amount to darken the color by.
.TP
.BI saturate " amount"
Takes a
.I float
(decimal value) as input,
.B "from 0.1 to 1.0"
, that corresponds to the amount to saturate the color by.
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

For applying a
.I "filter"
you use the 'pipe character` (|) like this:

.RS
.nf
\fC
{{background | strip}}
\fP
.fi
.RE

And if the filter requires an argument:

.RS
.nf
\fC
{{background | lighten(0.3)}}
\fP
.fi
.RE

Remember that filters require a valid type to
.B "apply to"
in these examples we are using colors, which can even be defined literally:

.RS
.nf
\fC
{{ "#4ff4ff" | lighten(0.3)}}
\fP
.fi
.RE

For
.B both
, being applied to or as an argument of a filter:

.RS
.nf
\fC
{{ color2 | blend("4ff4ff")}}
\fP
.fi
.RE


If you need to write a literal `{{`, that doesn't references any variable, you can write literals inside the delimiters:

.RS
.nf
\fC
{{ "{{" }} {{ "}}" }}
\fP
.fi
.RE

You can also use control flow expressions with `{% %}` delimiters:

.RS
.nf
\fC
{% if backend == "wal" %}
I am using the '{{backend}}' backend, getting a pywal like scheme.
{% elif backend == "fastresize" %}
This backend is called "{{palette}}" and, uses SIMD optimizations and is so fast!
{% else %}
I don't care about any other backends. Be happy!
{% endif %}
\fP
.fi
.RE

Or inline them:

.RS
.nf
\fC
{{ "I'm using the kmeans algo!" if backend == "kmeans" else "Some backend is in use" }}
\fP
.fi
.RE

Since mostly everything can be represented as a string (we've seen how colors are represented),
indexing results very useful! The syntax for indexing is basically the Python one.

.RS
.nf
\fC
{# I'll hardcode a color based on the palette being used. #}
{% if palette[:4] == "dark" %}
somevariable = "#eeffbb"
{% else %}
somevariable = "#aabbee"
{% endif %}
\fP
.fi
.RE

And yes, you can comment inside your template, the comments won't be rendered in the final target file:

.RS
.nf
\fC
{# This won't be visible! #}
\fP
.fi
.RE

There are more control flow instructions, like the for loop:

.RS
.nf
\fC
{# This will generate color0 = .. to color18,
since `colors` contains background, foreground and cursor variables #}
{% for c in colors %}
color{{- loop.index }} = {{c-}}
{% endfor %}
\fP
.fi
.RE

You can add a minus sign (-) at the start or the end of the delimiters to supress
.BR "vertical spacing" [3]

The syntax comes from the library being used, which is
.I minijinja
, a subset of the template engine `Jinja2'.

You can read more at:
.BR "Jinja2 official syntax" [4]
and contrast features with the supported syntax at
.BR "Compatibility of minijinja" [5]

.SH "TEMPLATE EXAMPLE"
You can use
.B wallust
generated colors in a program by
.I templating
the colors in it's config file, like the following example:

.RS
.nf
\fC
# zathurarc config sample
...

# colors
set default-bg     "{{background}}"
set default-fg     "{{foreground}}"

# make it a bit lighter than background
set statusbar-bg   "{{background | lighten(0.3)}}"

# make it darken by blending to a darken color
set statusbar-fg   "{{foreground | blend("#eeeeee")}}"

# use it's complementary
set inputbar-bg    "{{background | complementary}}"
\fP
.fi

.RE
.sp
Then you can add this file to
.I ~/.config/wallust/templates
and use the config file to template it. For example,
.I "zathura.template = 'zathurarc'"
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
.BR alpha " and"
.BR alpha_dec
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

Don't forget to visit the
.BR "full pywal spec" [6]
"##;


/// This is specialized footer for wallust.1
const footer_wallust:&str = r#"
.SH "SEE ALSO"
.BR wallust (5),
.BR wallust-run (1),
.BR wallust-cs (1),
.BR wallust-theme (1),
.BR wallust-themes [7].
.br

.SH "NOTES"
.nr step 1

.TP 4
.B "1."
ps to search active terminals
.br
.I https://github.com/dylanaraps/pywal/pull/510

.TP 4
.B "2."
Hexadecimal color code
.br
.I https://gist.github.com/lopspower/03fb1cc0ac9f32ef38f4

.TP 4
.B "3."
White space contron with the minus sign (-)
.br
.I http://jinja.pocoo.org/docs/templates/#whitespace-control

.TP 4
.B "4."
Official Jinja2 documentation
.br
.I https://jinja.palletsprojects.com/en/2.10.x/

.TP 4
.B "5."
Compatibility of Minijinja with Jinja2
.br
.I https://github.com/mitsuhiko/minijinja/blob/main/COMPATIBILITY.md

.TP 4
.B "6."
Full pywal template specification
.br
.I https://github.com/dylanaraps/pywal/wiki/User-Template-Files

.TP 4
.B "7."
Suggestions for new colorschemes returned by the
.B themes
subcommand should be filled here.
.br
.I https://codeberg.org/explosion-mental/wallust-themes

.SH "BUGS"
.I https://codeberg.org/explosion-mental/wallust
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

    let cmd = wallust::args::Cli::command();

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
    write!(out, "{footer_wallust}").unwrap();
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
