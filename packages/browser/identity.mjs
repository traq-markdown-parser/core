export const metadata = new WeakMap();
export function data(value, kind) {
  const state = metadata.get(value);
  if (!state || state.kind !== kind) throw new TypeError("Expected " + kind);
  return state;
}
export function owned(value, kind, runtime) {
  const state = data(value, kind);
  if (state.runtime !== runtime)
    throw new TypeError("Object belongs to a different Runtime");
  return state;
}
