import { Window, saveImageData, takeScreenshot } from '../index'
import type { ImageData, Rect, Point, WindowView } from '../index'
import { chaintask } from './nano-fune'

import { objfSortByKey } from './nano-flag'

// feat(core): sleep - let current func statck sleep miliseconds to wait
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

// feat(core): jsonstro - let json flag to string to code
export function jsonstro(json: any, trim: boolean = true) {
  let text = typeof json == 'string' ? json : JSON.stringify(json, null, 0)
  if (trim) {
    text = text.replace(/,"/gi, ',').replace(/":/gi, ':').replace(/{"/gi, '{')
  }
  return text
}

// feat(core): getWindowBaseInfo - get window title,rect,className,id
export async function getWindowBaseInfo(window: Window) {
  // info some window base info
  let rect = await window.getWindowRect()
  // log(`[zero] window rect:`);
  // log(jsonstro(rect));
  let title = await window.getTitle()
  // log(`[zero] window title:`, title);

  let className = await window.getClassName()
  // log(`[zero] window class name:`, className);

  let id = await window.getId()
  // log(`[zero] window id:`, id);

  return {
    title,
    rect,
    id,
    className,
  }
}

// feat(core): getWindowStatus - get window status and info
export async function getWindowStatus(window: Window) {
  let open = await window.isOpen()
  let visible = await window.isVisible()
  let minimize = await window.isMinimized()
  let foreground = await window.isForeground()
  const { log } = console
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

// feat(core): getWindowListInfo - get window list info and log
export async function getWindowListInfo() {
  // list all windows
  let windows = await Window.getAllWindows()

  // info all window info mation
  let asyncTaskFuna = windows.map(async (v) => {
    return await getWindowBaseInfo(v)
  })

  let base = await Promise.all(asyncTaskFuna)
  // log(base);
  return {
    windows,
    info: base,
  }
}

export interface FindWindowProp {
  title?: string
  className?: string
  id?: string
  sort?: string
}
export interface WindowBaseInfo {
  title: string
  className: string
  id: number
  rect: Rect
}
export interface WindowList {
  windows: Window[]
  info: WindowBaseInfo[]
}
export interface WindowRuntimeInfo {
  title: string
  className: string
  id: number
  left: number
  top: number
  right: number
  bottom: number
  width: number
  height: number
  window: Window
}

// feat(core): findWindowList - get window list through title,class name
/**
 *
 * @sample
 * ```
 * await findWindowList({title:'xx'})
 *
 * await findWindowList({id:xx})
 * ```
 */
export async function findWindowList(opts: FindWindowProp) {
  let { windows, info: base } = await getWindowListInfo()

  // findWindowList
  // find window list in with data and title,id,className

  let result: WindowList = {
    windows: [],
    info: [],
  }

  let { title, className, id } = opts
  for (let wi = 0; wi < base.length; wi++) {
    let keys = [title, className, id]
    let vals = keys.map((v, i) => {
      if (i === 0) {
        return v ? base[wi].title.indexOf(v) >= 0 : true
      }
      if (i === 1) {
        return v ? base[wi].className.indexOf(v) >= 0 : true
      }
      if (i === 2) {
        return v ? base[wi].id == Number(v) : true
      }
    })
    let matched = vals.every((v) => v == true)
    if (matched) {
      result.windows.push(windows[wi])
      result.info.push(base[wi])
    }
  }

  return result
}

// feat(core): sortWindowInfo - sort window info
export function sortWindowInfo(list: WindowBaseInfo[]) {
  // sort info through left top
  // return list.sort(objfSortByKey("id", 1));
  list.sort(objfSortByKey('rect.top', 1))
  list.sort(objfSortByKey('rect.left', 1))
  return list
}

// feat(core): readRuntimeWindowList - get runtime window list from window list and sort with left top
export function readRuntimeWindowList(list: WindowList) {
  let { windows, info } = list
  let flag: WindowRuntimeInfo[] = []

  for (let index = 0; index < info.length; index++) {
    const base = info[index]
    let runtime: WindowRuntimeInfo = {
      window: windows[index],
      className: base.className,
      title: base.title,
      id: base.id,
      ...base.rect,
    }
    flag.push(runtime)
  }
  // sort
  flag.sort(objfSortByKey('left', 1))
  flag.sort(objfSortByKey('top', 1))
  return flag
}

// feat(core): readWindowListFromRWL - runtime window list to window list alpha
export function readWindowListFromRWL(list: WindowRuntimeInfo[]) {
  let windows = list.map((v) => v.window)
  let info = list.map((v) => {
    let { left, top, right, bottom, width, height, className, title } = v
    return {
      className,
      title,
      rect: {
        left,
        top,
        right,
        bottom,
        width,
        height,
      },
    } as WindowBaseInfo
  })
  return { windows, info } as WindowList
}

// feat(core): captureWindowList - capture widnow list
/**
 *
 * @sample
 * ```
 * await captureWindowList({desFile:`runtime/window/{index}/main.png`})
 * ```
 */
export async function captureWindowList(
  opts: FindWindowProp & {
    desFile?: string
    desInfo?: boolean
    windows?: WindowList
    view?: WindowView
    takeScreen?: boolean
    foreGround?: boolean
  },
) {
  let { desFile, desInfo, windows, view, takeScreen, foreGround } = opts

  // findWindowList + readRuntimeWindowList
  let list = await readRuntimeWindowList(windows ? windows : await findWindowList(opts))
  let des = desFile ? desFile : ``
  // `runtime/window/{index}/main.png`

  const { log } = console
  let task = list.map((item, i) => {
    return async () => {
      let index = i + 1
      log(`[zero] capture window: ${index}`)

      let { window } = item
      let imgdata: ImageData

      // let winstate = await getWindowState(window);
      // log(`[zero] window state: `, jsonstro(winstate));

      // if (winstate.minimize) {
      //   //ensure window is not minimize! but not active!
      //   // log(`[zero] window set forground when window is isMinimized`)
      //   // let setfored = await window.setForeground()
      //   // log(`[zero] set foreground status`, setfored)
      //   // log(`[zero] window show when window is isMinimized`);
      //   let showed = await window.show();
      //   // log(`[zero] set showed status`, showed);
      //   // show ? show + fouground

      //   // log(`[zero] please wait for showing`);
      //   await sleep(100); // wait for show
      //   // Promise.race()
      // }

      // imgdata = await window.capture();

      let { x, y, width, height } = view ? view : await window.getWindowView()
      log(`[zero] window view: `, jsonstro({ x, y, width, height }))
      let rect = await window.getWindowRect()
      log(`[zero] window rect: `, jsonstro(rect))

      // view = { x: 1, y: 26, width: 1024, height: 768 };
      // ({ x, y, width, height } = view);
      // imgdata = await window.capture();

      // imgdata = await window.captureArea(x, y, width, height);
      // imgdata = await window.captureArea(0, 0, width, height);

      imgdata = await window.captureArea(0, 0, 1030, 797)
      // imgdata = await window.captureArea(0, 0, 1030, 797);

      // save when pass --des-file xx
      if (des) {
        let loc: string = des.replace(/{index}/gi, `${index}`) // render --des-file exp to txt
        if (desInfo) log(`[zero] ${loc}:`)
        await saveImageData(loc, imgdata)
        // await sleep(1000);
      }

      if (takeScreen) {
        let screendata = await takeScreenshot(x, y, width, height)
        if (foreGround && !(await window.isForeground())) {
          await window.foreground()
          await window.show()
          await sleep(2000)
          let winstate = await getWindowState(window)
          log(`[zero] window state: `, jsonstro(winstate))
        }
        if (des) {
          let loc: string = des.replace(/{index}/gi, `screen_${index}`) // render --des-file exp to txt
          if (desInfo) log(`[zero] ${loc}:`)
          await saveImageData(loc, screendata)
          // await sleep(1000);
        }
      }

      return imgdata
    }
  })
  // return await Promise.all(task);
  let result = await chaintask(task)
  return result
}

export async function getWindowState(window: Window, info: boolean = false) {
  let open = await window.isOpen()
  let visible = await window.isVisible()
  let minimize = await window.isMinimized()
  let foreground = await window.isForeground()

  if (info) {
    const { log } = console
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
