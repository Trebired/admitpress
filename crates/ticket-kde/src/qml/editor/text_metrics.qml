import QtQuick

QtObject {
  function estimateTextUnits(value, fontWeight) {
    let units = 0
    for (let index = 0; index < value.length; ++index) {
      const character = value[index]
      if (character === " ") units += 0.32
      else if (".,:;'\"!|".indexOf(character) >= 0) units += 0.26
      else if ("ijIl1".indexOf(character) >= 0) units += 0.32
      else if ("ftr-/".indexOf(character) >= 0) units += 0.40
      else if ("mwMW@".indexOf(character) >= 0) units += 0.82
      else if (character >= "0" && character <= "9") units += 0.56
      else if (character >= "A" && character <= "Z") units += 0.64
      else units += 0.56
    }
    return units * (fontWeight >= 700 ? 1.08 : 1.0)
  }

  function textLines(value) {
    return String(value || "").split(/\r?\n/)
  }

  function lineBlockUnits(lineCount) {
    return 1 + Math.max(0, lineCount - 1) * 1.18
  }

  function fittedFontSizeMm(field, value) {
    if (!field) return 3.5
    const lines = textLines(value)
    const lineCount = Math.max(1, lines.length)
    const requested = Math.max(0.8, Number(field.font_size_pt || 12) * 25.4 / 72)
    const maxForHeight = Math.max(0.8, Number(field.height_mm || 8) * 0.82 / lineBlockUnits(lineCount))
    let fontSize = Math.min(requested, maxForHeight)
    let widestLineUnits = 0
    for (let index = 0; index < lines.length; ++index) {
      widestLineUnits = Math.max(widestLineUnits, estimateTextUnits(lines[index], Number(field.font_weight || 400)))
    }
    const estimatedWidth = widestLineUnits * fontSize
    const availableWidth = Math.max(1, Number(field.width_mm || 20) - 0.6)
    if (estimatedWidth > availableWidth) fontSize *= availableWidth / estimatedWidth
    return Math.max(0.8, fontSize)
  }
}
