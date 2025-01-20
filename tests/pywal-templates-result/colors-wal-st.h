const char *colorname[] = {

  /* 8 normal colors */
  [0] = "#000000", /* black   */
  [1] = "#010000", /* red     */
  [2] = "#020000", /* green   */
  [3] = "#030000", /* yellow  */
  [4] = "#040000", /* blue    */
  [5] = "#050000", /* magenta */
  [6] = "#060000", /* cyan    */
  [7] = "#070000", /* white   */

  /* 8 bright colors */
  [8]  = "#080000",  /* black   */
  [9]  = "#090000",  /* red     */
  [10] = "#0A0000", /* green   */
  [11] = "#0B0000", /* yellow  */
  [12] = "#0C0000", /* blue    */
  [13] = "#0D0000", /* magenta */
  [14] = "#0E0000", /* cyan    */
  [15] = "#0F0000", /* white   */

  /* special colors */
  [256] = "#EEEEEE", /* background */
  [257] = "#DDDDDD", /* foreground */
  [258] = "#DDDDDD",     /* cursor */
};

/* Default colors (colorname index)
 * foreground, background, cursor */
 unsigned int defaultbg = 0;
 unsigned int defaultfg = 257;
 unsigned int defaultcs = 258;
 unsigned int defaultrcs= 258;
