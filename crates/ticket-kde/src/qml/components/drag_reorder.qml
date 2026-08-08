pragma Singleton

import QtQml

QtObject {
  function moveOnDrop(model, drag, sourceRow, targetIndex) {
    if (drag.source && drag.source !== sourceRow) {
      model.move(drag.source.index, targetIndex, 1)
    }
  }
}
