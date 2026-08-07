pragma Singleton

import QtQml

QtObject {
  id: localeText

  property var backend: null
  readonly property string localeName: Qt.locale().name
  readonly property string language: localeName.toLowerCase().replace("-", "_").split("_")[0]
  readonly property bool isCzech: language === "cs"
  readonly property var monthKeys: [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
  ]
  readonly property var weekdayKeys: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]

  function tr(source) {
    const key = String(source || "")
    if (backend && backend.translate) return backend.translate(key, localeName)
    return key
  }

  function monthName(month) {
    const index = Math.max(0, Math.min(11, Number(month || 0)))
    return tr(monthKeys[index])
  }

  function weekdayName(index) {
    const normalized = Math.max(0, Math.min(6, Number(index || 0)))
    return tr(weekdayKeys[normalized])
  }
}
