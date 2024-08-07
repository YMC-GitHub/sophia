import type { ImageData, Rect } from '../index'
import {
  Window,
  getAllWindows,
  saveImageData,
  getScreenSize,
  takeScreenshot,
  Mouse,
  getForegroundWindow,
  Keyboard,
  Key,
} from '../index'

// constants
/*
 * 2 - 3 - 4
 * |       |
 * 1 - 0   5
 *         |
 * 8 - 7 - 6
 */
const INTERSECTION_OFFSETS = [
  { x: 0, y: 0 }, // 0
  { x: -1, y: -1 }, // 2
  { x: 1, y: -1 }, // 4
  { x: 1, y: 1 }, // 6
  { x: -1, y: 1 }, // 8
]
const INTERSECTION_OFFSETS_LEN = INTERSECTION_OFFSETS.length
export const { log } = console
export const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

export function getRect(rect: Rect) {
  let { left, top, right, bottom } = rect
  return {
    x: left,
    y: top,
    width: right - left,
    height: bottom - top,
  }
}

export function jsonstro(json: any, trim: boolean = true) {
  let text = typeof json == 'string' ? json : JSON.stringify(json, null, 0)
  if (trim) {
    text = text.replace(/,"/gi, ',').replace(/":/gi, ':').replace(/{"/gi, '{')
  }
  return text
}

export function randomInt(min: number, max: number) {
  return Math.round(Math.random() * (max - min) + min)
}

export function num2strhex(a: number) {
  let hexstr = a.toString(16).padStart(8, '0')
  hexstr = `0x${hexstr}`
  return hexstr
}

/**
 * string to unit8 array buffer
 * @sample
 * ```
 *
 * ```
 */
export function strToBuf(str: string) {
  let array = new Uint8Array(new ArrayBuffer(str.length))
  for (let i = 0, il = str.length; i < il; i++) {
    let value = str.charCodeAt(i)
    array[i] = value > 0xff ? 0x20 : value
  }
  let arrBuffer = array.buffer
  return arrBuffer
}

export async function getWindowState(window: Window, info: boolean = false) {
  let open = await window.isOpen()
  let visible = await window.isVisible()
  let minimize = await window.isMinimized()
  let foreground = await window.isForeground()
  if (info) {
    log(`[zero] window is open:`, open)
    log(`[zero] window is isVisibled:`, visible)
    log(`[zero] window is isMinimized:`, minimize)
    log(`[zero] window is isForeground:`, foreground)
  }
  return {
    open,
    visible,
    minimize,
    foreground,
  }
}

// info mouse position in screen and window per 1s
export function infoMousePosition(window: Window | null) {
  setInterval(async () => {
    log(`[zero] mouse postion:`)
    let flag = {
      screen: await Mouse.getPosition(),
      window: window ? await window.getMousePos() : {},
    }
    log(jsonstro(flag))
  }, 1000)
}

// info mouse position in screen and window per 1s
export function moveMousePositionRand(window: Window | null) {
  setInterval(async () => {
    log(`[zero] move mouse random:`)
    if (window) {
      let { width, height } = await window.getWindowView()
      let coords = {
        x: randomInt(0, width),
        y: randomInt(0, height),
      }
      log(`[zero] random coords:`, jsonstro(coords))
      await window.mouseMove(coords, false)
    }
  }, 1000)
}

export function moveMousePositionInMenuRect(window: Window | null) {
  setInterval(async () => {
    log(`[zero] move mouse in menu rect:`)
    if (window) {
      // let { width, height } = await window.getWindowView()
      let coords = {
        x: randomInt(10, 253),
        y: randomInt(39, 757),
      }
      log(`[zero] random coords:`, jsonstro(coords))
      await window.mouseMove(coords, false)
      await window.mouseToggle(coords, 'left', false)
      await sleep(50)
      await window.mouseToggle(coords, 'left', true)
      await sleep(50)
    }
  }, 1000)
}

export function scrollMouseWheeInScrollRect(window: Window | null) {
  setInterval(async () => {
    log(`[zero] scroll mouse in scroll rect:`)
    if (window) {
      let { width, height } = await window.getWindowView()
      let coords = {
        x: Math.round(width / 2),
        y: Math.round(height / 2),
      }
      coords = {
        x: 1279,
        y: 473,
      }
      let up = randomInt(0, 10) >= 5 ? true : false
      log(`[zero] random direction:`, up ? 'up' : 'down')
      coords = up
        ? {
            x: 1279,
            y: 201,
          }
        : {
            x: 1279,
            y: 841,
          }

      log(`[zero] random coords:`, jsonstro(coords))
      await window.mouseMove(coords, false)

      // click up or down at coord to mock scroll whell
      // // press left
      // await window.mouseToggle(coords, 'left', false)
      // await sleep(50)
      // // release left
      // await window.mouseToggle(coords, 'left', true)
      // await sleep(50)

      await window.mouseWheelScroll(coords, up)
    }
  }, 1000)
}

export function typingInInputRect(window: Window | null, text: string | Buffer) {
  setInterval(async () => {
    log(`[zero] typing text in input rect:`)
    if (window) {
      let { width, height } = await window.getWindowView()
      let coords = {
        x: Math.round(width / 2),
        y: Math.round(height / 2),
      }
      coords = {
        x: 602,
        y: 97,
      }
      // move -> click ->  typing : done
      await window.mouseMove(coords, false)
      // press left
      await window.mouseToggle(coords, 'left', false)
      await sleep(50)
      // release left
      await window.mouseToggle(coords, 'left', true)
      await sleep(50)
      // ucs2
      let buffer = typeof text == 'string' ? Buffer.from(text, 'utf8') : text
      // log(buffer.toString('ucs2'))
      log(buffer.toString('utf8'))
      // await window.typing(buffer)
      await window.typing(buffer.toString('utf8'))
    }
  }, 1000)
}

export async function sendkeysToActiveWindow(window: Window) {
  // active window -> send keys: ctrl + v
  log(`[zero] sendkeys: mock ctrl + v done: based on active window & global keyboard press`)
  await window.setForeground()
  await Keyboard.press(Key.Control)
  await Keyboard.press(Key.V)
}

export async function sendkeysToNotActiveWindow(window: Window) {
  // active window -> send keys: ctrl + v
  log(`[zero] sendkeys: mock ctrl + v done: based on not active window & global keyboard press`)
  await Keyboard.press(Key.Control)
  await window.keyboardToggleKey(['V'], true, false) //v down when ctrl is down
}

export function toggleKeyInWindow(window: Window | null) {
  setInterval(async () => {
    log(`[zero] toggle key in widnow rect:`)

    // let active = await window.fromActive()

    if (window) {
      let { width, height } = await window.getWindowView()
      let coords = {
        x: Math.round(width / 2),
        y: Math.round(height / 2),
      }
      coords = {
        x: 602,
        y: 97,
      }

      // await window.setForeground()

      // // move -> click ->  typing : done
      // move -> click ->  typing : done
      await window.mouseMove(coords, false)
      // press left
      await window.mouseToggle(coords, 'left', false)
      await sleep(50)
      // release left
      await window.mouseToggle(coords, 'left', true)
      await sleep(50)
      // await window.typing(buffer)
      // await window.typing('hello 你好')

      // active window -> send keys: ctrl + v
      // await sendkeysToActiveWindow(window)

      // await sendkeysToNotActiveWindow(window)

      // log(`ctrl + v:`)
      // await Keyboard.press(Key.Control)
      await window.keyboardToggleKey(['ctrl'], true, false) //ctrl down
      // await sleep(50)
      // await window.typing('V')
      await window.keyboardToggleKey(['V'], true, false) //v down when ctrl is down
      //00000056 VK:V WM_KEYDOWN OR WM_KEYUP
      //00000076 chCharCode 118 WM_CHAR
      //00000016 chCharCode 22 WM_CHAR
      //00000099 chCharCode 63 WM_CHAR

      // await sleep(50)
      // await window.keyboardToggleKey(['V'], false, true) //v up when ctrl is down
      // await sleep(50)
      // await window.keyboardToggleKey(['ctrl'], false, false) //ctrl up
      // await sleep(50)

      // log(`ctrl + v:`)
      // await window.keyboardToggleKey(['ctrl', 'V'], true) //down
      // await sleep(50)
      // await window.keyboardToggleKey(['ctrl', 'V'], false) //up

      // lWin + d: not work
      // log(`lWin + d:`)
      // await window.keyboardToggleKey(['lWin', 'd'], true) //down
      // await sleep(100)
      // await window.keyboardToggleKey(['lWin', 'd'], false) //down

      // log(`lAlt + f4:`)
      // await window.keyboardToggleKey(['lAlt', 'f4'], true) //down
      // await sleep(100)
      // await window.keyboardToggleKey(['lAlt', 'f4'], false) //down

      // lCtrl + f in vscode : done
      // log(`lCtrl + F:`)
      // await window.keyboardToggleKey(['lCtrl', 'F'], true) //down
      // await sleep(100)
      // await window.keyboardToggleKey(['lCtrl', 'F'], false) //up

      // log(`ctrl + f:`)
      // await window.keyboardToggleKey(['ctrl', 'F'], true) //down
      // await sleep(10)
      // await window.keyboardToggleKey(['ctrl', 'F'], false) //up

      // log(`alt + Z:`)
      // await window.keyboardToggleKey(['alt', 'Z'], true) //down
      // await sleep(100)f
      // await window.keyboardToggleKey(['alt', 'Z'], false) //up

      // log(`ctrl + p:`)
      // await window.keyboardToggleKey(['ctrl', 'p'], true) //down
      // await sleep(100)
      // await window.keyboardToggleKey(['ctrl', 'p'], false) //up

      // log(`ctrl + o:`)
      // await window.keyboardToggleKey(['ctrl', 'o'], true) //down
      // await sleep(100)
      // await window.keyboardToggleKey(['ctrl', 'o'], false) //up
      // toggleKey(["ctrl", "shift", "a"], true, [25, 50])
    }
  }, 1000)
}

/**
 *
 * @sample
 * ```
 * infolparam(0x20380001);
 * ```
 */
export async function infolparam(a: number) {
  // let a = 0x20380001
  let hexstr = num2strhex(a)
  let flag = {
    ...(await Window.decodeLparamValue(a)),
  }
  log(`[zero] info lparam:`, hexstr)
  log(`[zero] lparam flag:`, jsonstro(flag))
  return flag
}

export async function infoLparamList() {
  await infolparam(0x20380001)
  await infolparam(0x20200001)
  await infolparam(0xe0200001)
  await infolparam(0xc0380001)

  // PostMessage(hWnd,WM_SYSKEYDOWN, VK_MENU, 0x20380001);
  // PostMessage(hWnd,WM_SYSKEYDOWN, 0x56,0x20200001);
  // PostMessage(hWnd,WM_SYSCHAR,0x76,0x20200001);
  // PostMessage(hWnd,WM_SYSKEYUP,0x56,0xE0200001);
  // PostMessage(hWnd,WM_KEYUP, VK_MENU, 0xC0380001);

  // let flag = await Window.decodeLparamValue(0x20380001)
  // 0x70 + xx  ->
  // let lparamvalue = await Window.makeLparamValue(0x70, flag)
  // await infolparam(lparamvalue)
  // // {scanCode:8251,repeatCount:1,transitionState:false,isExtended:false,previousKeyState:false,contextCode:true,lparam:"0x203b0001"}

  // // down +
  // flag.transitionState = true
  // flag.previousKeyState = true
  // flag.contextCode = false
  // lparamvalue = await Window.makeLparamValue(0x70, flag)
  // infolparam(lparamvalue)

  // up
  // flag.transitionState = true
  // flag.previousKeyState = true
  // flag.contextCode = false
  // lparamvalue = await Window.makeLparamValue(0x56, flag)
  // infolparam(lparamvalue)

  // log(`[zero] Key.V to lparam value when it press with previousKey press:`)
  // flag.transitionState = true
  // flag.previousKeyState = true
  // flag.contextCode = false
  // lparamvalue = await Window.makeLparamValue(Key.V, flag)
  // infolparam(lparamvalue)

  function keyState(s: 'up' | 'down' = 'down') {
    return s == 'down'
  }

  // lparamvalue = await Window.makeLparamValue(Key.V, {
  //   ...flag,
  //   transitionState: keyState('down'),
  //   previousKeyState: keyState('down'),
  // })
  // infolparam(lparamvalue)
  // log(`${Key.V} -> ${num2strhex(lparamvalue)}:fail`)

  // log(`[zero] Key.V to lparam value when it release with previousKey press:`)
  // lparamvalue = await Window.makeLparamValue(Key.V, {
  //   ...flag,
  //   transitionState: keyState('up'),
  //   previousKeyState: keyState('down'),
  // })
  // infolparam(lparamvalue)
  // log(`${Key.V} -> ${num2strhex(lparamvalue)}:fail`)
}
