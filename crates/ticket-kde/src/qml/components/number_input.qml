import QtQuick
import QtQuick.Controls as Controls

Controls.SpinBox {
  id: numberInput

  property int decimals: 1
  property real realFrom: 0
  property real realTo: 9999
  property real realStepSize: 0.1
  property real realValue: 0

  readonly property int factor: Math.pow(10, decimals)

  from: Math.round(realFrom * factor)
  to: Math.round(realTo * factor)
  stepSize: Math.max(1, Math.round(realStepSize * factor))
  value: Math.max(from, Math.min(to, Math.round(realValue * factor)))
  editable: true
  inputMethodHints: Qt.ImhFormattedNumbersOnly

  function parseNumber(text, fallback) {
    const value = Number(String(text).replace(",", "."))
    return Number.isFinite(value) ? value : fallback
  }

  textFromValue: function(value) {
    return Number(value / factor).toFixed(decimals)
  }

  valueFromText: function(text) {
    return Math.max(from, Math.min(to, Math.round(parseNumber(text, realValue) * factor)))
  }

  onValueModified: realValue = value / factor
}
