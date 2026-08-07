import QtQuick

QtObject {
  id: defaults

  property var logic

  function widthMm(dynamicKind) {
    const kind = dynamicKind === "" ? null : dynamicKind
    const width = kind === "date" ? 28 : kind === "number" ? 24 : 35
    return Math.min(width, Math.max(8, logic.ticketWidthMm() - 4))
  }

  function heightMm(dynamicKind) {
    const kind = dynamicKind === "" ? null : dynamicKind
    const height = kind === "bulk_items" ? 18 : 8
    return Math.min(height, Math.max(4, logic.ticketHeightMm() - 4))
  }

  function imageSizeMm(widthPx, heightPx) {
    const maxWidth = Math.max(1, logic.ticketWidthMm() - 4)
    const maxHeight = Math.max(1, logic.ticketHeightMm() - 4)
    const defaultWidth = Math.min(maxWidth, Math.max(8, logic.ticketWidthMm() * 0.42))
    const defaultHeight = Math.min(maxHeight, Math.max(8, logic.ticketHeightMm() * 0.42))
    const aspect = Number(widthPx || 0) > 0 && Number(heightPx || 0) > 0 ? Number(widthPx) / Number(heightPx) : 1
    let width = defaultWidth
    let height = width / aspect
    if (height > defaultHeight) {
      height = defaultHeight
      width = height * aspect
    }
    return { width: logic.clamp(width, 1, maxWidth), height: logic.clamp(height, 1, maxHeight) }
  }
}
