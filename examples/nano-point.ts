import type { Point } from '../'
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
// refer:
// https://github.com/konvajs/konva/blob/master/src/Layer.ts

export function offset_get_map_by_random() {
  let max = INTERSECTION_OFFSETS_LEN
  let min = 0
  let index = Math.floor(Math.random() * (max - 1 - min) + min)
  return INTERSECTION_OFFSETS[index]
}
export function offset_get(map: { x: number; y: number }, w: number, h: number) {
  let { x, y } = map
  return { x: Math.floor((x * w) / 2), y: Math.floor((y * h) / 2) }
}
export function offset_get_map_by_key(key: string = '0') {
  let index = '02468'.indexOf(key)
  index = index >= 0 ? index : 0
  return INTERSECTION_OFFSETS[index]
}

function point_move(c: Point, offset: Point) {
  return {
    x: c.x + offset.x,
    y: c.y + offset.y,
  } as Point
}

// feat(core): point_stro - let point to stro
// feat(core): output `(x,y)`
export function point_stro(c: Point) {
  let { x, y } = c
  return `(${x},${y})`
}

export function get_random_coord_in_win_center_rect(ww: number, wh: number, rw: number = 200, rh: number = 200) {
  let center = {
    x: Math.floor(ww / 2),
    y: Math.floor(wh / 2),
  }
  // log(`center:`, jsonstro(center));

  let map = offset_get_map_by_random()
  // log(`map:`, jsonstro(map));
  let offset = offset_get(map, rw, rh)
  // offset = offset_get(offset_get_map_by_key("2"), 50, 50);
  let coords = point_move(center, offset)
  return coords
}

// feat(core): point_from_srto_to_numa
// feat(core): from stro `[x1,y1],[x2,y2]` to numa `[x1,y1,x2,y2]`
// feat(core): from stro `[[x1,y1],[x2,y2]]` to numa `[x1,y1,x2,y2]`

export function point_from_srto_to_numa(point: string): number[] {
  let numbers: number[]
  //del [ and ] // why not use ( and ) ? to avoid using different chars!
  //to array with , char
  //trim each string char
  //like-number string  to number
  numbers = point
    .replace(/\[/gi, '')
    .replace(/\]/gi, '')
    .split(/,/)
    .map((item) => parseInt(item.trim()))
  return numbers
}
