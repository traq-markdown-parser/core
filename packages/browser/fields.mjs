// Small validators shared by the common AST and plugin-owned payload codecs.
export const string = (value) => typeof value === "string";
export const boolean = (value) => typeof value === "boolean";
export const uint = (value) =>
  Number.isInteger(value) && value >= 0 && value <= 0xffffffff;
export const nullable = (check) => (value) => value === null || check(value);
export const oneOf =
  (...values) =>
  (value) =>
    values.includes(value);
export const object = (value) =>
  value !== null && typeof value === "object" && !Array.isArray(value);
export function fields(value, required, optional = {}) {
  return (
    object(value) &&
    Object.entries(required).every(
      ([key, check]) => Object.hasOwn(value, key) && check(value[key]),
    ) &&
    Object.entries(value).every(
      ([key, item]) =>
        Object.hasOwn(required, key) ||
        (Object.hasOwn(optional, key) && optional[key](item)),
    )
  );
}
