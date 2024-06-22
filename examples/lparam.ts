// interface Uiny32Prop {
//   low: number
//   hight: number
// }
// https://github.com/pierrec/js-cuint/tree/master
// pnpm add cuint
class UINT32 {
  low: number
  high: number
  remainder: UINT32 | null = null
  constructor(l: number | string, h?: number) {
    this.low = 0
    this.high = 0

    this.remainder = null
    if (typeof l == 'string') {
      return this.fromString.call(this, l, h)
    }
    if (typeof h == 'undefined') {
      return this.fromNumber.call(this, l)
    }

    return this.fromBits.call(this, l, h)
  }
  fromBits(l: number = 0, h: number = 0) {
    this.low = l | 0
    this.high = h | 0
    return this
  }
  fromNumber(value: number) {
    this.low = value & 0xffff
    this.high = value >>> 16
    return this
  }
  fromString(s: string, radix: number = 10) {
    let value = parseInt(s, radix || 10)
    this.low = value & 0xffff
    this.high = value >>> 16
    return this
  }
  toNumber() {
    // 65536=2^16
    return this.high * 65536 + this.low
  }
  toString(radix: number = 10) {
    // 65536=2^16
    return this.toNumber().toString(radix || 10)
  }
  /**
   *
   * @param other
   * @returns
   * @description
   * ```
   * low + low , high + high , x & 0xffff
   * ```
   */
  add(other: UINT32) {
    let a00 = this.low + other.low
    let a16 = a00 >>> 16

    a16 += this.high + other.high

    this.low = a00 & 0xffff
    this.high = a16 & 0xffff

    return this
  }
  subtract(other: UINT32) {
    // todo
    return this.add(other.clone().negate())
  }
  multiply(other: UINT32) {
    let a16 = this.high
    let a00 = this.low
    let b16 = other.high
    let b00 = other.low
    let c16, c00
    c00 = a00 * b00
    c16 = c00 >>> 16

    c16 += a16 * b00
    c16 &= 0xffff // Not required but improves performance
    c16 += a00 * b16

    this.low = c00 & 0xffff
    this.high = c16 & 0xffff

    return this
  }
  div(other: UINT32) {
    // todo
    if (other.low == 0 && other.high == 0) throw Error('division by zero')

    // other == 1
    if (other.high == 0 && other.low == 1) {
      this.remainder = new UINT32(0)
      return this
    }

    // other > this: 0
    if (other.greaterThan(this)) {
      this.remainder = this.clone()
      this.low = 0
      this.high = 0
      return this
    }
    // other == this: 1
    if (this.equals(other)) {
      this.remainder = new UINT32(0)
      this.low = 1
      this.high = 0
      return this
    }

    // Shift the divisor left until it is higher than the dividend
    var _other = other.clone()
    var i = -1
    while (!this.lessThan(_other)) {
      // High bit can overflow the default 16bits
      // Its ok since we right shift after this loop
      // The overflown bit must be kept though
      _other.shiftLeft(1, true)
      i++
    }

    // Set the remainder
    this.remainder = this.clone()
    // Initialize the current result to 0
    this.low = 0
    this.high = 0
    for (; i >= 0; i--) {
      _other.shiftRight(1)
      // If shifted divisor is smaller than the dividend
      // then subtract it from the dividend
      if (!this.remainder.lessThan(_other)) {
        this.remainder.subtract(_other)
        // Update the current result
        if (i >= 16) {
          this.high |= 1 << (i - 16)
        } else {
          this.low |= 1 << i
        }
      }
    }

    return this
  }

  negate() {
    let v = (~this.low & 0xffff) + 1
    this.low = v & 0xffff
    this.high = (~this.high + (v >>> 16)) & 0xffff
    return this
  }
  equals(other: UINT32) {
    return this.low == other.low && this.high == other.high
  }
  greaterThan(other: UINT32) {
    if (this.high > other.high) return true
    if (this.high < other.high) return false
    return this.low > other.low
  }
  lessThan(other: UINT32) {
    if (this.high < other.high) return true
    if (this.high > other.high) return false
    return this.low < other.low
  }
  or(other: UINT32) {
    this.low |= other.low
    this.high |= other.high
    return this
  }
  and(other: UINT32) {
    this.low &= other.low
    this.high &= other.high
    return this
  }
  not() {
    this.low = ~this.low
    this.high = ~this.high
    return this
  }
  xor(other: UINT32) {
    this.low ^= other.low
    this.high ^= other.high
    return this
  }

  shiftRight(n: number) {
    if (n > 16) {
      this.low = this.high >> (n - 16)
      this.high = 0
    } else if (n == 16) {
      this.low = this.high
      this.high = 0
    } else {
      this.low = (this.low >> n) | ((this.high << (16 - n)) & 0xffff)
      this.high >>= n
    }
    return this
  }

  shiftLeft(n: number, allowOverflow: boolean) {
    if (n > 16) {
      this.high = this.low << (n - 16)
      this.low = 0
      if (!allowOverflow) {
        this.high &= 0xffff
      }
    } else if (n == 16) {
      this.high = this.low
      this.low = 0
    } else {
      this.high = (this.high << n) | (this.low >> (16 - n))
      this.low = (this.low << n) & 0xffff
      if (!allowOverflow) {
        // Overflow only allowed on the high bits...
        this.high &= 0xffff
      }
    }

    return this
  }

  rotateRight(n: number) {
    let v = (this.high << 16) | this.low
    v = (v >>> n) | (v << (32 - n))
    this.low = v & 0xffff
    this.high = v >>> 16

    return this
  }

  rotateLeft(n: number) {
    let v = (this.high << 16) | this.low
    v = (v << n) | (v >>> (32 - n))
    this.low = v & 0xffff
    this.high = v >>> 16

    return this
  }
  clone() {
    return new UINT32(this.low, this.high)
  }
}
function u32(l: number | string, h?: number) {
  return new UINT32(l, h)
}

function jsonstro(json: any, trim: boolean = true) {
  let text = typeof json == 'string' ? json : JSON.stringify(json, null, 0)
  if (trim) {
    text = text.replace(/,"/gi, ',').replace(/":/gi, ':').replace(/{"/gi, '{')
  }
  return text
}
const { log } = console
interface IFlag {
  cRepeat: number
  scanCode: string | number
  fExtended: number
  fAltDown: number
  fRepeat: number
  fUp: number
}
type IFlagLike = Partial<IFlag>
function defaultIFlag(o?: IFlagLike) {
  return {
    cRepeat: 1,
    scanCode: 0,
    fExtended: 0,
    fAltDown: 0,
    fRepeat: 0,
    fUp: 0,
    ...(o ? o : {}),
  } as IFlag
}
const IFlagList = {
  'ctrl down': defaultIFlag({ fUp: 0, scanCode: 0x1d }), //
  'ctrl long-down': defaultIFlag({ fUp: 0, scanCode: 0x1d, fRepeat: 1 }), //
  'ctrl up': defaultIFlag({ fUp: 1, scanCode: 0x1d, fRepeat: 1 }),
  'v down': defaultIFlag({ fUp: 0, scanCode: 0x2f }), //
  'v char': defaultIFlag({ fUp: 0, scanCode: 0x2f }), //
  'v up': defaultIFlag({ fUp: 1, scanCode: 0x2f, fRepeat: 1 }), //
  'd down': defaultIFlag({ fUp: 0, scanCode: 0x20 }), //
  'd up': defaultIFlag({ fUp: 1, scanCode: 0x20, fRepeat: 1 }), //
}
/**
 *
 * @sample
 * ```
 * log(u16_fmt(MAKELONG(1, 0x1d), 16, 16)) // 00000000001D0001 // ctrl down done
 * log(u16_fmt(MAKELONG(1, 0x2f), 16, 16)) // 00000000002F0001 // V up and down done
 * // fail when ctrl up
 * log(u16_fmt(MAKELONG(1, 0x1d), 16, 16)) // EXPECT 00000000C01D0001 BUT 00000000001D0001
 * // from https://github.com/deskbtm/win32-ffi
 * ```
 */
export const MAKELONG = (cRepeat: number, scanCode: number): number => {
  return (cRepeat & 0xfff) | ((scanCode & 0xfff) << 16)
}

// v1
// export const MAKELONG = (cRepeat: number, scanCode: number): number => {
//   return (cRepeat & 0xfff) | ((scanCode & 0xfff) << 16)
// }

// Key.V
//
function bits_stro_from(i: number) {
  return u32(i).toString(2).padStart(32, '0')
}
function bits_select_from_strb(str: string, s: number, e: number) {
  return str
    .split('')
    .reverse()
    .slice(s, e + 1)
    .reverse()
    .join('')
}

function bits_as_flag_from_num_delt(i: number) {
  let u = u32(i)
  let res = {
    cRepeat: bits_select_from_u(u, 0, 15).toNumber(),
    scanCode: bits_select_from_u(u, 16, 23).toString(16).toUpperCase(),
    fExtended: bits_select_from_u(u, 24, 24).toNumber(),
    preversed: bits_select_from_u(u, 25, 28).toNumber(),
    fAltDown: bits_select_from_u(u, 29, 29).toNumber(),
    fRepeat: bits_select_from_u(u, 30, 30).toNumber(),
    fUp: bits_select_from_u(u, 31, 31).toNumber(),
  }
  // log(jsonstro(res))
  return res
}

function bits_stro_info(i: number) {
  let binstr = bits_stro_from(i)

  log(`input: bin-str:`)
  log(binstr)
  // log(binx)
  log(`input: bin-str:0-15`)
  log(bits_select_from_strb(binstr, 0, 15))
  log(`input: bin-str:16-23`)
  log(bits_select_from_strb(binstr, 16, 23))
  log(u32(bits_select_from_strb(binstr, 16, 23), 2).toString(16))

  log(`input: bin-str:24`)
  log(bits_select_from_strb(binstr, 24, 24))

  log(`input: bin-str:25-28`)
  log(bits_select_from_strb(binstr, 25, 28))

  log(`input: bin-str:29`)
  log(bits_select_from_strb(binstr, 29, 29))

  log(`input: bin-str:30`)
  log(bits_select_from_strb(binstr, 30, 30))

  log(`input: bin-str:31`)
  log(bits_select_from_strb(binstr, 31, 31))
}
function bits_as_flag_from_num_next(i: number) {
  let strb = bits_stro_from(i)
  let u32_from_strb = (s: string) => {
    return u32(s, 2)
  }
  let res = {
    cRepeat: u32_from_strb(bits_select_from_strb(strb, 0, 15)).toNumber(),
    scanCode: u32_from_strb(bits_select_from_strb(strb, 16, 23))
      .toString(16)
      .toUpperCase(),
    fExtended: u32_from_strb(bits_select_from_strb(strb, 24, 24)).toNumber(),
    preversed: u32_from_strb(bits_select_from_strb(strb, 25, 28)).toNumber(),
    fAltDown: u32_from_strb(bits_select_from_strb(strb, 29, 29)).toNumber(),
    fRepeat: u32_from_strb(bits_select_from_strb(strb, 30, 30)).toNumber(),
    fUp: u32_from_strb(bits_select_from_strb(strb, 31, 31)).toNumber(),
  }
  // log(jsonstro(res))
  return res
}

function bits_select_from_u(u: UINT32, s: number = 16, e: number = 23) {
  // return u
  //   .clone()
  //   .shiftLeft(e - s + 1, false)
  //   .shiftRight(32 - 16)
  //   .shiftRight(16 - (e - s + 1))
  return u
    .clone()
    .shiftLeft(31 - e, false)
    .shiftRight(s + 31 - e)
}
// cook lparam with spy++ data
/**
 *
 * @sample
 * ```
 * lparamCook({cRepeat:1,scanCode:29,fExtended:0,preversed:0,fAltDown:0,fRepeat:1,fUp:1}) //3223126017
 * ```
 */
function lparamCook(o: IFlagLike) {
  let { cRepeat, scanCode, fExtended, fAltDown, fUp, fRepeat } = defaultIFlag(o)
  let res: number
  // res = (fUp << 31) | (fRepeat << 30) | (fAltDown << 29) | (fExtended << 24) | (scanCode << 16) | cRepeat
  let isKeyUp = u32(fUp).shiftLeft(31, false)
  let isLastKeyDown = u32(fRepeat).shiftLeft(30, false)
  let isAltKeyDown = u32(fAltDown).shiftLeft(29, false)
  let isKeyIsExtendedKey = u32(fExtended).shiftLeft(24, false)
  let scanCodeFromOremOrKey = u32(scanCode, 16).shiftLeft(16, false)
  let keyRepeatCount = u32(cRepeat)
  // log(isLastKeyDown.toString(2))
  res = isKeyUp
    .or(isLastKeyDown)
    .or(isAltKeyDown)
    .or(isKeyIsExtendedKey)
    .or(scanCodeFromOremOrKey)
    .or(keyRepeatCount)
    .toNumber()
  return res
}

/**
 *
 * @sample
 * ```
 * lparamParse(0xc01d0001) // {cRepeat:1,scanCode:29,fExtended:0,preversed:0,fAltDown:0,fRepeat:1,fUp:1}
 * ```
 */
function lparamParse(i: number) {
  // bits_stro_info(i)
  // bits_as_flag_from_num_base(i)
  return bits_as_flag_from_num_next(i)
  // todo: use bits operation!!!
}
function lparamDebug(n: number) {
  log(n.toString(16).toUpperCase().padStart(8, '0'))
  log(n.toString(10))
  log(n.toString(2).toUpperCase().padStart(32, '0'))
}

function main() {
  // log(lparamParse(3223126017))
  let o = lparamParse(0xc01d0001)
  let n = lparamCook(o)

  log(jsonstro(o))
  lparamDebug(n)

  // todo: get scan code and isExtendKey from key name or key code
  //
  o.fUp = 0
  o.fRepeat = 0
  log(jsonstro(o))
  lparamDebug(lparamCook(o))

  // o = bits_as_flag_from_num_delt(0xc01d0001)
  // o = bits_as_flag_from_num_delt(n)
  // log(jsonstro(o))

  // use keyboard focus to publish to windows when non-system keys are pressed.
  // Non-system keys are keys that are pressed when the Alt key is not pressed

  // https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-keydown
  o.fUp = 0
  o.fRepeat = 0 // 0 or 1 // 0 for prev key press, 1 for prev key other
  o.fAltDown = 0 // awaly use 0
  // o.fExtended = 0 // 0 for non-extended key, 1 for extended key
  // o.scanCode = xx; this from key code based on OEM
  o.cRepeat = 0 // 0 or 1 // current repeat count when long press.
  log(jsonstro(o))
  lparamDebug(lparamCook(o))

  // https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-keyup
  // use keyboard focus to publish to windows when releasing non-system keys
  // when the Alt key is not pressed
  // or Keyboard keys pressed when window has keyboard focus
  o.fUp = 1 //aways 1
  o.fRepeat = 1 // awalys 1
  o.fAltDown = 0 // awayls 0
  // o.fExtended = 0 // 0 for non-extended key, 1 for extended key
  // o.scanCode = xx; this from key code based on OEM
  o.cRepeat = 1 // alway 1
  log(jsonstro(o))
  lparamDebug(lparamCook(o))

  // https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-setfocus
  // Send to window after gaining keyboard focus.
  // set wparam and lparam to 0

  // https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-killfocus
  // Send to window immediately before losing keyboard focus
  // set wparam and lparam to 0

  // https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-char
  o.fUp = 1 // 0 or 1
  o.fRepeat = 1 // 0 for prev key press, 1 for prev key other
  o.fAltDown = 0 // 1 for alt key press, 0 for alt key other
  o.fExtended = 0 // 0 for non-extended key, 1 for extended key
  // o.scanCode = xx; this from key code based on OEM
  o.cRepeat = 1 // current repeat count when long press.
  log(jsonstro(o))
  lparamDebug(lparamCook(o))

  // https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-sysdeadchar
  // https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-deadchar
  // https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-unichar
}
main()
