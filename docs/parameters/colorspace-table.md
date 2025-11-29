| Color Space | Description |
|-------------|-------------|
**Lab** | Uses Cie L*a*b color space
**LabMixed** | Variant of `lab` that mixes the colors gathered, if not enough colors it fallbacks to usual lab (not recommended in small images)
**Lch** | CIE Lch, you can understand this color space like LAB but with chrome and hue added. Could help when sorting.
**LchMixed** | CIE Lch variant that mixed on every similar color.
**Salience** | Differentiates colors by visual **salience**. Salience refers to how much something (a color) pops out from a context (the background). Currently based on CIE Lch.
**LchAnsi** | Variant of Lch which preserves 8 colors: black, red, green, yellow, blue, magenta, cyan and gray. This works best with 'darkansi' palette, allowing a constant color order.
