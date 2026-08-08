import QtQuick

QtObject {
  readonly property real lineAdvanceFactor: 1.18
  readonly property real ascentFactor: 0.8
  readonly property real heightFillFactor: 0.82
  readonly property real widthPaddingMm: 0.6
  readonly property real minimumFontSizeMm: 0.8
  readonly property real pointToMm: 25.4 / 72

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
    return 1 + Math.max(0, lineCount - 1) * lineAdvanceFactor
  }

  function fittedFontSizeMm(field, value) {
    if (!field) return 3.5
    const lines = textLines(value)
    const requested = Math.max(minimumFontSizeMm, Number(field.font_size_pt || 12) * pointToMm)
    const maxForHeight = Math.max(
      minimumFontSizeMm,
      Number(field.height_mm || 8) * heightFillFactor / lineBlockUnits(lines.length)
    )
    let fontSize = Math.min(requested, maxForHeight)
    let widestLineUnits = 0
    for (let index = 0; index < lines.length; ++index) {
      widestLineUnits = Math.max(
        widestLineUnits,
        estimateTextUnits(lines[index], Number(field.font_weight || 400))
      )
    }
    const estimatedWidth = widestLineUnits * fontSize
    const availableWidth = Math.max(1, Number(field.width_mm || 20) - widthPaddingMm)
    if (estimatedWidth > availableWidth) fontSize *= availableWidth / estimatedWidth
    return Math.max(minimumFontSizeMm, fontSize)
  }

  function blockTopMm(field, fontSizeMm, lineCount) {
    const y = Number(field.y_mm || 0)
    const height = Number(field.height_mm || 8)
    const blockHeight = fontSizeMm * lineBlockUnits(lineCount)
    if (field.vertical_alignment === "center") return y + (height - blockHeight) / 2
    if (field.vertical_alignment === "bottom") return y + height - blockHeight
    return y
  }

  function anchorXMm(field) {
    const x = Number(field.x_mm || 0)
    const width = Number(field.width_mm || 20)
    if (field.horizontal_alignment === "center") return x + width / 2
    if (field.horizontal_alignment === "right") return x + width
    return x
  }

  function layout(field, value) {
    const lines = textLines(value)
    const fontSizeMm = fittedFontSizeMm(field, value)
    return {
      lines: lines,
      fontSizeMm: fontSizeMm,
      lineAdvanceMm: fontSizeMm * lineAdvanceFactor,
      firstBaselineMm: blockTopMm(field, fontSizeMm, lines.length) + fontSizeMm * ascentFactor,
      anchorXMm: anchorXMm(field)
    }
  }
}
