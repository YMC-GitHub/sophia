import { Window, getWindows, saveImageData, getScreenSize, takeScreenshot } from '../index'
import type { ImageData, Rect } from '../index'

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
async function main() {
  // let screenSize = await getScreenSize()
  log(`[zero] read all window:`)
  let windows = await getWindows()
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

  let window = await Window.fromContainsName(title)
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

    // let imgdata: ImageData
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
      winx = await window.getWindowByPid(pid)
      if (!winx) {
        winx = await Window.fromContainsName(title)
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
      log(`[zero] window close:`)
      if (await winx.isVisible()) {
        await winx.close()
      }
      let winstate = await getWindowState(window)
    }
    // winx = await window.getWindowByPid(pid)
    // if (winx) {
    //   log(`[zero] window kill:`)
    //   await winx.kill()
    // }
  }
}
main()
