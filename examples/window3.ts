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

function getView(rect: Rect) {
  let { left, top, right, bottom } = rect
  return {
    x: left,
    y: top,
    width: right - left,
    height: bottom - top,
  }
}
async function main() {
  let screenSize = await getScreenSize()

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
  log(await Promise.all(task4))

  let title: string = ''
  //  find window contains title 'Windows PowerShell'
  title = 'Windows PowerShell'
  //  find window contains title 'Clash for Windows'
  title = 'Clash for Windows'

  let window = await Window.fromContainsName(title)
  // window = await Window.getForegroundWindow()
  // fromContainsName
  // window = await Window.findWindowByTitle('code')
  if (window) {
    // let windows = await window?.enumerate()
    // let task = windows.map(async (v) => await v.getTitle())
    // log(await Promise.all(task))

    let rect = await window.getWindowRect()
    log(rect)
    let title = await window.getTitle()
    log(title)

    let imgdata: ImageData
    let isopened = await window.isOpen()
    let isVisibled = await window.isVisible()
    let isMinimized = await window.isMinimized()
    let isForeground = await window.isForeground()

    log(`[zero] window is open:`, isopened)
    log(`[zero] window is isVisibled:`, isVisibled)
    log(`[zero] window is isMinimized:`, isMinimized)
    log(`[zero] window is isForeground:`, isForeground)

    if (isMinimized) {
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
    rect = await window.getWindowRect()
    log(rect)
    imgdata = await window.capture()

    // // imgdata = await window.capture(rect.left + 0, rect.top + 0, rect.left + 100, rect.top + 100)
    // // imgdata = await takeScreenshot(0, 0, screenSize.x, screenSize.y)
    // // // writeFileSync(``,imgdata.data)
    // // // msgLog(`[zero] demo: save image data:`);

    // let view = getView(rect)
    // log(view)

    // // let { x, y, width, height } = view
    // // imgdata = await takeScreenshot(x, y, width, height)
    // let { left, top, right, bottom } = rect
    // // imgdata = await takeScreenshot(left, top, right, bottom)
    // imgdata = await window.capture(left, top, right, bottom)

    // // imgdata = await window.capture(0, 0, rect.left + 100, rect.top + 100)

    await saveImageData(`runtime-images-sync-window.png`, imgdata)
  }
}
main()
