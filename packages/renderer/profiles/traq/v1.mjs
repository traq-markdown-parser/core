import { plugin as common } from "../../common.mjs";
import { plugin as generic } from "../../extensions/generic.mjs";
import { plugin as trap } from "../../extensions/trap.mjs";
import { PresetBuilder } from "../../preset.mjs";

export function html(options) {
  return new PresetBuilder()
    .add(common(options))
    .add(generic())
    .add(trap(options))
    .build();
}
