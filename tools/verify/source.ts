import { readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { PROJECT_ROOT, type Check, walkFiles } from "./common.js";

type StripState = {
  blockDepth: number;
  index: number;
  language: string;
  quote: string;
  rawHashes: number;
  result: string[];
  state: "normal" | "lineComment" | "blockComment" | "rawString" | "string";
  text: string;
};

const BLOCK_COMMENT_END_PAIR = "*" + "/";
const BLOCK_COMMENT_START_PAIR = "/" + "*";
const LINE_COMMENT_PAIR = "/" + "/";

function stripStringsAndComments(text: string, language: string): string {
  const data: StripState = {
    blockDepth: 0,
    index: 0,
    language,
    quote: "",
    rawHashes: 0,
    result: [],
    state: "normal",
    text,
  };
  while (data.index < data.text.length) {
    if (data.state === "normal") handleNormal(data);
    else if (data.state === "lineComment") handleLineComment(data);
    else if (data.state === "blockComment") handleBlockComment(data);
    else if (data.state === "rawString") handleRawString(data);
    else handleString(data);
  }
  return data.result.join("");
}

function handleNormal(data: StripState): void {
  const character = data.text[data.index] ?? "";
  const pair = data.text.slice(data.index, data.index + 2);
  if (pair === LINE_COMMENT_PAIR) {
    enterComment(data, "lineComment", 2);
  } else if (pair === BLOCK_COMMENT_START_PAIR) {
    data.blockDepth = 1;
    enterComment(data, "blockComment", 2);
  } else if (startsRawString(data)) {
    enterRawString(data);
  } else if (startsRustCharacter(data)) {
    consumeRustCharacter(data);
  } else if (
    character === '"' ||
      (character === "'" && data.language !== "rust")
  ) {
    data.state = "string";
    data.quote = character;
    data.result.push(" ");
    data.index += 1;
  } else {
    data.result.push(character);
    data.index += 1;
  }
}

function enterComment(
  data: StripState,
  state: "lineComment" | "blockComment",
  length: number,
): void {
  data.state = state;
  data.result.push(" ".repeat(length));
  data.index += length;
}

function startsRawString(data: StripState): boolean {
  const pattern = /^r(#{0,16})"/.exec(data.text.slice(data.index));
  return (
  data.language === "rust" &&
    data.text[data.index] === "r" &&
    pattern !== null
  );
  }

  function enterRawString(data: StripState): void {
  const match = /^r(#{0,16})"/.exec(data.text.slice(data.index));
  if (match === null) return;
  data.rawHashes = match[1]?.length ?? 0;
  data.state = "rawString";
  data.result.push(" ".repeat(match[0].length));
  data.index += match[0].length;
}

function startsRustCharacter(data: StripState): boolean {
  const pattern = /^'(?:\\.|[^\\'\n])'/.exec(data.text.slice(data.index));
return (
data.language === "rust" &&
  data.text[data.index] === "'" &&
  pattern !== null
);
}

function consumeRustCharacter(data: StripState): void {
const character = /^'(?:\\.|[^\\'\n])'/.exec(data.text.slice(data.index));
if (character === null) return;
data.result.push(" ".repeat(character[0].length));
data.index += character[0].length;
}

function handleLineComment(data: StripState): void {
const character = data.text[data.index] ?? "";
if (character === "\n") {
data.state = "normal";
data.result.push("\n");
} else {
data.result.push(" ");
}
data.index += 1;
}

function handleBlockComment(data: StripState): void {
const character = data.text[data.index] ?? "";
const pair = data.text.slice(data.index, data.index + 2);
if (pair === BLOCK_COMMENT_START_PAIR) {
data.blockDepth += 1;
data.result.push("  ");
data.index += 2;
} else if (pair === BLOCK_COMMENT_END_PAIR) {
closeBlockComment(data);
} else {
data.result.push(character === "\n" ? "\n" : " ");
data.index += 1;
}
}

function closeBlockComment(data: StripState): void {
data.blockDepth -= 1;
data.result.push("  ");
data.index += 2;
if (data.blockDepth === 0) data.state = "normal";
}

function handleRawString(data: StripState): void {
const ending = `"${"#".repeat(data.rawHashes)}`;
if (data.text.startsWith(ending, data.index)) {
data.result.push(" ".repeat(ending.length));
data.index += ending.length;
data.state = "normal";
} else {
appendHiddenChar(data);
}
}

function handleString(data: StripState): void {
const character = data.text[data.index] ?? "";
if (character === "\\") {
consumeEscape(data);
} else if (character === data.quote) {
data.result.push(" ");
data.index += 1;
data.state = "normal";
} else {
appendHiddenChar(data);
}
}

function consumeEscape(data: StripState): void {
data.result.push(" ");
if (data.index + 1 < data.text.length) {
data.result.push(data.text[data.index + 1] === "\n" ? "\n" : " ");
data.index += 2;
} else {
data.index += 1;
}
}

function appendHiddenChar(data: StripState): void {
const character = data.text[data.index] ?? "";
data.result.push(character === "\n" ? "\n" : " ");
data.index += 1;
}

function balancedDelimiters(path: string, language: string): [boolean, string] {
const cleaned = stripStringsAndComments(readFileSync(path, "utf8"), language);
const pairs = new Map([
[")", "("],
["]", "["],
["}", "{"],
]);
const stack: [string, number][] = [];
for (let index = 0; index < cleaned.length; ++index) {
const character = cleaned[index] ?? "";
if ("([{".includes(character)) stack.push([character, index]);
else if (
pairs.has(character) &&
  stack.pop()?.[0] !== pairs.get(character)
) {
return [false, `unexpected ${character} at byte ${index}`];
}
}
const last = stack.at(-1);
return last === undefined
? [true, "balanced"]
: [false, `unclosed ${last[0]} at byte ${last[1]}`];
}

function checkSourceStructure(): Check {
const sourceFiles = walkFiles(join(PROJECT_ROOT, "crates")).filter(
(path) => path.endsWith(".rs") || path.endsWith(".qml"),
);
const failures: string[] = [];
for (const path of sourceFiles) {
const language = path.endsWith(".rs") ? "rust" : "qml";
const [ok, detail] = balancedDelimiters(path, language);
if (!ok) failures.push(`${relative(PROJECT_ROOT, path)}: ${detail}`);
}
return {
detail: failures.length === 0 ? "all balanced" : failures.join("; "),
name: "Source delimiter scan",
ok: failures.length === 0,
};
}

export { checkSourceStructure };
