export class MarkdownParseError extends Error {
  constructor(detail) {
    super(detail.code);
    this.name = "MarkdownParseError";
    this.detail = detail;
  }
}
export class GrammarBuildError extends Error {
  constructor(detail) {
    super(
      detail.code +
        ": " +
        (detail.reason ??
          detail.element ??
          [detail.scope, detail.name].filter(Boolean).join(" / ")),
    );
    this.name = "GrammarBuildError";
    this.detail = detail;
  }
}
