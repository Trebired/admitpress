import QtQuick

QtObject {
  id: selection

  property var editor
  property var fieldsModel
  property var logic
  property var selectedMap: ({})
  property int revision: 0

  function validIndex(index) {
    return fieldsModel && index >= 0 && index < fieldsModel.count
  }

  function selectedCount() {
    revision
    let count = 0
    for (let key in selectedMap) {
      if (selectedMap[key] && validIndex(Number(key))) count += 1
    }
    return count
  }

  function isSelected(index) {
    revision
    return validIndex(index) && !!selectedMap[index]
  }

  function hasSelection() {
    return selectedCount() > 0
  }

  function selectedIndices() {
    revision
    const indices = []
    for (let key in selectedMap) {
      const index = Number(key)
      if (selectedMap[key] && validIndex(index)) indices.push(index)
    }
    indices.sort(function(left, right) {
        return left - right
    })
    return indices
  }

  function normalizeIndices(indices) {
    const next = []
    const seen = ({})
    for (let value of indices || []) {
      const index = Number(value)
      if (!validIndex(index) || seen[index]) continue
      seen[index] = true
      next.push(index)
    }
    next.sort(function(left, right) {
        return left - right
    })
    return next
  }

  function clear() {
    selectedMap = ({})
    if (editor) editor.selectedFieldIndex = -1
    revision += 1
  }

  function selectSingle(index) {
    if (!validIndex(index)) {
      clear()
      return
    }
    const next = ({})
    next[index] = true
    selectedMap = next
    editor.selectedFieldIndex = index
    revision += 1
  }

  function selectIndices(indices) {
    const normalized = normalizeIndices(indices)
    if (normalized.length === 0) {
      clear()
      return
    }
    const next = ({})
    for (let index of normalized) next[index] = true
    selectedMap = next
    editor.selectedFieldIndex = normalized[0]
    revision += 1
  }

  function addIndices(indices) {
    const normalized = normalizeIndices(indices)
    if (normalized.length === 0) return
    const next = Object.assign({}, selectedMap)
    for (let index of normalized) next[index] = true
    selectedMap = next
    if (!isSelected(editor.selectedFieldIndex)) editor.selectedFieldIndex = normalized[0]
    revision += 1
  }

  function firstSelectedIndex() {
    const indices = selectedIndices()
    return indices.length > 0 ? indices[0] : -1
  }

  function setPrimary(index) {
    if (!validIndex(index)) return
    editor.selectedFieldIndex = index
  }

  function toggle(index) {
    if (!validIndex(index)) return
    const next = Object.assign({}, selectedMap)
    if (next[index]) delete next[index]
    else next[index] = true
    selectedMap = next
    if (next[index]) editor.selectedFieldIndex = index
    else if (!isSelected(editor.selectedFieldIndex)) editor.selectedFieldIndex = firstSelectedIndex()
    revision += 1
  }

  function selectForPress(index, modifiers) {
    const shiftPressed = (modifiers & Qt.ShiftModifier) !== 0
    if (shiftPressed) {
      toggle(index)
      return isSelected(index)
    }
    if (!isSelected(index) || selectedCount() <= 1) {
      selectSingle(index)
    } else {
      setPrimary(index)
    }
    return isSelected(index)
  }

  function indicesIntersectingRect(leftMm, topMm, rightMm, bottomMm) {
    const indices = []
    const left = Math.min(leftMm, rightMm)
    const right = Math.max(leftMm, rightMm)
    const top = Math.min(topMm, bottomMm)
    const bottom = Math.max(topMm, bottomMm)
    for (let index = 0; fieldsModel && index < fieldsModel.count; ++index) {
      const field = fieldsModel.get(index)
      const fieldLeft = Number(field.x_mm || 0)
      const fieldTop = Number(field.y_mm || 0)
      const fieldRight = fieldLeft + Number(field.width_mm || 0)
      const fieldBottom = fieldTop + Number(field.height_mm || 0)
      if (fieldLeft < right && fieldRight > left && fieldTop < bottom && fieldBottom > top) {
        indices.push(index)
      }
    }
    return indices
  }

  function selectRect(leftMm, topMm, rightMm, bottomMm, additive) {
    const indices = indicesIntersectingRect(leftMm, topMm, rightMm, bottomMm)
    if (additive) addIndices(indices)
    else selectIndices(indices)
  }

  function fieldGeometrySnapshot(indices) {
    const values = []
    for (let index of indices) {
      if (!validIndex(index)) continue
      const field = fieldsModel.get(index)
      values.push({
          index: index,
          x_mm: Number(field.x_mm),
          y_mm: Number(field.y_mm),
          width_mm: Number(field.width_mm),
          height_mm: Number(field.height_mm)
      })
    }
    return values
  }

  function moveGeometrySnapshot(snapshot, deltaXMm, deltaYMm, useSnap) {
    for (let item of snapshot) {
      logic.updateFieldGeometry(
        item.index,
        item.x_mm + deltaXMm,
        item.y_mm + deltaYMm,
        item.width_mm,
        item.height_mm,
        useSnap
      )
    }
  }

  function settleGeometrySnapshot(snapshot) {
    for (let item of snapshot) {
      if (!validIndex(item.index)) continue
      const field = fieldsModel.get(item.index)
      logic.updateFieldGeometry(
        item.index,
        field.x_mm,
        field.y_mm,
        field.width_mm,
        field.height_mm,
        true
      )
    }
  }
}
