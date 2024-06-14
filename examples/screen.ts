import { getScreenSize, takeScreenshot, saveImageData } from '../index'
import type { ImageData } from '../index'

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

async function main() {
  // msgLog(`[zero] demo: read screen size:`);
  let screenSize = await getScreenSize()
  // successLog(jsonStroify(screenSize, true));

  // msgLog(`[zero] demo: take screenshot:`);

  let imgdata: ImageData
  imgdata = await takeScreenshot(0, 0, screenSize.x, screenSize.y)
  // writeFileSync(``,imgdata.data)
  // msgLog(`[zero] demo: save image data:`);

  await saveImageData(`runtime-images-sync-screen.png`, imgdata)

  // imgdata = await takeScreenshot(0, 0, screenSize.x / 2, screenSize.y / 2)
  // await saveImageData(`runtime-images-sync-screen-0123.png`, imgdata)
}
main()
