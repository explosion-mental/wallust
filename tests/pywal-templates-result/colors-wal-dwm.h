static const char norm_fg[] = "#0F0000";
static const char norm_bg[] = "#000000";
static const char norm_border[] = "#080000";

static const char sel_fg[] = "#0F0000";
static const char sel_bg[] = "#020000";
static const char sel_border[] = "#0F0000";

static const char urg_fg[] = "#0F0000";
static const char urg_bg[] = "#010000";
static const char urg_border[] = "#010000";

static const char *colors[][3]      = {
    /*               fg           bg         border                         */
    [SchemeNorm] = { norm_fg,     norm_bg,   norm_border }, // unfocused wins
    [SchemeSel]  = { sel_fg,      sel_bg,    sel_border },  // the focused win
    [SchemeUrg] =  { urg_fg,      urg_bg,    urg_border },
};
