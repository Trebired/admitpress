import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"
import "."

EditorPropertySection {
  id: priceVariants

  property var subticketsModel

  title: "Prices"

  Repeater {
    model: priceVariants.subticketsModel

    delegate: RowLayout {
      required property int index
      required property int amount_minor

      Layout.fillWidth: true
      spacing: Kirigami.Units.smallSpacing

      EditorFormField {
        Layout.fillWidth: true
        label: LocaleText.tr("Price") + " " + (index + 1)

        NumberInput {
          Layout.fillWidth: true
          decimals: 2
          realFrom: 0
          realTo: 999999
          realStepSize: 1
          realValue: Number(amount_minor || 0) / 100
          onValueModified: {
            subticketsModel.setProperty(index, "amount_minor", Math.round(realValue * 100))
            subticketsModel.setProperty(index, "name", "")
            subticketsModel.setProperty(index, "display_label", "")
          }
        }
      }

      DangerButton {
        iconOnly: true
        Layout.alignment: Qt.AlignBottom
        text: LocaleText.tr("Remove price")
        onClicked: subticketsModel.remove(index)
      }
    }
  }

  Controls.Button {
    Layout.fillWidth: true
    text: LocaleText.tr("Add price")
    icon.name: "list-add"
    onClicked: subticketsModel.append({
        id: "",
        name: "",
        amount_minor: 0,
        currency: "",
        display_label: "",
        keyboard_shortcut: null,
        enabled: true,
        sort_order: subticketsModel.count
    })
  }
}
