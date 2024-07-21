export type ArrxOrObjf = any[] | Record<string | number, any>;
// sizes.sort(sortByk("x", 1));
export function objfSortByKey(k: string | number, rev?: number, nsSep = ".") {
  //https://blog.csdn.net/weixin_41192489/article/details/111400551

  let rec: number = ifAnyDoAElseB(rev);
  return function (a: ArrxOrObjf, b: ArrxOrObjf) {
    let ca = typeof k == "number" ? a[k] : getContextValue(a, k, nsSep);
    let cb = typeof k == "number" ? b[k] : getContextValue(b, k, nsSep);

    // if (ca < cb) {
    //     return rev * 1
    // }
    // if (ca > cb) {
    //     return rev * 1
    // }
    // return 0
    // return ca - cb //shengxu
    // return cb - ca //jiangxu
    return rec === 1 ? ca - cb : cb - ca;
  };

  // use a when i not pass or i === a
  function ifAnyDoAElseB(I?: number, A: number = 1, B: number = -1) {
    let res: number = 0;
    if (I === undefined) {
      res = A;
    } else {
      res = I === A ? A : B;
    }
    return res;
  }

  function getContextValue(ctx: any, key: string, nsSep = ".") {
    let p = key.split(nsSep);
    let context = ctx;
    for (let index = 0; index < p.length; index++) {
      const name = p[index];
      context = context[name];
    }
    return context;
  }
}
