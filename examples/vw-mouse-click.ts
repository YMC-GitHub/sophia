import { Window, saveImageData } from '..'
import type { ImageData, Rect, Point, WindowView } from '..'

import { chaintask } from './nano-fune'
import { formatDate } from './nano-time'
// import { formatDate } from "./nano-point";
import { isAllowIndex } from './nano-stre'
import { point_stro, point_from_srto_to_numa } from './nano-point'

import { getNanoFromArgvAndDefaultFlag as getNano } from './ncli-util'

import {
  sleep,
  jsonstro,
  findWindowList,
  readRuntimeWindowList,
  readWindowListFromRWL,
  captureWindowList,
  searchImage,
} from './winx'

// import { Window, saveImageData } from '../'
// import type { ImageData } from '../'

async function main() {
  let argv = process.argv.slice(2)
  const { log } = console
  // log(argv.join(" "));

  let nano = getNano(argv, {})
  log(`[info] nano: `, jsonstro(nano))

  // let [src, des] = argv;
  let flag = nano.flags as Record<string, any>
  let { title, outFile, rightNow, eachTime, afterTime, allowIndex } = flag

  // list all windows
  log(`[zero] read window list:`)
  // let { windows, info } = await getWindowListInfo();
  // let mxClassName = "GAMECLIENT";

  title = title ? title : '2.144.11258.3388'

  outFile = outFile ? outFile : `runtime/window/{index}_main.png`

  allowIndex = allowIndex ? `${allowIndex}` : ''
  //todo(ncli): add ncli like  mhzx list/window --title xx --class-name xx --id xx
  let { windows, info } = await findWindowList({ title })
  let view = await windows[0].getWindowView()
  let runtimeWindows = readRuntimeWindowList({ windows, info })
  ;({ windows, info } = readWindowListFromRWL(runtimeWindows))
  log(`[zero] window title:`)
  log(info.map((v) => v.title).join(`\n`))

  // let assetSrc = 'assets/images/map/{mapname}.png'

  // let matchedClosing = await searchImage({
  //   title,
  //   src: assetSrc.replace(/{mapname}/gi, 'pannel-m-miji-opened-close'),
  //   // allowIndex: `1`,
  //   // allowIndex: `1,2,3,4,5`,
  // })
  // log(jsonstro(matchedClosing))

  const fn = async function () {
    let tasks = windows.map((win, i) => {
      return async () => {
        let index = i + 1
        if (allowIndex && !isAllowIndex(index, allowIndex)) {
          return null
        }
        let coords = {
          x: 208,
          y: 708,
        }
        // await win.mouseToggle(coords, "left", true);
        // await sleep(50);
        // await win.keyboardToggleKey(["ctrl", "V"], true, false); // down
        // await win.keyboardToggleKey(["ctrl", "V"], false, false); //up
        // 2.144.11258.3388

        // hey ,i am robot!
        let knks = ['ctrl', '1'] //work with sleep
        // knks = ["ctrl", "V"]; //work with sleep
        knks = ['tab'] //work with sleep
        knks = ['f8'] //work with sleep. not work F8 (tofix)

        // let info = await win.getWindowView();
        // log(jsonstro(info));
        // 6,163,159,214

        // log(jsonstro({ x: 305 + 100, y: 780 + 200 + 100 }));
        // 405,1080
        // 267,746
        // general:[86,128]
        // proxies:[86,183]
        // profiles:[86,257]
        // let [x, y] = point_from_srto_to_numa('[86,257]')
        // await ocBaoguoClick(win, { x, y })

        // await ocBaoguoClick(win, { x: 305 + 100, y: 780 + 200 + 100 });

        // await ocBaoguoClick(win, { x: 132, y: 157 });

        let mpos = await win.getMousePos()
        log(`[zero] mouse in window: `, jsonstro(mpos))
        mpos.y -= 26
        // open /close baoguo
        await ocBaoguoClick(win, mpos)
        // await ocBaoguo(win, ["alt", "e"]); //done
      }
    })
    let res = await chaintask(tasks)
  }
  // await fn();

  log(`[zero] run shedule`)
  if (rightNow) {
    log(`[zero] run at once right now`)
    await fn()
  }
  // // sometime get black pixel in mhzx, but this not occur in other window , eg. clash .
  if (eachTime) {
    log(`[zero] run each time`)
    setInterval(fn, 1000 * eachTime)
  }

  if (afterTime) {
    log(`[zero] run after time ${afterTime}`)
    setTimeout(fn, 1000 * afterTime)
  }
}

async function ocBaoguo(win: Window, knks: string[] = ['alt', 'e']) {
  // knks = ["alt", "e"]; //work with sleep. not work F8 (tofix)
  await win.keyboardToggleKey(knks, true, false) // down
  await sleep(50) // please wait for window fire keys
  await win.keyboardToggleKey(knks, false, false) //up
  await sleep(50) // please wait for window fire keys
}
async function ocBaoguoClick(
  win: Window,
  coord: { x: number; y: number } = {
    x: 666,
    y: 763,
  },
) {
  // fix coord
  // coord.x += 405 - 267
  // coord.y += 1080 - 746

  // await win.mouseMove(coord, false)
  await win.mouseToggle(coord, 'left', true)
  await sleep(50)
  await win.mouseToggle(coord, 'left', false)
  await sleep(50)
}
main()
