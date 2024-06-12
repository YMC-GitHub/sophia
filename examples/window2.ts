import { Window } from '../index'

const TAU = Math.PI * 2

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

async function main() {
  const chrome = await Window.findWindowByClassName('Chrome_WidgetWin_1')
  console.log(await chrome?.getTitle())
}

main()
