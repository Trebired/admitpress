import QtQuick

QtObject {
  id: grid

  property var logic

  function gridStepMm(axisMm) {
    return Math.min(logic.editor.gridTargetMm, Math.max(0.5, axisMm))
  }

  function gridMinimumK(axisMm) {
    const step = gridStepMm(axisMm)
    return Math.ceil(-axisMm / (2 * step) - 0.5 + 0.000001)
  }

  function gridMaximumK(axisMm) {
    const step = gridStepMm(axisMm)
    return Math.floor(axisMm / (2 * step) - 0.5 - 0.000001)
  }

  function gridLineCount(axisMm) {
    return Math.max(0, gridMaximumK(axisMm) - gridMinimumK(axisMm) + 1)
  }

  function gridLineMm(index, axisMm) {
    const step = gridStepMm(axisMm)
    const k = gridMinimumK(axisMm) + index
    return axisMm / 2 + (k + 0.5) * step
  }

  function isMajorGridLine(index, axisMm) {
    const k = gridMinimumK(axisMm) + index
    return Math.abs(k) % 4 === 0
  }

  function snapAxisCoordinateMm(value, axisMm) {
    if (!logic.editor.snapToGrid) return value
    const step = gridStepMm(axisMm)
    const center = axisMm / 2
    const snapped = center + (Math.round((value - center) / step - 0.5) + 0.5) * step
    if (snapped < step / 2) return 0
    if (snapped > axisMm - step / 2) return axisMm
    return snapped
  }

  function snapAxisLengthMm(value, axisMm) {
    if (!logic.editor.snapToGrid) return value
    const step = gridStepMm(axisMm)
    return Math.max(step, Math.round(value / step) * step)
  }

  function snapXMm(value) {
    return snapAxisCoordinateMm(value, logic.ticketWidthMm())
  }

  function snapYMm(value) {
    return snapAxisCoordinateMm(value, logic.ticketHeightMm())
  }

  function snapXLengthMm(value) {
    return snapAxisLengthMm(value, logic.ticketWidthMm())
  }

  function snapYLengthMm(value) {
    return snapAxisLengthMm(value, logic.ticketHeightMm())
  }
}
