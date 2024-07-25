# Threshold

Wallust automatically uses the **best threshold**, heuristically, if this
variable isn't defined (default behaviour).

>    If you really want to define this variable, keep in mind the
>    following. Thershold is the **difference between similar colors** ,
>    used inside the colorspace.
>
>    **Each colorspace may have different results**
>    with different thresholds, meaning _you should try which one works for you best_.

An **overall** table looks like this:

| Number | Description |
|--------|-------------|
1        | Not Perceptible by human eyes.
1 - 2   | Perceptible through close observation.
2 - 10  | Perceptible at a glance.
11 - 49 | Colors are more similar than opposite.
100      | Colors are exact opposite.

<hr>

To edit this value:
- **Config file**: `threshold = 10`
- **Cli**: `wallust run image.png --threshold 18`
