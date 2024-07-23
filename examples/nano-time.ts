export function oneOfDay(list: string | string[], input: number | string) {
  let days = Array.isArray(list) ? list : list.split(",");
  // let cwi = input + 1
  if (!days.includes(`${input}`)) return false;
  return true;
}

/**
 *
 * @sample
 * ```
 * let now = new Date()
 * let timenow = formatDate(`HH:mm:ss`, now)
 * timeInRange(timenow, `20:00:00->22:00:00`)
 * timeInRange(timenow, `20:00:00,22:00:00`)//todo
 * timeInRange(timenow, `[20:00:00,22:00:00]`)//todo
 * timeInRange(timenow, `(20:00:00,22:00:00)`)//todo
 * ```
 */
export function timeInRange(time: string, range: string, today = `2017-06-06`) {
  function getSeconds(formatedTime: string) {
    // new Date('2017-06-06 15:31:09').getTime();
    return new Date(`${today} ${formatedTime}`).getTime();
  }

  //trim
  let trimedRg = range.trim();
  //del special exp char []()
  let validRannge = trimedRg.replace(/^[\[\(]/i, "").replace(/^[\]\)]/i, "");
  let [s, e] = validRannge
    .split(/->|,/i)
    .map((v) => v.trim())
    .filter((v) => v);
  // log(s, e, time)
  let [min, max, inp] = [getSeconds(s), getSeconds(e), getSeconds(time)];
  // log([min, max, inp])

  let A: boolean = false;
  let B: boolean = false;
  if (trimedRg.startsWith("[")) {
    A = min <= inp;
  } else if (trimedRg.startsWith("(")) {
    A = min < inp;
  } else {
    A = min <= inp;
  }

  if (trimedRg.endsWith("]")) {
    B = inp <= max;
  } else if (trimedRg.endsWith(")")) {
    B = inp < max;
  } else {
    B = inp <= max;
  }
  return A && B;
  // return min <= inp && inp <= max
}
/**
 * format date
 * @param {string} fmt
 * @returns {string}
 * @sample
 * ```
 * let now = new Date();
 * formatDate("yyyy-MM-dd HH:mm:ss",now);
 * ```
 * @description
 * ```
 * M+
 * ```
 */
export function formatDate(fmt: string, ctx: Date) {
  let res = fmt;
  // let ctx = this;
  const o: Record<string, any> = {
    "M+": ctx.getMonth() + 1,
    "d+": ctx.getDate(),
    "H+": ctx.getHours(),
    "m+": ctx.getMinutes(),
    "s+": ctx.getSeconds(),
    "S+": ctx.getMilliseconds(),
    "W+": ctx.getDay(),
  };
  let reg;
  reg = /(y+)/;
  if (reg.test(res)) {
    res = res.replace(reg, (x) =>
      `${ctx.getFullYear()}`.substring(4 - x.length)
    );
  }
  /* eslint-disable no-restricted-syntax,guard-for-in */
  for (const k in o) {
    reg = new RegExp(`(${k})`);
    if (reg.test(res)) {
      res = res.replace(reg, (x) =>
        x.length === 1 ? o[k] : `00${o[k]}`.substring(String(o[k]).length)
      );
    }
  }
  return res;
}
