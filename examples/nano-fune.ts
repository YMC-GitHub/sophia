// feat(core): getRepeatFns - get a func to repeat fn with number count
export function getRepeatFns(fn: Function, num: number = 1): Function[] {
  return new Array(num)
    .fill(0)
    .map((_, k) => k + 1)
    .map((_) => {
      return async () => {
        return await fn();
      };
    });
}

export type UnknowFunc = (...args: any[]) => any;

// feat(core): chaintask - run async func list with chain
/**
 * chain async task
 */
export async function chaintask(tasks: UnknowFunc[]): Promise<any[]> {
  let res: any[] = [];
  let chain = Promise.resolve();
  // fix Unary operator '++' used       no-plusplus
  /* eslint-disable no-plusplus */
  for (let index = 0; index < tasks.length; index++) {
    const task = tasks[index];
    // fix Unexpected console statement   no-console
    // fix 'v' is defined but never used  no-unused-vars
    /* eslint-disable no-unused-vars,no-console */
    chain = chain
      .then(async (_) => {
        // feat: save each result to res
        res[index] = await task();
        return res[index];
      })
      .catch(console.log);
  }
  await chain;
  return res;
}
