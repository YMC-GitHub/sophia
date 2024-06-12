import b from 'benny'

// import { plus100 } from '../index'

// function add(a: number) {
//   return a + 100
// }

import { fib as fibrs } from '../index'

function fib(n: number): number {
  if (n === 1 || n === 2) {
    return 1
  }
  return fib(n - 1) + fib(n - 2)
}

async function run() {
  // await b.suite(
  //   'Add 100',

  //   b.add('Native a + 100', () => {
  //     plus100(10)
  //   }),

  //   b.add('JavaScript a + 100', () => {
  //     add(10)
  //   }),

  //   b.cycle(),
  //   b.complete(),
  // )

  await b.suite(
    'fib 40',

    b.add('Native fib 40', () => {
      fibrs(40)
    }),

    b.add('JavaScript fib 40', () => {
      fib(40)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
