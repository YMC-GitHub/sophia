<div align="center">
  <h1>Sophia</h1>
  <p>
    <strong>🤖 A Node.js library for automating Windows applications.</strong>
  </p>
  
  [![NPM Version](https://img.shields.io/npm/v/@yors/sophia)](https://www.npmjs.com/package/@yors/sophia)
  ![NPM License](https://img.shields.io/npm/l/@yors/sophia)
</div>

## forked from @deeean/sophia

## Features adding

- to send combine keys to non-active window
- capture non-active window

get more info in [CHANELOG.md](./CHANELOG.md)

## Features based

- Keyboard
- Mouse
- Screen
- Window
- Memory

get more info in [@deeean/sophia](https://www.npmjs.com/package/@deeean/sophia)

## Installation

```bash
npm install @yors/sophia
```

## Example

```ts
import { Window } from '@yors/sophia'
import type { ImageData } from '@yors/sophia'

// import { Window, saveImageData } from '../'
// import type { ImageData } from '../'

const { log } = console
main()

async function main() {
  // list all windows
  let windows = await Window.getAllWindows()

  // info all window info mation
  let task4 = windows.map(async (v) => {
    return {
      title: await v.getTitle(),
      rect: await v.getWindowRect(),
      id: await v.getId(),
      className: await v.getClassName(),
    }
  })
  let wins = await Promise.all(task4)
  log(wins)

  // find window by title
  // or className,pid ...
  let title = 'Clash for Windows'
  let window = await Window.findWindowByTitle(title)

  if (window) {
    // info some window base info
    let rect = await window.getWindowRect()
    log(`[zero] window rect:`)
    log(jsonstro(rect))
    let title = await window.getTitle()
    log(`[zero] window title:`, title)

    let className = await window.getClassName()
    log(`[zero] window class name:`, className)

    let pid = await window.getId()
    log(`[zero] window pid:`, pid)

    // capture window to image data
    let imgdata: ImageData
    let winstate = await getWindowState(window)
    if (winstate.minimize) {
      //ensure window is not minimize! but not active!
      // log(`[zero] window set forground when window is isMinimized`)
      // let setfored = await window.setForeground()
      // log(`[zero] set foreground status`, setfored)
      log(`[zero] window show when window is isMinimized`)
      let showed = await window.show()
      log(`[zero] set showed status`, showed)
      // show ? show + fouground

      log(`[zero] please wait for showing`)
      await sleep(100) // wait for show
      // Promise.race()
    }

    log(`[zero] window capture:`)
    imgdata = await window.capture()
    let loc: string = `./runtime-images-sync-window.png`
    log(`[zero] ${loc}:`)
    await saveImageData(loc, imgdata)
    let { width, height } = await window.getWindowView()
    let coords = {
      x: Math.round(width / 2),
      y: Math.round(height / 2),
    }
    coords = {
      x: 602,
      y: 97,
    }

    await window.mouseMove(coords, false)
    // press left
    await window.mouseToggle(coords, 'left', false)
    await sleep(50)
    // release left
    await window.mouseToggle(coords, 'left', true)
    await sleep(50)

    // await window.typing('hello 你好')

    // sending combine keys to non-active window
    // method 01: work
    // await Keyboard.press(Key.Control)
    // await window.keyboardToggleKey(['V'], true, false)

    // method 02: work
    // await window.keyboardToggleKey(['ctrl'], true, false) // down
    // await window.keyboardToggleKey(['V'], true, false) //down
    // await window.keyboardToggleKey(['V'], false, false) //up
    // await window.keyboardToggleKey(['ctrl'], false, false) //up

    // method 03: work (please not!please release ctrl)
    // await window.keyboardToggleKey(['ctrl', 'V'], true, false) // down
    // await window.keyboardToggleKey(['V', 'ctrl'], false, false) //up

    // method 04: work
    await window.keyboardToggleKey(['ctrl', 'V'], true, false) // down
    await window.keyboardToggleKey(['ctrl', 'V'], false, false) //up
  }
}

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
function jsonstro(json: any, trim: boolean = true) {
  let text = typeof json == 'string' ? json : JSON.stringify(json, null, 0)
  if (trim) {
    text = text.replace(/,"/gi, ',').replace(/":/gi, ':').replace(/{"/gi, '{')
  }
  return text
}

async function getWindowState(window: Window) {
  let open = await window.isOpen()
  let visible = await window.isVisible()
  let minimize = await window.isMinimized()
  let foreground = await window.isForeground()

  log(`[zero] window is open:`, open)
  log(`[zero] window is isVisibled:`, visible)
  log(`[zero] window is isMinimized:`, minimize)
  log(`[zero] window is isForeground:`, foreground)
  return {
    open,
    visible,
    minimize,
    foreground,
  }
}
```

## Supported Platforms

Only support Windows x64 for now.

## Inspiration

I'm a big fan of [AutoHotkey](https://www.autohotkey.com/), but I want to use it in Node.js. So I decided to create a library that can automate Windows applications.

## Related projects

- [AutoHotkey](https://github.com/AutoHotkey/AutoHotkey)
- [PyAutoGUI](https://github.com/asweigart/pyautogui)
- [RobotJS](https://github.com/octalmage/robotjs)
- [Krombik/keysender](https://github.com/Krombik/keysender)
- [deeean/sophia](https://github.com/deeean/sophia)

## Thanks

- [miniben-90/x-win](https://github.com/miniben-90/x-win)
- [rubickCenter/rubick-native](https://github.com/rubickCenter/rubick-native)
- [nashaofu/node-screenshots](https://github.com/nashaofu/node-screenshots)
- [LaunchMenu/node-global-key-listener](https://github.com/LaunchMenu/node-global-key-listener)

- [dimusic/active-win-pos-rs](https://github.com/dimusic/active-win-pos-rs)

## License

Sophia is licensed under the MIT License. Feel free to use it in your projects, adhering to the license terms.

## Todo

- ~~init impl in an init fn like [dimusic/active-win-pos-rs 's init_platform_api](https://github.com/dimusic/active-win-pos-rs/blob/main/src/win/mod.rs)~~
- share keyboard keys in global and virtual window

- [x] sophia-util,sophia-core
- [x] sophia-wasm
- [ ] sophia-node
