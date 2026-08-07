import QtQuick

QtObject {
  id: history

  property var logic
  property var entries: []
  property int position: -1
  property bool restoring: false
  property int limit: 100

  readonly property bool canUndo: position > 0
  readonly property bool canRedo: position >= 0 && position < entries.length - 1

  property Timer sampler: Timer {
    interval: 400
    repeat: true
    running: history.position >= 0
    onTriggered: history.record()
  }

  function currentSnapshot() {
    return logic && logic.persistence ? logic.persistence.snapshot() : ""
  }

  function reset() {
    entries = [currentSnapshot()]
    position = 0
  }

  function record() {
    if (restoring || position < 0) return
    const snapshot = currentSnapshot()
    if (snapshot === entries[position]) return

    const kept = entries.slice(0, position + 1)
    kept.push(snapshot)
    entries = kept.length > limit ? kept.slice(kept.length - limit) : kept
    position = entries.length - 1
  }

  function applyEntry(index) {
    restoring = true
    try {
      logic.persistence.restore(entries[index])
      position = index
    } finally {
      restoring = false
    }
  }

  function undo() {
    record()
    if (!canUndo) return
    applyEntry(position - 1)
  }

  function redo() {
    if (!canRedo) return
    applyEntry(position + 1)
  }
}
