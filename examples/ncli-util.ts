// args-stro to args nano
// --path "tauri-app" --bucket-loc "bucket/{name}.json" --pkg-org "yors"
// --browser-download-url-prefix "https://github.com/YMC-GitHub/yowa/releases/download/v{version}"

// utxt-strm to args default flag

// utxt-strm to args-objm

// utxt-strm to args flag keys

// utxt-strm to env flag

// get flag from process env

// feat(core): NanoFlagLike - bind any value with string name in flag
export type NanoFlagLike = Record<string, any>

// feat(core): nanoFlagShimValueExtract - shim -  extract value of flag
/**
 * get obj val only in values  - ts extract like
 * @sample
 * ```
 * // {a:'b',c:'',d:undefined} ->  {a:'b',c:''}
 * nanoFlagShimValueExtract({a:'b',c:'',d:undefined},[''])
 * ```
 */
export function nanoFlagShimValueExtract(data: NanoFlagLike, values: any = [undefined]) {
  const res: NanoFlagLike = {}
  Object.keys(data).forEach((v) => {
    let val = data[v]
    if (values.includes(val)) {
      res[v] = val
    }
  })
  return res
}

// feat(core): nanoFlagShimKeysPassed - shim - pick by keys
/**
 *
 * @sample
 * ```
 * import { pickbykeys as nanoflagshimpickbykeys } from "nano-flag-shim-keys-passed"
 * nanoflagshimpickbykeys(nl, custom);
 * ```
 */
export function nanoFlagShimKeysPassed(keys: string[], flag: NanoFlagLike) {
  let res: NanoFlagLike = {}
  for (let index = 0; index < keys.length; index++) {
    const key = keys[index]
    res[key] = flag[key]
  }
  return res
}

// keys camelize
// keys underscode
// keys upper
// keys slug
/**
 *
 * @sample
 * ```
 * humanize('per_page')// Per page
 * humanize('per-page')// Per page
 * ```
 * @description
 * ```
 * ## idea
 * - [x] replace multi - or _ to one space
 * - [x] add space to the char that is uppercase and is not the first index
 * - [x] the first char to upper ,other lowercase
 * ```
 */
export function humanize(s: string) {
  return s
    .replace(/(?:^\w|[A-Z_-]|\b\w)/g, (word, index) => {
      let res = ''
      // log(word, index); //desc: for debug
      // feat(core): replace multi - or _ to one space
      res = word.replace(/[-_]+/g, ' ')
      // feat(core): add space to the char that is uppercase and is not the first index
      res = index !== 0 ? res.replace(/[A-Z]/, ' $&') : res
      // feat(core): the first char to upper ,other lowercase
      return index === 0 ? res.toUpperCase() : res.toLowerCase()
    })
    .replace(/\s+/g, ' ')
}

export function slugify(s: string) {
  return humanize(s)
    .replace(/(?:^\w|[A-Z]|\b\w)/g, (word) => word.toLowerCase())
    .replace(/\s+/g, '-')
}

export function camelize(s: string) {
  return humanize(s)
    .replace(/(?:^\w|[A-Z]|\b\w)/g, (word, index) => (index === 0 ? word.toLowerCase() : word.toUpperCase()))
    .replace(/\s+/g, '')
}

export function underscoped(s: string) {
  return humanize(s)
    .replace(/(?:^\w|[A-Z]|\b\w)/g, (word) => word.toLowerCase())
    .replace(/\s+/g, '_')
}

export function classify(s: string) {
  return humanize(s)
    .replace(/(?:^\w|[A-Z]|\b\w)/g, (word) => word.toUpperCase())
    .replace(/\s+/g, '')
}

export function swapCase(s: string) {
  return s.replace(/(?:^\w|[A-Z-a-z]|\b\w)/g, (word) => {
    if (/[A-Z]/.test(word)) {
      return word.toLowerCase()
    }
    return word.toUpperCase()
  })
}

// ouhv with flag
export function getFlagFromProcessEnv(keys: string[]) {
  let stdkeys = keys.map((v) => underscoped(v).toUpperCase()) // eg. a-b -> A_B
  // console.log(stdkeys)
  if (process?.env) {
    return nanoFlagShimKeysPassed(stdkeys, process.env)
  }
  return {}
}

/**
 * parse cli input to node.js boolean,number,null,undefined,string
 * @sample
 * ```ts
 * StrvParse('true') // true
 * StrvParse('false') // false
 * StrvParse('1') // 1
 * StrvParse('null') // null
 * StrvParse('undefined') // undefined
 * StrvParse('zero') // 'zero'
 * StrvParse('1','string') // '1'
 * StrvParse(1,'string') // '1'
 * // why use ?
 * // parse value of usage objo
 * ```
 */
/* eslint-disable-next-line @typescript-eslint/no-explicit-any */
export function StrvParse(thing: any, type: string = '') {
  if (type === 'string') {
    return String(thing)
  }
  // case:  exp for true. eg: true-string or true-boolean ( or other custom exp(todo))
  // if (isOneOfThem(thing, ['true', true])) {
  //     return true
  // }

  if ([true].includes(thing) || strIsOneOfThem(thing, ['true'])) {
    return true
  }

  // case:  exp for false.
  // if (isOneOfThem(thing, ['false', false])) {
  //     return true
  // }
  if ([false].includes(thing) || strIsOneOfThem(thing, ['false'])) {
    return false
  }

  // case:  exp for number.
  if (Number(thing)) {
    return Number(thing)
  }

  if ([null].includes(thing) || strIsOneOfThem(thing, ['null'])) {
    return null
  }

  if ([undefined].includes(thing) || strIsOneOfThem(thing, ['undefined'])) {
    return undefined
  }

  // case: other string
  return String(thing)
}

function strIsOneOfThem(one: string, them: string[] = []) {
  // let reg = new RegExp(`^${one}$`, "i");
  // return them.some((v) => reg.test(v));
  // fix Invalid regular expression Nothing to repeat when pass 'xx**'
  return them.some((v) => v.toLowerCase() == one.toLowerCase())
}

export type NanoPlainValue = string
export type NanoParsedValue = boolean | number | undefined | null | string

export type NanoArgvs = string[]
export type NanoExtras = string[]

// export type NanoFlags = Record<string, any>
export type NanoFlags = Record<string, NanoPlainValue>
export type NanoParsedFlags = Record<string, NanoParsedValue>
export type NanoStrvFlag = NanoFlags //alias
export type NanoJssvFlag = NanoParsedFlags //alias

// export type NanoArgsMap = [string, any][]
export type NanoArgsMap = [string, NanoPlainValue][]
export type NanoParsedArgsMap = [string, NanoParsedValue][]

export interface Nano {
  flags: NanoParsedFlags
  argv: NanoArgvs
  extras: NanoExtras
}

/**
 * parse cli cmd string
 * @sample
 * ```ts
 * nanoargs(process.argv)
 * nanoargs(`ns cmd -a -b -c -- -a -b -c`)
 * nanoargs(`ns subns cmd -a -b -c -- -a -b -c`)
 * nanoargs(`ns subns subcmd -a -b -c -- -a -b -c`)
 *
 * ```
 */
export function nanoargs(
  input: string | string[],
  util: {
    nanoFlagParse: (...args: any[]) => NanoFlags | NanoParsedFlags
  } = {
    nanoFlagParse: nanoFlagParse,
  },
) {
  // 1.
  // feat(core): arrayify input to array when it is js-string
  const stra = nanoArgvStraSimple(input)

  // 2. ini extras , args , argvs
  let extras: NanoExtras = []
  let args: string[] = []

  // 3. get extras and head
  // feat(core): support extras when '--' bind to ouput.extras
  ;({ tail: extras, head: args } = nanoArgsStraDecode(stra))

  // 4. get args map and argvs from head of input
  const { argvs, argsMap } = nanoArgsHeadDecode(args)

  // 5. get flags
  const flags = nanoArgsHeadKvpDecode(argsMap)
  // console.log(flags)
  // 6. parse value in flags
  return {
    flags: util.nanoFlagParse(flags),
    argv: argvs,
    extras: extras,
  }
}

// put nano util here.
function nanoArgvStraSimple(input: string | string[]) {
  return Array.isArray(input) ? input : input.split(/ +/)
}

/**
 *
 * @sample
 * ```ts
 * nanoValIsOneOfThem(value, [undefined, true, "", "true"])
 * nanoValIsOneOfThem(value, ["", "true"])
 * ```
 */
function nanoValIsOneOfThem(one: unknown, them: unknown[], caseSensitive: boolean = false) {
  return them.some((exp) => {
    // ignore case when string
    if (caseSensitive && typeof exp === 'string' && typeof one === 'string') {
      return exp.toLowerCase() == one.toLowerCase()
    }
    return exp == one
  })
}

// nano 's util,api,shared
/**
 *
 * @sample
 * ```ts
 * nanoArgsStraDecode(process.argv)
 * nanoArgsStraDecode(arrayify(`zero -- code`)) // {head:['zero'],tail:['code']}
 * ```
 */
function nanoArgsStraDecode(handledInput: string[]) {
  let head: string[] = handledInput
  let tail: string[] = []
  // feat(core): support extras when '--' bind to ouput.extras
  // 1. get the first index
  const theFirstIndex = handledInput.indexOf('--')

  // 2. get extras and head when index >= 0
  if (handledInput.includes('--')) {
    tail = handledInput.slice(theFirstIndex + 1)
    head = handledInput.slice(0, theFirstIndex)
  }
  return { tail, head }
}

/**
 *
 * @sample
 * ```
 * nanoArgsHeadDecode(['code','--color=true']) //{argvs:['code'],argsMap:[['--color','true']]}
 * nanoArgsHeadDecode(['code','--color']) //{argvs:['code'],argsMap:[['--color','true']]}
 * nanoArgsHeadDecode(['code','--no-color']) //{argvs:['code'],argsMap:[['--no-color','true']]}
 * nanoArgsHeadDecode(['code','-xyz']) //{argvs:['code'],argsMap:[['x','true'],['y','true'],['z','true']]}
 * ```
 */
function nanoArgsHeadDecode(args: string[]) {
  // 4. get argv and args map from head
  const argvs: NanoArgvs = []
  const argsMap: NanoArgsMap = []
  /* eslint-disable no-plusplus */
  for (let i = 0; i < args.length; i++) {
    const previous = args[i - 1]
    const curr = args[i]
    const next = args[i + 1]

    // eg:ymc.rc.json
    const nextIsValue = next && !/^--.+/.test(next) && !/^-.+/.test(next)

    const pushWithNext = (x: string) => {
      //[string,boolean]
      //[string,string]
      // argsMap.push([x, nextIsValue ? next : true])
      argsMap.push([x, nextIsValue ? next : 'true'])
    }

    // case: key val exp. eg:--conf=ymc.rc.json -f=ymc.rc.json
    if (/^--.+=/.test(curr) || /^-.=/.test(curr)) {
      //string[]
      // argsMap.push(curr.split('='))
      const [key, value] = curr.split('=')
      argsMap.push([key, value])
    } else if (/^-[^-].*/.test(curr)) {
      //case: key exp . eg: -xyz

      let current = curr

      if (current.includes('=')) {
        const index = current.indexOf('=')
        argsMap.push([current.slice(index - 1, index), current.slice(index + 1, index + 2)])
        current = current.slice(0, index - 1) + current.slice(index + 2)
      }

      // Push all the flags but the last (ie x and y of -xyz) with true
      const xyz = current.slice(1).split('').slice(0, -1)
      // eslint-disable no-restricted-syntax
      for (const char of xyz) {
        //[string,true]
        argsMap.push([char, 'true'])
        // argsMap.push([char, true])
      }

      // If the next string is a value, push it with the last flag
      const final = current[current.length - 1]
      pushWithNext(final)
    } else if (/^--.+/.test(curr) || /^-.+/.test(curr)) {
      //case: key val exp . eg: -help true, --help true, -h true
      pushWithNext(curr)
    } else {
      let valueTaken = argsMap.find((arg) => arg[0] === previous)

      if (!valueTaken && /^-./.test(previous)) {
        const previousChar = previous[previous.length - 1]
        valueTaken = argsMap.find((arg) => arg[0] === previousChar)
      }
      //case: only key or  exp . eg: a b c
      if (!valueTaken) {
        argvs.push(curr)
      }
    }
  }
  return { argvs, argsMap }
}

/**
 *
 * @sample
 * ```ts
 * nanoDecodeArgsMap([['--name','zero']]) //{name:'zero'}
 * nanoDecodeArgsMap([['--color',true]]) // {color:true}
 * nanoDecodeArgsMap([['--no-color',true]]) // {color:false}
 * nanoDecodeArgsMap([['--no-color',undefined]]) // {color:false}
 * ```
 */
function nanoArgsHeadKvpDecode(argsMap: NanoArgsMap) {
  // 1. init result as NanoFlags
  const result: NanoFlags = {}
  // 2. get flag for each item in map
  for (const item of argsMap) {
    // 2.1 get key of item and delete head - or -- of js-string key
    let key: string = item[0].replace(/^-{1,2}/g, '')
    // 2.2 get value of item
    let value = item[1] //string|boolean|number|undefined
    // 2.3 set color to false  when '--no-color true' or '--no-color'
    if (key.startsWith('no-') && nanoValIsOneOfThem(value, [undefined, true, '', 'true'])) {
      key = key.slice(3)
      // value = false
      value = 'false'
    }
    // 2.4 parse string value to number,boolean or string
    // result[key] = parseValue(value)
    result[key] = value
  }
  return result
}

/**
 *
 * @sample
 * ```ts
 * let passedflag = nanoFlagParse(flag)
 *
 * // todo:
 * // let jssvflag:NanoParsedFlags =(flag,nanoStrvParse)
 *
 * // let strvflag:NanoFlags =(flag,identy)
 * ```
 *
 */
function nanoFlagParse(
  flag: NanoStrvFlag | NanoJssvFlag,
  util: {
    nanoStrvParse: (v: string | NanoParsedValue) => string | NanoParsedValue
  } = {
    nanoStrvParse: StrvParse,
  },
): NanoParsedFlags {
  const res: NanoParsedFlags = {}
  Object.keys(flag).forEach((key) => (res[key] = util.nanoStrvParse(flag[key])))
  return res
}

// feat(core): getNanoFromStra - get nano from stra
export function getNanoFromStra(stra: string[]) {
  return nanoargs(stra)
}

// feat(core): getStraFromStro - get nano from stro
export function getStraFromStro(stro: string) {
  return nanoArgvStroToStra(stro)
}

// feat(core): mock process.argv
// idea(core): like using argv as func input
/**
 * argv stro to argv arro
 * @sample
 * ```ts
 * let input:string=''
 * input = `you say -hi --name 'ye mian cheng' --first-name ye --old-name "ye min cong"`
 * argv = MockProcessAgrv(input)
 * //["you","say","-hi","--name","ye mian cheng","--first-name","ye","--old-name","ye min cong"]
 * // why use ?
 * // mock process.argv
 * // like using argv as func input
 * ```
 */
export function nanoArgvStroToStra(s: string) {
  const res: string[] = []
  const input = s.trim()
  // code(core): no need to nomalize quotation text
  // input = NomalizeQuotationText(input)

  const list: string[] = input.split(/ /)
  let currIsItem: boolean = true
  let cacheCurr: string[] = []
  // /* eslint-disable no-plusplus */
  for (let i = 0; i < list.length; i++) {
    // const previous = list[i - 1]
    const curr = list[i]

    // const next = list[i + 1]
    // code(core): detect curr type
    // code(core): no to detect curr type when currIsItem false
    if (currIsItem) {
      currIsItem = StartWithQuotation(curr) ? false : true
    }

    // case is [`"xx`,`xx"`]
    if (!currIsItem) {
      if (EndsWithQuotation(curr)) {
        // case is [`"xx"`] or [`'xx`]
        currIsItem = true
        res.push(TrimQuotation([...cacheCurr, curr].join(' ')))
        cacheCurr = []
      } else {
        // case is [`"xx`,`xx"`]
        cacheCurr.push(curr)
      }
    } else {
      // case is [`xx`] or [`xx`]
      res.push(TrimQuotation(curr))
    }
    // log middleware cache value
    // log(curr, currIsItem, cacheCurr)
  }
  return res
}
// nano-

/**
 * @sample
 * ```ts
 * TrimQuotation(`"ye mian cheng"`)//'ye mian cheng'
 * TrimQuotation(`'ye mian cheng'`)//'ye mian cheng'
 * ```
 */
export function TrimQuotation(s: string) {
  return s.replace(/(^("|'))|(("|')$)/g, '')
}

/**
 * @sample
 * ```ts
 * EndsWithQuotation(`"ye min cong"`)//true
 * EndsWithQuotation(`'ye min cong'`)//true
 * EndsWithQuotation(`'ye min cong`)//false
 * EndsWithQuotation(`ye min cong`)//false
 * ```
 */
export function EndsWithQuotation(s: string) {
  return /("|')$/.test(s)
}

/**
 * @sample
 * ```ts
 * StartWithQuotation(`"ye min cong"`)//true
 * StartWithQuotation(`'ye min cong'`)//true
 * StartWithQuotation(`'ye min cong`)//false
 * StartWithQuotation(`ye min cong`)//false
 * ```
 */
export function StartWithQuotation(s: string) {
  return /^("|')/.test(s)
}

/**
 * is wraped with quotation or correct quotation
 * @sample
 * ```ts
 * WrapedQuotation(`"ye mian cheng"`)//true
 * WrapedQuotation(`'ye mian cheng'`)//true
 * WrapedQuotation(`ye min cong`)//false
 * WrapedQuotation(`'ye mian cheng"`)//false
 * ```
 */
export function WrapedQuotation(s: string) {
  return /(^"[^"]*"$)|(^'[^']*'$)/.test(s)
}

/**
 * @sample
 * ```ts
 * WrapQuotation(`"ye mian cheng"`,'"')//'"ye mian cheng"'
 * WrapQuotation(`ye mian cheng`,'"')//'"ye mian cheng"'
 * ```
 */
export function WrapQuotation(s: string, quote: string = '"') {
  return [quote, TrimQuotation(s), quote].join('')
}

/**
 * find text that wrap with correct quotation
 * @sample
 * ```ts
 * FindQuotationText('--name "ye mian cheng"')//['ye mian cheng']
 * // you can use it to get space item when use cli-args-like as input
 * // find , read . load or parse , which is the best name ?
 * ```
 */
export function FindQuotationText(s: string, trimQuote: boolean = false) {
  // feat(core): def regexp to match quotes space item
  // feat(core): support quotes with double quotes
  // feat(core): support quotes with single quotes
  const reg = /("[^"]*")|('[^']*')/g

  // feat(core): get match of  quotes space item
  const spaceItemList: string[] | null = s.match(reg)
  // feat(core): trim quotation when match & enable trim-quote
  return spaceItemList ? (trimQuote ? spaceItemList.map((v) => TrimQuotation(v)) : spaceItemList) : []
}

/**
 * find text that not wrap with correct quotation
 * @sample
 * ```ts
 * FindNoQuotationText('--name "ye mian cheng"')//['--name']
 * ```
 */
export function FindNoQuotationText(s: string) {
  const otherItem = s.trim().replace(/("[^"]*")|('[^']*')/g, '')
  return otherItem.split(/ +/).filter((v) => v)
}

/**
 * trim quotation text
 * @sample
 * ```
 * let s = `-hi --name 'ye mian cheng'`
 * let input = TrimQuotationText(s.trim()) //`-hi --name ye mian cheng`
 * ```
 */
export function TrimQuotationText(s: string) {
  let res: string = s
  // feat(core): trim quotation of args-oline
  const spaceItemList = FindQuotationText(res, false)
  for (let index = 0; index < spaceItemList.length; index++) {
    const spaceItem = spaceItemList[index]
    res = res.replace(spaceItem, TrimQuotation(spaceItem))
  }
  return res
}

/**
 * make double or single quotation text
 * @sample
 * ```
 * let s = `-hi --name 'ye mian cheng'`
 * let input = NomalizeQuotationText(s.trim()) //`-hi --name "ye mian cheng"`
 * // why use ?
 * // trim quotation of args-oline
 * ```
 */
export function NomalizeQuotationText(
  s: string,
  quoteType: 'single-quotation' | 'double-quotation' = 'double-quotation',
) {
  let res: string = s
  // feat(core): nomalize quotation of args-oline
  const spaceItemList = FindQuotationText(res, false)
  for (let index = 0; index < spaceItemList.length; index++) {
    const spaceItem = spaceItemList[index]
    res = res.replace(spaceItem, WrapQuotation(spaceItem, quoteType === 'double-quotation' ? `"` : `'`))
  }
  return res
}

export interface KcOption {
  noAutoCamelize: boolean
  slim: boolean
  camelize?: (...args: any[]) => string
}
export type KcOptionLike = Partial<KcOption>
// export type NanoParsedValue = boolean | number | undefined | null | string;
// export type NanoFlagLike = Record<string, NanoParsedValue>;
export function nanoFlagKeysCamelize(data: NanoFlagLike = {}, opts: KcOptionLike = {}) {
  // let res = {}
  const option: KcOption = {
    slim: true,
    noAutoCamelize: false,
    ...opts,
  }
  if (option.noAutoCamelize) return data
  let fn = opts.camelize ? opts.camelize : camelize
  Object.keys(data).forEach((k) => {
    const ck = fn(k)
    // res[ck]=flags[k]
    if (ck !== k) {
      data[ck] = data[k]
      if (option.slim) {
        delete data[k]
      }
    }
  })
  return data
}

// feat(core): shim - exclude value of flag
/**
 * get obj val that not in values  - ts exclude like
 * @sample
 * ```
 * // {a:'b',c:'',d:undefined} ->  {a:'b',c:''}
 * nanoFlagShimValueExclude({a:'b',c:'',d:undefined},[undefined])
 * ```
 */
export function nanoFlagShimValueExclude(data: NanoFlagLike, values: any = [undefined]) {
  const res: NanoFlagLike = {}
  Object.keys(data).forEach((v) => {
    let val = data[v]
    if (!values.includes(val)) {
      res[v] = val
    }
  })
  return res
}

export function putFlag<T>(...flags: object[]) {
  let flag: object = {}
  for (let index = 0; index < flags.length; index++) {
    const item = flags[index]
    flag = {
      ...flag,
      ...item,
    }
  }
  return flag as unknown as T
}

// feat(core): getNanoFromArgvAndDefaultFlag
// feat(core): pass stra as the first arg
// feat(core): pass version and usage
// feat(core): custom undefined values
/**
 *
 * @sample
 * ```
 * getNanoFromArgvAndDefaultFlag(argv,builtinBurnBucketOption,[undefined, ''])
 * ```
 */
export function getNanoFromArgvAndDefaultFlag(argv: string[], defaulFlag: object, undefinedValue = [undefined, '']) {
  let flagKeys = Object.keys(defaulFlag)

  let defFlag = { ...defaulFlag }
  // underscode -> lowercase -> camel
  let ulc = (v: string) => camelize(v.toLowerCase())

  let envflag = nanoFlagKeysCamelize(getFlagFromProcessEnv(flagKeys), {
    camelize: ulc,
  })

  // cli or api flag
  let cliNano = getNanoFromStra(argv)

  let cliFlag = nanoFlagKeysCamelize(cliNano.flags)
  // todo: get flag in position
  // ...
  // do you real need position flag ?
  // cliNano.argv + flagIndexPosition -> pozFlag

  //cli flag-> process.env flag -> default-flag  ->
  // let flag = {
  //   ...defFlag,
  //   ...nanoFlagShimValueExclude(envflag, undefinedvalue),
  //   ...nanoFlagShimValueExclude(cliFlag, undefinedvalue)
  // }
  let flag = putFlag<NanoFlags | NanoParsedFlags>(
    defFlag,
    nanoFlagShimValueExclude(envflag, undefinedValue),
    nanoFlagShimValueExclude(cliFlag, undefinedValue),
  )
  cliNano.flags = flag
  return cliNano
}

export function getPoszFlagFromArgv(argv: string[], posz: Record<string, number>) {
  let keys = Object.keys(posz)
  let flag: Record<string, any> = {}
  for (let index = 0; index < keys.length; index++) {
    let fk = keys[index]
    const nk = posz[keys[index]]
    let nv = nk > -1 && nk < argv.length ? argv[nk] : undefined
    if (nv != undefined) {
      flag[fk] = nv
    }
  }
  return flag
}
// get posz from utxt
