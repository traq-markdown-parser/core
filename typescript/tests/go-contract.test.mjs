import test from "node:test";
import assert from "node:assert/strict";
import { goContract } from "../../scripts/contracts/go.mjs";

test("named string enums remain compatible with Go string fields", () => {
  assert.equal(
    goContract({ title: "LookupKind", type: "string", enum: ["user", "group"] }),
    "type LookupKind = string\n",
  );
});

test("nullable integer fields preserve their format and constraints", () => {
  const parent = {
    type: ["integer", "null"],
    format: "uint32",
    minimum: 0,
  };
  const schema = {
    title: "Group",
    type: "object",
    additionalProperties: false,
    properties: { parent },
    required: ["parent"],
  };
  assert.match(goContract(schema), /Parent \*uint32/);
  assert.throws(
    () => goContract({ ...schema, properties: { parent: { ...parent, minimum: 1 } } }),
    /Unsupported integer/,
  );
});
