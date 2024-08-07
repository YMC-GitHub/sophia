/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function fib(n: number): number
export interface Point {
  x: number
  y: number
}
export interface Rect {
  left: number
  top: number
  right: number
  bottom: number
  width: number
  height: number
}
export interface WindowView {
  x: number
  y: number
  width: number
  height: number
}
export interface WindowMetaInfo {
  id: number
  title: string
  className: string
  rect: Rect
}
export interface LParamFlag {
  scanCode: number
  repeatCount: number
  transitionState: boolean
  isExtended: boolean
  previousKeyState: boolean
  contextCode: boolean
}
export interface Color {
  r: number
  g: number
  b: number
}
export const MAGENTA: Color
export function readImageData(path: string): Promise<ImageData>
export function saveImageData(path: string, imageData: ImageData): Promise<void>
export function imageSearch(source: ImageData, target: ImageData, variant?: number | undefined | null, transColor?: Color | undefined | null): Promise<Point | null>
export function multipleImageSearch(source: ImageData, target: ImageData, variant?: number | undefined | null, transColor?: Color | undefined | null): Promise<Array<Point>>
export const enum Modifiers {
  Alt = 1,
  AltGraph = 2,
  CapsLock = 4,
  Control = 8,
  Fn = 16,
  FnLock = 32,
  Meta = 64,
  NumLock = 128,
  ScrollLock = 256,
  Shift = 512,
  Symbol = 1024,
  SymbolLock = 2048,
  Hyper = 4096,
  Super = 8192
}
export const enum Key {
  None = 0,
  Back = 8,
  Tab = 9,
  LineFeed = 10,
  Clear = 12,
  Enter = 13,
  Shift = 16,
  Control = 17,
  Alt = 18,
  Pause = 19,
  CapsLock = 20,
  Esc = 27,
  Space = 32,
  PageUp = 33,
  PageDown = 34,
  End = 35,
  Home = 36,
  ArrowLeft = 37,
  ArrowUp = 38,
  ArrowRight = 39,
  ArrowDown = 40,
  Insert = 45,
  Delete = 46,
  D0 = 48,
  D1 = 49,
  D2 = 50,
  D3 = 51,
  D4 = 52,
  D5 = 53,
  D6 = 54,
  D7 = 55,
  D8 = 56,
  D9 = 57,
  A = 65,
  B = 66,
  C = 67,
  D = 68,
  E = 69,
  F = 70,
  G = 71,
  H = 72,
  I = 73,
  J = 74,
  K = 75,
  L = 76,
  M = 77,
  N = 78,
  O = 79,
  P = 80,
  Q = 81,
  R = 82,
  S = 83,
  T = 84,
  U = 85,
  V = 86,
  W = 87,
  X = 88,
  Y = 89,
  Z = 90,
  LeftWin = 91,
  RightWin = 92,
  Apps = 93,
  Sleep = 95,
  NumPad0 = 96,
  NumPad1 = 97,
  NumPad2 = 98,
  NumPad3 = 99,
  NumPad4 = 100,
  NumPad5 = 101,
  NumPad6 = 102,
  NumPad7 = 103,
  NumPad8 = 104,
  NumPad9 = 105,
  Multiply = 106,
  Add = 107,
  Separator = 108,
  Subtract = 109,
  Decimal = 110,
  Divide = 111,
  F1 = 112,
  F2 = 113,
  F3 = 114,
  F4 = 115,
  F5 = 116,
  F6 = 117,
  F7 = 118,
  F8 = 119,
  F9 = 120,
  F10 = 121,
  F11 = 122,
  F12 = 123,
  F13 = 124,
  F14 = 125,
  F15 = 126,
  F16 = 127,
  F17 = 128,
  F18 = 129,
  F19 = 130,
  F20 = 131,
  F21 = 132,
  F22 = 133,
  F23 = 134,
  F24 = 135,
  NumLock = 144,
  ScrollLock = 145,
  LeftShift = 160,
  RightShift = 161,
  LeftControl = 162,
  RightControl = 163,
  LeftAlt = 164,
  RightAlt = 165
}
export interface Process {
  pid: number
  name: string
}
export const enum ProcessAccess {
  AllAccess = 0,
  CreateProcess = 1,
  CreateThread = 2,
  Delete = 3,
  DupHandle = 4,
  QueryInformation = 5,
  QueryLimitedInformation = 6,
  ReadControl = 7,
  SetInformation = 8,
  SetLimitedInformation = 9,
  SetQuota = 10,
  SetSessionId = 11,
  Synchronize = 12,
  Terminate = 13,
  VmOperation = 14,
  VmRead = 15,
  VmWrite = 16,
  WriteDac = 17,
  WriteOwner = 18
}
export function openProcess(access: ProcessAccess, pid: number): Promise<OpenedProcess>
export function getProcesses(): Promise<Array<Process>>
export const enum MouseButton {
  Left = 0,
  Right = 1,
  Middle = 2
}
export function getScreenSize(): Promise<Point>
export function takeScreenshot(x: number, y: number, width: number, height: number): Promise<ImageData>
export function listWindow(): Promise<Array<Window>>
/** alias of list_window */
export function getAllWindows(): Promise<Array<Window>>
export function getForegroundWindow(): Promise<Window | null>
/** create a Window instance with pid */
export function findWindowByPid(pid: number): Promise<Window | null>
/**
 * create a Window instance with title
 *
 * NOTE
 *
 *
 */
export function findWindowByTitle(title: string): Promise<Window | null>
/** */
export function findWindowByClassName(classname: string): Promise<Window | null>
/**
 * create a Window instance with title substring
 *
 * NOTE
 *
 * list window ->  find
 */
export function findWindowContainsTitle(title: string): Promise<Window | null>
/**
 * create a Window instance with class name substring
 *
 * NOTE
 *
 * list window ->  find
 */
export function findWindowContainsClassName(name: string): Promise<Window | null>
export class ImageData {
  data: Array<number>
  width: number
  height: number
  pixelWidth: number
}
export class Keyboard {
  static press(key: number): Promise<void>
  static release(key: number): Promise<void>
  static click(key: number): Promise<void>
  static typing(text: string): Promise<void>
  static registerHotkey(mods: Array<Modifiers>, key: Key, callback: (...args: any[]) => any): number
  static unregisterHotkey(id: number): void
}
export class OpenedProcess {
  readMemoryBool(address: bigint): Promise<boolean>
  readMemoryChainBool(baseAddress: bigint, offsets: Array<bigint>): Promise<boolean>
  readMemoryUint8(address: bigint): Promise<bigint>
  readMemoryChainUint8(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryInt8(address: bigint): Promise<bigint>
  readMemoryChainInt8(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryUint16(address: bigint): Promise<bigint>
  readMemoryChainUint16(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryInt16(address: bigint): Promise<bigint>
  readMemoryChainInt16(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryUint32(address: bigint): Promise<bigint>
  readMemoryChainUint32(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryInt32(address: bigint): Promise<bigint>
  readMemoryChainInt32(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryUint64(address: bigint): Promise<bigint>
  readMemoryChainUint64(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryInt64(address: bigint): Promise<bigint>
  readMemoryChainInt64(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryUsize(address: bigint): Promise<bigint>
  readMemoryChainUsize(baseAddress: bigint, offsets: Array<bigint>): Promise<bigint>
  readMemoryFloat32(address: bigint): Promise<number>
  readMemoryChainFloat32(baseAddress: bigint, offsets: Array<bigint>): Promise<number>
  readMemoryFloat64(address: bigint): Promise<number>
  readMemoryChainFloat64(baseAddress: bigint, offsets: Array<bigint>): Promise<number>
  writeMemoryBool(address: bigint, value: boolean): Promise<void>
  writeMemoryChainBool(baseAddress: bigint, offsets: Array<bigint>, value: boolean): Promise<void>
  writeMemoryUint8(address: bigint, value: bigint): Promise<void>
  writeMemoryChainUint8(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryInt8(address: bigint, value: bigint): Promise<void>
  writeMemoryChainInt8(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryUint16(address: bigint, value: bigint): Promise<void>
  writeMemoryChainUint16(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryInt16(address: bigint, value: bigint): Promise<void>
  writeMemoryChainInt16(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryUint32(address: bigint, value: bigint): Promise<void>
  writeMemoryChainUint32(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryInt32(address: bigint, value: bigint): Promise<void>
  writeMemoryChainInt32(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryUint64(address: bigint, value: bigint): Promise<void>
  writeMemoryChainUint64(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryInt64(address: bigint, value: bigint): Promise<void>
  writeMemoryChainInt64(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryUsize(address: bigint, value: bigint): Promise<void>
  writeMemoryChainUsize(baseAddress: bigint, offsets: Array<bigint>, value: bigint): Promise<void>
  writeMemoryFloat32(address: bigint, value: number): Promise<void>
  writeMemoryChainFloat32(baseAddress: bigint, offsets: Array<bigint>, value: number): Promise<void>
  writeMemoryFloat64(address: bigint, value: number): Promise<void>
  writeMemoryChainFloat64(baseAddress: bigint, offsets: Array<bigint>, value: number): Promise<void>
}
export class Mouse {
  static move(x: number, y: number): Promise<void>
  static press(button: MouseButton): Promise<void>
  static release(button: MouseButton): Promise<void>
  static click(button: MouseButton, x: number, y: number): Promise<void>
  static getPosition(): Promise<Point>
}
export class Window {
  static getAllWindows(): Promise<Array<Window>>
  static getForegroundWindow(): Promise<Window | null>
  static findWindowByPid(pid: number): Promise<Window | null>
  static findWindowByTitle(title: string): Promise<Window | null>
  static findWindowByClassName(classname: string): Promise<Window | null>
  static findWindowBySubTitle(title: string): Promise<Window | null>
  static findWindowBySubClassName(title: string): Promise<Window | null>
  fromActive(): Promise<Window | null>
  fromTitle(title: string): Promise<Window | null>
  fromClassName(name: string): Promise<Window | null>
  fromPid(pid: number): Promise<Window | null>
  fromSubTitle(title: string): Promise<Window | null>
  fromSubClassName(name: string): Promise<Window | null>
  asRawHwnd(): bigint
  getId(): Promise<number>
  getTitle(): Promise<string>
  getClassName(): Promise<string>
  getWindowRect(): Promise<Rect>
  /**
   * like keysender's workwindow.getWindowView
   *
   */
  getWindowView(): Promise<WindowView>
  getWindowMetaInfo(): Promise<WindowMetaInfo>
  getMousePos(): Promise<Point>
  setPosition(x: number, y: number): Promise<void>
  setSize(width: number, height: number): Promise<void>
  /**
   * like keysender's workwindow.isForeground
   *
   */
  isForeground(): Promise<boolean>
  foreground(): Promise<boolean>
  /**
   * like keysender's workwindow.setForeground
   *
   */
  setForeground(): Promise<boolean>
  /**
   * like keysender's workwindow.isOpen
   *
   */
  isOpen(): Promise<boolean>
  isMinimized(): Promise<boolean>
  show(): Promise<void>
  hide(): Promise<void>
  minimize(): Promise<void>
  maximize(): Promise<void>
  close(): Promise<void>
  kill(): Promise<void>
  isVisible(): Promise<boolean>
  /**
   *
   * not move coords to last coord in if is_absolute
   */
  mouseMove(coords: Point, isAbsolute: boolean): Promise<void>
  mouseToggle(coords: Point, button: string, isButtonDown: boolean): Promise<void>
  mouseWheelScroll(coords: Point, isUp: boolean): Promise<void>
  typing(text: string): Promise<void>
  keyboardToggleKey(keys: Array<string>, isKeyDown: boolean, isPrevKeyDown: boolean): Promise<void>
  static decodeLparamValue(value: number): Promise<LParamFlag>
  static cookLparamValue(vk: number, flag: LParamFlag): Promise<number>
  capture(): Promise<ImageData>
  captureArea(x: number, y: number, width: number, height: number): Promise<ImageData>
}
