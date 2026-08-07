import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "."

RowLayout {
  id: quantityStepper

  property int quantity: 0
  property int minimumQuantity: 0
  property int maximumQuantity: 10000
  property int controlSpacing: Kirigami.Units.smallSpacing
  readonly property int controlWidth: 152 + controlSpacing * 2

  signal quantityModified(int nextQuantity)

  function requestQuantity(rawValue) {
    const numericValue = Number(rawValue)
    const roundedValue = Number.isFinite(numericValue) ? Math.round(numericValue) : 0
    const nextQuantity = Math.max(minimumQuantity, Math.min(maximumQuantity, roundedValue))
    if (nextQuantity === quantity) return
    quantityStepper.quantityModified(nextQuantity)
  }

  spacing: controlSpacing
  Layout.preferredWidth: controlWidth
  Layout.minimumWidth: controlWidth

  NumberInput {
    Layout.preferredWidth: 56
    Layout.preferredHeight: 44
    decimals: 0
    realFrom: quantityStepper.minimumQuantity
    realTo: quantityStepper.maximumQuantity
    realStepSize: 1
    realValue: quantityStepper.quantity
    onValueModified: quantityStepper.requestQuantity(realValue)
  }

  DangerButton {
    Layout.preferredWidth: 48
    Layout.preferredHeight: 44
    text: LocaleText.tr("Decrease")
    showTooltip: false
    enabled: quantityStepper.quantity > quantityStepper.minimumQuantity
    iconOnly: true
    icon.name: AppIcons.pick("remove")
    autoRepeat: true
    autoRepeatDelay: 350
    autoRepeatInterval: 80
    onClicked: quantityStepper.requestQuantity(quantityStepper.quantity - 1)
  }

  Controls.Button {
    Layout.preferredWidth: 48
    Layout.preferredHeight: 44
    enabled: quantityStepper.quantity < quantityStepper.maximumQuantity
    display: Controls.AbstractButton.IconOnly
    icon.name: AppIcons.pick("add")
    autoRepeat: true
    autoRepeatDelay: 350
    autoRepeatInterval: 80
    onClicked: quantityStepper.requestQuantity(quantityStepper.quantity + 1)
  }
}
