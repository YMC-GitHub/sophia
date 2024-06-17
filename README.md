<div align="center">
  <h1>Sophia</h1>
  <p>
    <strong>🤖 A Node.js library for automating Windows applications.</strong>
  </p>
  
  [![NPM Version](https://img.shields.io/npm/v/@yors/sophia)](https://www.npmjs.com/package/@yors/sophia)
  ![NPM License](https://img.shields.io/npm/l/@yors/sophia)
</div>

## forked from @deeean/sophia

## Features

- Keyboard
- Mouse
- Screen
- Window
- Memory

get more info in [CHANELOG.md](./CHANELOG.md)

## Installation

```bash
npm install @yors/sophia
```

## Example

Typing a string

```typescript
import { Keyboard } from '@yors/sophia'

async function main() {
  await Keyboard.typing('Hello, World!')
}

main()
```

<br />

Registering a hotkey for specific key combinations and handling events.

```typescript
import { Keyboard, Modifiers, Key } from '@yors/sophia'

Keyboard.registerHotkey([Modifiers.Control], Key.A, () => {
  console.log('Ctrl + A is pressed')
})
```

<br />

Finding the location of one image within another

```typescript
import { readImageData, imageSearch } from '@yors/sophia'

async function main() {
  const [baboon, partsOfBaboon] = await Promise.all([
    readImageData('./examples/images/baboon.png'),
    readImageData('./examples/images/parts_of_baboon.png'),
  ])

  const position = await imageSearch(baboon, partsOfBaboon)
  if (position) {
    console.log('Found at', position)
  } else {
    console.log('Not found')
  }
}

main()
```

<br />

Finding the location of one image within the screenshot

```typescript
import * as sophia from '@yors/sophia'

async function main() {
  const [baboon] = await Promise.all([sophia.readImageData('./examples/images/baboon.png')])

  const screenSize = await sophia.getScreenSize()
  const screenshot = await sophia.takeScreenshot(0, 0, screenSize.x, screenSize.y)

  const position = await sophia.imageSearch(screenshot, baboon)
  if (position) {
    console.log('Found at', position)
  } else {
    console.log('Not found')
  }
}

main()
```

<br />

Getting the list of processes and reading/writing memory

```typescript
import { getProcesses, openProcess, ProcessAccess } from '@yors/sophia'

const BASE_ADDRESS = BigInt(0x003264d0)
const OFFSETS = [BigInt(0x48), BigInt(0x0), BigInt(0xf8), BigInt(0x18), BigInt(0x408), BigInt(0x50), BigInt(0x7f8)]

async function main() {
  const processes = await getProcesses()
  const tutorial = processes.find((p) => p.name === 'Tutorial-x86_64.exe')
  if (!tutorial) {
    console.log('Tutorial-x86_64.exe not found')
    return
  }

  const openedProcess = await openProcess(ProcessAccess.AllAccess, tutorial.pid)

  const health = await openedProcess.readMemoryChainUint32(BASE_ADDRESS, OFFSETS)
  if (health < 1000n) {
    await openedProcess.writeMemoryChainUint32(BASE_ADDRESS, OFFSETS, 1000n)
  }
}

main()
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
