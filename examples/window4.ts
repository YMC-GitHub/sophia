import { Window, saveImageData } from '../index'
import type { ImageData, Rect, Point, WindowView } from '../index'

import { chaintask } from './nano-fune'
// import { formatDate } from "./nano-time";
// import { formatDate } from "../lib/nano-point";
import {
  sleep,
  jsonstro,
  findWindowList,
  readRuntimeWindowList,
  readWindowListFromRWL,
  captureWindowList,
} from './winx'

// import { Window, saveImageData } from '../'
// import type { ImageData } from '../'

const { log } = console
main()

async function main() {
  // list all windows
  log(`[zero] read window list`)
  // let { windows, info } = await getWindowListInfo();
  let mxClassName = 'GAMECLIENT'
  let mxTitle = '2.144.11258.3388'
  // mxTitle = 'Clash for Windows'

  //todo(ncli): add ncli like  mhzx list/window --title xx --class-name xx --id xx
  let { windows, info } = await findWindowList({ title: mxTitle })
  // print info to termnal window ? do
  // log(info);
  // print info to termnal window as json-stro format ? do
  log(jsonstro(info))

  // save info to json file ? do
  // ...

  // 2.144.11258.3388

  // info = sortWindowInfo(info);
  // print sorted info's title to termnal window ? do
  // log(info.map((v) => v.title).join(`\n`));

  let view = await windows[0].getWindowView()
  log(jsonstro(view))
  // view = { x: 1, y: 26, width: 1024, height: 768 };
  // log(jsonstro(view));

  let runtimeWindows = readRuntimeWindowList({ windows, info })
  ;({ windows, info } = readWindowListFromRWL(runtimeWindows))
  // print sorted info's title to termnal window ? do
  log(info.map((v) => v.title).join(`\n`))

  setInterval(async () => {
    let index = 1
    let win = windows[index - 1]
    let pos = await win.getMousePos()
    log(`[zero] mouse position in window ${index} :`, jsonstro(pos))
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

    // await win.keyboardToggleKey(knks, true, false); // down
    // await sleep(50); // please wait for window fire keys
    // await win.keyboardToggleKey(knks, false, false); //up
    // await sleep(50); // please wait for window fire keys

    // await win.typing("我不是脚本"); // work with sleep
    // await win.typing("根据按键精灵原理，模拟玩家操作，但不是按键精灵"); // work with sleep

    await win.keyboardToggleKey(['enter'], true, false) // down
    // await win.keyboardToggleKey(["enter"], false, false); //up

    // click enter
    coords = {
      x: 344,
      y: 708,
    }

    let wininfo = info[index - 1]
    let { width, height, left, top } = wininfo.rect

    // coords = get_random_coord_in_win_center_rect(width, height);
    // // mock click left mouse button in coord in widnow ? do
    // // 0,2,4,
    // await win.mouseToggle(coords, "left", true);
    // await sleep(50);
    // await win.mouseToggle(coords, "left", false);
    // await sleep(50);
    // log(`coord:`, jsonstro(coords));

    // let msg = `go to ${point_stro(coords)}`;
    // log(msg);
    // // await win.typing(`go to ${point_stro(coords)}`); // work with sleep

    // await sleep(50);
    // await win.keyboardToggleKey(["enter"], true, false); // down
  }, 1000 * 2)

  // sometime get black pixel in mhzx, but this not occur in other window , eg. clash .
  setInterval(async () => {
    log(`[zero] record window :`)
    // let now = new Date();
    // let strn = formatDate("yyyy_MM_dd_HH_mm_ss", now);
    await captureWindowList({
      title: mxTitle,
      // id: String(wininfo.id),
      // view: view,
      desFile: `examples/images/{index}_main.png`,
      // desFile: `examples/images/{index}_main_${strn}.png`,
      takeScreen: true,
      foreGround: true, // when window length > 1 & like mhzx , it can not active in virtual window!
    })
  }, 1000 * 15)
}
