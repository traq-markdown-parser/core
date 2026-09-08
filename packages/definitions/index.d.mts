export class PluginGroup {
  private readonly identity;
  constructor(name: string);
  readonly name: string;
  readonly parent: PluginGroup | null;
  group(name: string): PluginGroup;
  new(name: string): Plugin;
}
/** Shared declaration, independent of parser and renderer implementations. */
export class Plugin {
  private readonly identity;
  constructor(name: string);
  static group(name: string): PluginGroup;
  readonly name: string;
  readonly namespace: PluginGroup | null;
}
