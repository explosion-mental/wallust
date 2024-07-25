# Backend

Allows you to choose which method to use in order to parse the image.


| Name | Description |
|------|-------------|
**full**    | Read and return the whole image pixels more precision slower.
**resized** | Resizes the image before parsing mantaining it s aspect ratio.
**wal**     | Uses image magick convert to generate the colors like pywal.
**thumb**   | Faster algo hardcoded to x no ratio respected.
**fastresize** | A much faster resize algo that uses SIMD. Given the different scheme palettes that this uses in contrast with _resized_, is kept as an alternative and not a replacement.
**kmeans**  | Kmeans is an algo that divides and picks pixels all around the image. Requires more tweaking and more in depth testing but for the most part it just werks.


<hr>
To edit this value:
- **Config file**: `backend = "full"`
- **Cli**: `wallust run image.png --backend full`
