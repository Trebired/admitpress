type OpenTag = {
  end: number;
  name: string;
  selfClosing: boolean;
};

function validateXmlText(text: string): string | null {
  const stack: string[] = [];
  let rootSeen = false;
  let index = 0;
  while (index < text.length) {
    const nextOpen = text.indexOf("<", index);
    if (nextOpen < 0) break;
    const skipped = skipSpecialMarkup(text, nextOpen);
    if (skipped > nextOpen) {
      index = skipped;
      continue;
    }
    if (text.startsWith("</", nextOpen)) {
      const closing = readClosingTag(text, nextOpen);
      if (typeof closing === "string") return closing;
      if (stack.pop() !== closing.name)
      return `mismatched closing tag ${closing.name}`;
      index = closing.end + 1;
      continue;
    }
    const opening = readOpeningTag(text, nextOpen);
    if (typeof opening === "string") return opening;
    rootSeen = true;
    if (!opening.selfClosing) stack.push(opening.name);
    index = opening.end + 1;
  }
  if (!rootSeen) return "missing root element";
  return stack.length === 0 ? null : `unclosed tag ${stack.at(-1)}`;
}

function skipSpecialMarkup(text: string, start: number): number {
  if (text.startsWith("<!--", start)) return skipUntil(text, start, "-->");
  if (text.startsWith("<![CDATA[", start)) return skipUntil(text, start, "]]>");
  if (text.startsWith("<?", start)) return skipUntil(text, start, "?>");
  if (text.startsWith("<!", start)) return skipUntil(text, start, ">");
  return start;
}

function skipUntil(text: string, start: number, marker: string): number {
  const end = text.indexOf(marker, start + marker.length);
  return end < 0 ? text.length : end + marker.length;
}

function readClosingTag(text: string, start: number): OpenTag | string {
  const end = text.indexOf(">", start + 2);
  if (end < 0) return "unterminated closing tag";
  const name = text.slice(start + 2, end).trim();
  if (!isXmlName(name)) return `invalid closing tag ${name}`;
  return { end, name, selfClosing: false };
}

function readOpeningTag(text: string, start: number): OpenTag | string {
  const name = readTagName(text, start);
  if (name === null) return "invalid opening tag";
  const end = findTagEnd(text, start + name.length + 1);
  if (end < 0) return `unterminated tag ${name}`;
  const body = text.slice(start, end).trimEnd();
  return { end, name, selfClosing: body.endsWith("/") };
}

function readTagName(text: string, start: number): string | null {
  const match = /^<\s*([A-Za-z_][A-Za-z0-9_.:-]*)/.exec(text.slice(start));
  return match?.[1] ?? null;
}

function findTagEnd(text: string, start: number): number {
  let quote = "";
  for (let index = start; index < text.length; ++index) {
    const character = text[index] ?? "";
    if (quote.length > 0) {
      if (character === quote) quote = "";
      continue;
    }
    if (character === '"' || character === "'") {
      quote = character;
      continue;
    }
    if (character === ">") return index;
  }
  return -1;
}

function isXmlName(value: string): boolean {
  return /^[A-Za-z_][A-Za-z0-9_.:-]*$/.test(value);
}

export { validateXmlText };
