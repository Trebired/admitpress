import { readFileSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";
import {
  type Check,
  commandDetail,
  createTemporaryDirectory,
  findExecutable,
  removeDirectory,
  run,
} from "./common.js";
import { validateXmlText } from "./xml.js";

function checkSvgAndRasterization(): Check {
  const sourceText = sampleSvgText();
  const xmlError = validateXmlText(sourceText);
  if (xmlError !== null) {
    return {
      detail: `invalid SVG: ${xmlError}`,
      name: "SVG and rasterization",
      ok: false,
    };
  }
  const executable =
  findExecutable("rsvg-convert") ?? findExecutable("inkscape");
  if (executable === null) {
    return {
      detail: "SVG parsed, rasterizer unavailable in this environment",
      name: "SVG and rasterization",
      ok: true,
    };
  }
  return rasterizeSvgText(sourceText, executable);
}

function rasterizeSvgText(sourceText: string, executable: string): Check {
  const directory = createTemporaryDirectory("ticket-svg-");
  try {
    const source = writeTemporarySvg(sourceText, directory);
    const output = join(directory, "ticket.png");
    const result = rasterCommand(executable, source, output);
    if (result.status !== 0) {
      return {
        detail: commandDetail(result) || "no PNG output",
        name: "SVG and rasterization",
        ok: false,
      };
    }
    return validatePngDimensions(output, executable);
  } finally {
    removeDirectory(directory);
  }
}

function writeTemporarySvg(sourceText: string, directory: string): string {
  const source = join(directory, "ticket.svg");
  writeFileSync(source, sourceText, "utf8");
  return source;
}

function sampleSvgText(): string {
  return [
    '<?xml version="1.0" encoding="UTF-8"?>',
    '<svg xmlns="http://www.w3.org/2000/svg" width="80mm" height="50mm" viewBox="0 0 80 50">',
    '<rect x="1" y="1" width="78" height="48" fill="white" stroke="black"/>',
    '<text x="40" y="44" text-anchor="middle" font-family="sans-serif" font-size="4">0001</text>',
    "</svg>",
  ].join("");
}

function rasterCommand(executable: string, source: string, output: string) {
  if (basename(executable) === "rsvg-convert") {
    return run([
        executable,
        "--format",
        "png",
        "--dpi-x",
        "203",
        "--dpi-y",
        "203",
        "--output",
        output,
        source,
    ]);
  }
  return run([
      executable,
      source,
      "--export-type=png",
      `--export-filename=${output}`,
      "--export-dpi=203",
  ]);
}

function validatePngDimensions(output: string, executable: string): Check {
  const data = readFileSync(output);
  if (data.length < 24 || !hasPngHeader(data)) {
    return {
      detail: "rasterizer did not produce a valid PNG",
      name: "SVG and rasterization",
      ok: false,
    };
  }
  const width = data.readUInt32BE(16);
  const height = data.readUInt32BE(20);
  const expectedWidth = Math.round((80 / 25.4) * 203);
  const expectedHeight = Math.round((50 / 25.4) * 203);
  const ok =
  Math.abs(width - expectedWidth) <= 1 &&
    Math.abs(height - expectedHeight) <= 1;
  return {
    detail: `${basename(executable)} produced ${width}x${height}px, expected about ${expectedWidth}x${expectedHeight}px`,
    name: "SVG and rasterization",
    ok,
  };
}

function hasPngHeader(data: Buffer): boolean {
  const header = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
  return data.subarray(0, 8).equals(header);
}

export { checkSvgAndRasterization };
