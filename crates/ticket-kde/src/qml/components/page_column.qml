pragma Singleton

import QtQml

QtObject {
  function configure(attached, widths) {
    if (!attached) return
    for (let key in widths) {
      if (attached[key] === undefined) continue
      attached[key] = widths[key]
    }
  }

  function resizeMode(columnView, mode) {
    if (!columnView || mode === undefined) return
    columnView.columnResizeMode = mode
  }
}
