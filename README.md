# YUNG's Better Witch Huts for Sunrise

Witch huts redesigned, plus witch circles, in swamps and mangrove swamps.

A port of [YUNG's Better Witch Huts](https://github.com/YUNG-GANG/YUNGs-Better-Witch-Huts) by YUNGNICKYOUNG and Acarii to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3). It is a content mod with a native part: the original's structures, templates, loot tables and strings, upgraded to 26.3, with one small native part for the locate hint. The 26.3 carry is by AdLf0208 ([AdLf0208/yungs_better_witch_huts](https://github.com/AdLf0208/yungs_better_witch_huts)); this fork moves it to Sunrise.

## What it adds

- New witch huts on legs, with their interiors, witches, black cats and chest loot, in swamps and mangrove swamps.
- Witch circles, a second structure in swamps.
- Brewing stands hold a half-brewed potion: an ingredient and one or two bottles of healing, swiftness, water breathing, night vision or slow falling, as in the original.
- Vanilla swamp huts no longer generate, as in the original (its "Disable Vanilla Witch Huts" option is on by default), and `/locate structure swamp_hut` answers "Use /locate structure betterwitchhuts:witch_hut instead!". That answer is given to commands a player types; command blocks, the console and commands run through `execute` still run the vanilla search.

## Building

With the Sunrise mod tool (`cargo sunrise-mod`, from the Sunrise SDK):

```sh
cargo sunrise-mod check
cargo sunrise-mod build    # target/sunrise-mod/betterwitchhuts.smod
```

This version is made for the Sunrise Dev build `dev-2026-11-22-7cfdf81`.

## Licence and credits

YUNG's Better Witch Huts is by YUNGNICKYOUNG and Acarii (https://github.com/YUNG-GANG/YUNGs-Better-Witch-Huts). Its structures, templates, loot tables and strings are theirs and are used here under the **GNU LGPL v3.0 only**, the original's licence; `LICENSE` holds the text.

Carried to Minecraft 26.3 by AdLf0208 ([AdLf0208/yungs_better_witch_huts](https://github.com/AdLf0208/yungs_better_witch_huts)), and ported to Sunrise by arismoko. This port is distributed under the same licence.
