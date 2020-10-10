# emoji-first Phasmophobia evidence tracker 

[![App]][appurl] [![Ghosts]][ghostsurl]

[ghosts]: https://img.shields.io/badge/ghosts-555555?style=for-the-badge&labelColor=555555&logoColor=white
[ghostsurl]: https://swfsql.github.io/phasmo/doc/phasmo_rs/phasmo/enum.Ghost.html#variants
[app]: https://img.shields.io/badge/app-555555?style=for-the-badge&labelColor=555555&logoColor=white
[appurl]: https://swfsql.github.io/phasmo/

Utility to help tracking evidences for the Phasmophobia game.

Information mostly taken from [fandom/phasmophobia](https://phasmophobia.fandom.com/wiki/Phasmophobia_Wiki) (8/oct/2020).  
Also inspired by other evidence trackers (see alternatives at the end).

## Why is this

It should be easier to just open the app and click around, but otherwise you can:

- activate/forbid evidences
- see possible ghost according to the evidence
- see strenght and weakness features for those ghosts
- check a ghost's description

The stuff you see are mostly emojis, as they are very terse ^^  
For VR, I believe this app should work as a custom app from tools like OVRToolkit - but the clicking should be hard as it's not optimized for "touch-like" clicks.

## Bugs

- If your pc/phone browser doesn't support emoji, you are in bad luck.
    - You can check [this page](https://swfsql.github.io/phasmo/doc/phasmo_rs/phasmo/enum.Ghost.html#variants) to verify.
    - Although apps like WhatsApp are able to show emojis, they use their own font so this doesn't mean that your browser will also be able to show them. (I didn't know that, and my own phone cant lol)
- At the start, only a small portion of the screen is shown. Moving/resizing/zooming the window appears to fix this.
- Zooming in/out mess up the cursor position. To mitigate this, I've added a small circle representing the "seen" cursor.

Please note that the underlying drawing library is experimental and is not stable yet.  
Also, note that this particular app is really non-optimal (resource-wise).

## Offline Testing

For offline building and running, you can either run on your native platform, or you can still run the local wasm file.

For both cases you'll need a [rust nightly toolchain](https://www.rust-lang.org/tools/install).

### Native

Just download the repo and `cargo run` it.

### Wasm

You'll need [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html) (no need to install anything related to npm) and a file server such as [http](https://github.com/thecoshman/http#installation).

Just download the repo, enter it, run the stuff below and then you may access [your local host](http://localhost:8000/).

```
wasm-pack build --target web
http
```

## Alternatives

- [Phasmophobia Evidence Tracker](https://phasmophobiatracker.site/) 
- [Phasmophobia Helper](https://lemon-field-0b94c1010.azurestaticapps.net/) - [reddit](https://www.reddit.com/r/PhasmophobiaGame/comments/j6qp9c/i_made_an_interactive_web_app_to_help_with/)
- [Cheat Sheet](https://www.reddit.com/r/PhasmophobiaGame/comments/j75rtv/master_cheat_sheet_that_covers_all_types_and/)

Feel free to correct or add more info! either here on this Readme or on the overall project itself!
