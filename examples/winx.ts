import { Window, saveImageData, takeScreenshot, getScreenSize, readImageData, imageSearch } from '../'
import type { ImageData, Rect, Point, WindowView } from '../'
import { chaintask } from './nano-fune'

import { objfSortByKey } from './nano-flag'
import { isAllowIndex } from './nano-stre'
import { makedirs } from './nanz-core'
import { formatDate } from './nano-time'

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
export interface FindWindowProp {
  title?: string
  className?: string
  id?: string
  sort?: string
}
export interface CaptureImageProp {
  desFile?: string
  desInfo?: boolean
  windows?: WindowList
  view?: WindowView
  takeScreen?: boolean
  foreGround?: boolean
  allowIndex?: string
}
export interface SearchImageCrop {
  src?: string
  srcLarge?: string
  screen?: string
  variant?: number
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
export async function captureWindowList(opts: FindWindowProp & CaptureImageProp) {
  let { desFile, desInfo, windows, view, takeScreen, foreGround, allowIndex } = opts

  // findWindowList + readRuntimeWindowList
  let list = await readRuntimeWindowList(windows ? windows : await findWindowList(opts))
  let des = desFile ? desFile : ``
  // `runtime/window/{index}/main.png`

  const { log } = console
  let task = list.map((item, i) => {
    return async () => {
      let index = i + 1
      if (allowIndex && !isAllowIndex(index, allowIndex)) {
        return null
      }

      // log(`[zero] capture window: ${index}`);

      let { window } = item
      let imgdata: ImageData
      let { x, y, width, height } = view ? view : await window.getWindowView()
      // get width and height in window when not pass view
      if (!view) {
        x = 0
        y = 0
      }
      imgdata = await window.captureArea(x, y, width, height)
      // save when pass --des-file xx
      if (des) {
        let loc: string = des.replace(/{index}/gi, `${index}`) // render --des-file exp to txt
        if (desInfo) log(`[zero] ${loc}:`)
        makedirs(loc)
        await saveImageData(loc, imgdata)
        // await sleep(1000);
      }

      if (takeScreen) {
        // --take-screen --foreground --des xx
        let screendata = await takeScreenshot(x, y, width, height)
        if (foreGround && !(await window.isForeground())) {
          await window.foreground()
          await window.show()
          await sleep(2000)
          // let winstate = await getWindowState(window);
          // log(`[zero] window state: `, jsonstro(winstate));
        }
        if (des) {
          let loc: string = des.replace(/{index}/gi, `screen_${index}`) // render --des-file exp to txt
          // if (desInfo) log(`[zero] ${loc}:`);
          makedirs(loc)
          await saveImageData(loc, screendata)
          // await sleep(1000);
        }
      }

      return imgdata
    }
  })
  // return await Promise.all(task);
  let result: (ImageData | null)[] = await chaintask(task)
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

export async function searchImage(opts: SearchImageCrop & FindWindowProp & CaptureImageProp) {
  let { src, srcLarge, screen, title, allowIndex, desFile, variant } = opts
  src = src ? src : ``
  srcLarge = srcLarge ? srcLarge : ``
  allowIndex = allowIndex ? `${allowIndex}` : ''
  desFile = desFile ? `${desFile}` : ``
  // runtime/window/{index}_main_search.png

  // image,search,replace
  const searchin = await Promise.all([src].filter((v) => v).map((loc) => readImageData(loc)))

  // let screenshot : ImageData
  let imagelist: (ImageData | null)[] = []

  if (srcLarge) {
    imagelist = await Promise.all([srcLarge].filter((v) => v).map((loc) => readImageData(loc)))
  } else {
    if (screen) {
      // capture screen when --screen
      const screenSize = await getScreenSize()
      const screenshot = await takeScreenshot(0, 0, screenSize.x, screenSize.y)
      imagelist.push(screenshot)
    } else {
      let { windows, info } = await findWindowList({ title })

      let runtimeWindows = readRuntimeWindowList({ windows, info })
      ;({ windows, info } = readWindowListFromRWL(runtimeWindows))
      let view = await windows[0].getWindowView()
      // const { log } = console;
      // log(jsonstro(view));
      let takeScreen = screen ? screen : false
      let now = new Date()
      let strn = formatDate('yyyy_MM_dd_HH_mm_ss', now)
      let saveFile: boolean = false
      let enableInfoImage: boolean = false

      let images = await captureWindowList({
        title: title,
        // id: String(wininfo.id),
        view: { ...view, x: 0, y: 0 },
        desFile: saveFile ? desFile.replace(/{now}/gi, strn) : '',
        takeScreen: takeScreen as boolean,
        foreGround: false,
        allowIndex: allowIndex,
        // windows: windows,
      })

      // imagelist = images;
      // pixelWidth=184!!
      // {width:1030,height:797,pixelWidth:184}
      // will occur err when pixelWidth large than 4

      // load image from files when using save-file mode
      // capture -> save -> load (not perf)
      if (saveFile) {
        let files = make_files_from_list(desFile, { now: `${strn}` }, images)
        imagelist = await read_image_from_files_allow_null(files)
        imagelist.map((i) => info_image(i, enableInfoImage))
        // {width:1030,height:797,pixelWidth:4}
      } else {
        images.forEach((i) => edit_image_pixel_width_when_lg_pixel_width(i, 4))
        images.forEach((i) => info_image(i, enableInfoImage))
        imagelist = images
      }

      // load images from files
    }
  }

  let [baboon] = searchin
  let res_point = await find_image_allow_null(baboon, imagelist, false, variant)
  let { width, height } = baboon
  let res_rect = res_point.map((v) => {
    if (v) {
      let center = {
        cx: v.x + Math.floor(width / 2),
        cy: v.y + Math.floor(height / 2),
      }
      return { ...v, width, height, ...center }
    }
    return null
  })
  return res_rect

  function info_image(img: ImageData | null, enableInfoImage: boolean = false) {
    if (enableInfoImage && img) {
      let { width, height, pixelWidth } = img
      console.log(jsonstro({ width, height, pixelWidth }))
    }
  }
  function edit_image_pixel_width_when_lg_pixel_width(img: ImageData | null, pixelWidth: number = 4) {
    if (img) {
      if (img.pixelWidth > pixelWidth) img.pixelWidth = pixelWidth
    }
  }

  function make_files_from_list(desFileTpl: string, data: Record<string, string>, images: any[]) {
    return images.map((_, i) => {
      let index = i + 1
      if (allowIndex && !isAllowIndex(index, allowIndex)) {
        return ''
      }
      data.index = `${index}`
      let keys = Object.keys(data)
      for (let k = 0; k < keys.length; k++) {
        const key = keys[k]
        let reg = new RegExp(`{${key}}`, 'ig')
        desFileTpl = desFileTpl.replace(reg, data[key])
      }
      return desFileTpl
    })
  }

  async function read_image_from_files_allow_null(files: (string | null)[]) {
    return await Promise.all(files.map((loc) => (loc ? readImageData(loc) : null)))
  }

  async function find_image_allow_null(
    baboon: ImageData,
    imagelist: (ImageData | null)[],
    info?: boolean,
    variant: number = 0,
  ) {
    let task = imagelist.map((screenshot, i) => {
      return async () => {
        // [baboon, screenshot] = [screenshot, baboon];

        if (screenshot) {
          if (info) {
            const { log } = console
            log(`[zero] search in ${i + 1}`)
          }

          if (variant < 1 && variant > 0) {
            variant = Math.floor(variant * 255)
          }
          return await imageSearch(screenshot, baboon, variant)
        }
        return null
      }
    })
    // promise all task
    return await Promise.all(task.map((fn) => fn()))
  }
}
