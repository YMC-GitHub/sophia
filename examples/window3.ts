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
import type { ImageData, Rect } from '../index'
import { Buffer } from 'buffer'

import {
  sleep,
  log,
  getRect,
  jsonstro,
  randomInt,
  num2strhex,
  getWindowState,
  infoMousePosition,
  moveMousePositionRand,
  moveMousePositionInMenuRect,
  scrollMouseWheeInScrollRect,
  typingInInputRect,
  sendkeysToActiveWindow,
  sendkeysToNotActiveWindow,
  toggleKeyInWindow,
  infoLparamList,
} from './utils'

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

  // title = 'hello.txt - Notepad'

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
      // typingInInputRect(winx, 'zero 你好!')

      // infolparam(0x20380001)
      // infoLparamList()

      toggleKeyInWindow(winx)
      // toggleKeyInWindow(await winx.fromActive())
    }
  }
}
main()
