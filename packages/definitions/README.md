# Shared plugin declarations

Declarations carry an instance identity, a display name, and an optional group.
They contain no parser rules, renderer handlers, or Wasm resources.

```ts
import * as definitions from '@traptitech/markdown-definitions';
import * as parser from '@traptitech/markdown-parser';
import * as html from '@traptitech/markdown-renderer';

const generic = definitions.Plugin.group('generic');
const math = generic.new('math');
const syntax = new parser.Plugin(math);
const rendering = new html.Plugin(math);
```

Reuse the group and declaration instances. Creating another group with the same
name creates a different identity. Selected names are checked within their parent
scope when building a grammar or rendering preset; unused declarations do not
participate. Names are diagnostic labels, not transport node identifiers.

Implementations snapshot their registered rules or handlers. Editing an
implementation does not mutate a composition which already adopted it.
