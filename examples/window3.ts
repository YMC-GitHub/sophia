import { Window, getAllWindows, saveImageData, getScreenSize, takeScreenshot, Mouse } from '../index'
import type { ImageData, Rect } from '../index'
import { Buffer } from 'buffer'
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
const { log } = console
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

function getRect(rect: Rect) {
  let { left, top, right, bottom } = rect
  return {
    x: left,
    y: top,
    width: right - left,
    height: bottom - top,
  }
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

function randomInt(min: number, max: number) {
  return Math.round(Math.random() * (max - min) + min)
}

// info mouse position in screen and window per 1s
function infoMousePosition(window: Window | null) {
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
function moveMousePositionRand(window: Window | null) {
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

function moveMousePositionInMenuRect(window: Window | null) {
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

function scrollMouseWheeInScrollRect(window: Window | null) {
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

function typingInInputRect(window: Window | null, text: string | Buffer) {
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

/**
 * string to unit8 array buffer
 * @sample
 * ```
 *
 * ```
 */
function strToBuf(str: string) {
  let array = new Uint8Array(new ArrayBuffer(str.length))
  for (let i = 0, il = str.length; i < il; i++) {
    let value = str.charCodeAt(i)
    array[i] = value > 0xff ? 0x20 : value
  }
  let arrBuffer = array.buffer
  return arrBuffer
}
async function main() {
  // let screenSize = await getScreenSize()
  log(`[zero] read all window:`)
  let windows = await getAllWindows()
  // // info windows title
  // let task = windows.map(async (v) => await v.getTitle())
  // log(await Promise.all(task))

  // // info windows rect
  // let task2 = windows.map(async (v) => await v.getWindowRect())
  // log(await Promise.all(task2))

  // // info windows rect
  // let task3 = windows.map(async (v) => await v.getId())
  // log(await Promise.all(task3))

  let task4 = windows.map(async (v) => {
    return {
      title: await v.getTitle(),
      rect: await v.getWindowRect(),
      id: await v.getId(),
      className: await v.getClassName(),
    }
  })
  let wins = await Promise.all(task4)
  // log(jsonstro(wins))
  log(wins)

  log(`[zero] read window by title:`)
  let title: string = ''
  //  find window contains title 'Windows PowerShell'
  title = 'Windows PowerShell'
  //  find window contains title 'Clash for Windows'
  title = 'Clash for Windows'
  // window3.ts - sophia - Visual Studio Code

  // title = 'window3.ts - sophia - Visual Studio Code'

  let window = await Window.findWindowByTitle(title)
  // window = await Window.getForegroundWindow()
  // fromContainsName
  // window = await Window.findWindowByTitle('code')
  if (window) {
    // let windows = await window?.enumerate()
    // let task = windows.map(async (v) => await v.getTitle())
    // log(await Promise.all(task))

    let rect = await window.getWindowRect()
    log(`[zero] window rect:`)
    log(jsonstro(rect))
    let title = await window.getTitle()
    log(`[zero] window title:`, title)

    let className = await window.getClassName()
    log(`[zero] window class name:`, className)

    let pid = await window.getId()
    log(`[zero] window pid:`, pid)

    let imgdata: ImageData
    let winstate = await getWindowState(window)
    if (winstate.minimize) {
      // log(`[zero] window set forground when window is isMinimized`)
      // let setfored = await window.setForeground()
      // log(`[zero] set foreground status`, setfored)

      log(`[zero] window show when window is isMinimized`)
      let showed = await window.show()
      log(`[zero] set showed status`, showed)
      // show ? show + fouground
      await sleep(100)
      // Promise.race()
    }

    // log(`[zero] window current rect:`)
    // rect = await window.getWindowRect()
    // log(jsonstro(rect))

    // log(`[zero] window capture view:`)
    // imgdata = await window.capture()
    // await saveImageData(`runtime-images-sync-window.png`, imgdata)

    // log(`[zero] window capture rect:`)
    // let { width, height } = getRect(rect)
    // imgdata = await window.captureArea(width / 2 - 50, height / 2 - 50, 100, 100)
    // await saveImageData(`runtime-images-sync-window-rect-01.png`, imgdata)

    log(`[zero] window get mouse pos in window:`)
    let pos = await window.getMousePos()
    log(jsonstro(pos))

    log(`[zero] window get by id:`)
    let winx: Window | null = null
    try {
      winx = await window.fromPid(pid)
      if (!winx) {
        winx = await Window.findWindowBySubTitle(title)
      }
    } catch (error) {
      // winx = await window.get(pid)
    }

    // called `Option::unwrap()` on a `None` value when not found
    if (winx) {
      log(`[zero] window get meta info when window found:`)
      let info = await winx.getWindowMetaInfo()
      log(jsonstro(info))
    }

    if (winx) {
      // [zero] window is isVisibled: false
      // [zero] window is isMinimized: false
      // [zero] window is isForeground: false
      // if (!(await winx.isForeground())) {
      //   log(`[zero] window set forground:`)
      //   await winx.setForeground()
      //   await sleep(500)
      //   await getWindowState(winx)
      // }
      // close -> show : fail
      // if (await winx.isVisible()) {
      //   log(`[zero] window close:`)
      //   await winx.close()
      //   await sleep(500)
      //   await getWindowState(winx)
      // }
      // if (!(await winx.isVisible())) {
      //   log(`[zero] window show:`)
      //   await winx.setForeground()
      //   await winx.show()
      //   await sleep(1000)
      //   await getWindowState(winx)
      // }
      // hide -> show :
      // if (await winx.isVisible()) {
      //   log(`[zero] window close:`)
      //   await winx.hide()
      //   await sleep(500)
      //   await getWindowState(winx)
      // }
      // if (!(await winx.isVisible())) {
      //   log(`[zero] window show:`)
      //   await winx.show()
      //   await sleep(1000)
      //   await getWindowState(winx)
      // }

      // minimize -> show: done
      // if (!(await winx.isMinimized())) {
      //   log(`[zero] window minimize:`)
      //   await winx.minimize()
      //   await sleep(500)
      //   await getWindowState(winx)
      // }
      // if (await winx.isMinimized()) {
      //   log(`[zero] window show:`)
      //   await winx.show()
      //   await sleep(500)
      //   await getWindowState(winx)
      // }
      // if (!(await winx.isForeground())) {
      //   log(`[zero] window set forground:`)
      //   await winx.setForeground()
      //   await sleep(500)
      //   await getWindowState(winx)
      // }

      log(`[zero] window capture:`)
      imgdata = await winx.capture()
      let loc: string = `./runtime-images-sync-window.png`
      log(`[zero] ${loc}:`)
      await saveImageData(loc, imgdata)
    }
    // winx = await window.getWindowByPid(pid)
    // if (winx) {
    //   log(`[zero] window kill:`)
    //   await winx.kill()
    // }

    if (winx) {
      infoMousePosition(winx)
      // moveMousePositionRand(winx)
      // moveMousePositionInMenuRect(winx)
      // scrollMouseWheeInScrollRect(winx)
      // typingInInputRect(winx, 'hello world!') //done
      // @ts-ignore
      // typingInInputRect(winx, Buffer.from('zero 你好!', 'ucs2')) //fail
      typingInInputRect(winx, Buffer.from('zero 你好!', 'utf8')) //fail
    }
  }
}
main()
