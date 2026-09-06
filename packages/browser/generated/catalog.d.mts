import type {Grammar,Plugin} from '../index.mjs'
export const catalog: unknown
export interface CatalogViews {
readonly plugins: { readonly "commonmark": { readonly "core": Plugin; readonly "html": Plugin }; readonly "generic": { readonly "linkify": Plugin; readonly "mark": Plugin; readonly "math": Plugin; readonly "strikethrough": Plugin; readonly "table": Plugin }; readonly "trap": { readonly "compat": Plugin; readonly "references": Plugin; readonly "spoiler": Plugin; readonly "stamp": Plugin } }
readonly presets: { readonly "commonmark": Grammar; readonly "traq": { readonly "v1": Grammar } }
}
