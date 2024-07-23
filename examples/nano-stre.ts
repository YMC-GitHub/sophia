export function isAllowIndex(
  input: string | number,
  allows: string | string[]
) {
  let list = tostra(allows);
  return list.includes(typeof input === "string" ? input : `${input}`);

  function tostra(allows: string | string[]) {
    return typeof allows === "string"
      ? allows
          .split(",")
          .map((v) => v.trim())
          .filter((v) => v)
      : allows;
  }
}
