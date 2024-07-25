# Fallback Generator

This field chooses a method to use when the gathered colors aren't enough:

| Name | Description |
|------|-------------|
**interpolation** | (default) Tries to pick two colors and built gradients over them
**complementary** | Uses the complementary colors of two colors, or more (if needed), colors.

<hr>

To edit this value:
- **Config file**: `fallback_generator = "complementary"`
- **Cli**: `wallust run image.png --fallback-generator complementary`
