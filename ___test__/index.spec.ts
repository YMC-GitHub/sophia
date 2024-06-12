import test from 'ava'

// import { plus100 } from '../index'

// test('sync function from native code', (t) => {
//   const fixture = 42
//   t.is(plus100(fixture), fixture + 100)
// })

import { fib } from '../index'

test('sync function from native code', (t) => {
  t.is(fib(40), 102334155)
})
