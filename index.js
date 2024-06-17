/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'sophia.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./sophia.android-arm64.node')
          } else {
            nativeBinding = require('@yors/sophia-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'sophia.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./sophia.android-arm-eabi.node')
          } else {
            nativeBinding = require('@yors/sophia-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'sophia.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sophia.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@yors/sophia-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'sophia.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sophia.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@yors/sophia-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'sophia.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sophia.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@yors/sophia-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'sophia.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./sophia.darwin-universal.node')
      } else {
        nativeBinding = require('@yors/sophia-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'sophia.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./sophia.darwin-x64.node')
          } else {
            nativeBinding = require('@yors/sophia-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'sophia.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sophia.darwin-arm64.node')
          } else {
            nativeBinding = require('@yors/sophia-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'sophia.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./sophia.freebsd-x64.node')
      } else {
        nativeBinding = require('@yors/sophia-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'sophia.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sophia.linux-x64-musl.node')
            } else {
              nativeBinding = require('@yors/sophia-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'sophia.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sophia.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@yors/sophia-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'sophia.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sophia.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@yors/sophia-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'sophia.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sophia.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@yors/sophia-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'sophia.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sophia.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('@yors/sophia-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'sophia.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sophia.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('@yors/sophia-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'sophia.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sophia.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('@yors/sophia-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'sophia.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sophia.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('@yors/sophia-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'sophia.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sophia.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('@yors/sophia-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { fib, ImageData, MAGENTA, readImageData, saveImageData, imageSearch, multipleImageSearch, Modifiers, Key, Keyboard, ProcessAccess, OpenedProcess, openProcess, getProcesses, MouseButton, Mouse, getScreenSize, takeScreenshot, Window, listWindow, getAllWindows, getForegroundWindow, findWindowByPid, findWindowByTitle, findWindowByClassName, findWindowContainsTitle, findWindowContainsClassName } = nativeBinding

module.exports.fib = fib
module.exports.ImageData = ImageData
module.exports.MAGENTA = MAGENTA
module.exports.readImageData = readImageData
module.exports.saveImageData = saveImageData
module.exports.imageSearch = imageSearch
module.exports.multipleImageSearch = multipleImageSearch
module.exports.Modifiers = Modifiers
module.exports.Key = Key
module.exports.Keyboard = Keyboard
module.exports.ProcessAccess = ProcessAccess
module.exports.OpenedProcess = OpenedProcess
module.exports.openProcess = openProcess
module.exports.getProcesses = getProcesses
module.exports.MouseButton = MouseButton
module.exports.Mouse = Mouse
module.exports.getScreenSize = getScreenSize
module.exports.takeScreenshot = takeScreenshot
module.exports.Window = Window
module.exports.listWindow = listWindow
module.exports.getAllWindows = getAllWindows
module.exports.getForegroundWindow = getForegroundWindow
module.exports.findWindowByPid = findWindowByPid
module.exports.findWindowByTitle = findWindowByTitle
module.exports.findWindowByClassName = findWindowByClassName
module.exports.findWindowContainsTitle = findWindowContainsTitle
module.exports.findWindowContainsClassName = findWindowContainsClassName
