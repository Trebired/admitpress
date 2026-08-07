import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: sudoFixDialog

  property var backend: root ? root.backendObject : null

  title: LocaleText.tr("Repair printer as administrator")
  dialogWidth: 460

  contentItem: ColumnLayout {
    spacing: sudoFixDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("This can start CUPS and enable or accept jobs for the selected printer queue. ")
      + LocaleText.tr("The repair runs as one elevated batch, so your desktop asks for the administrator password once when it starts.")
      wrapMode: Text.WordWrap
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("Run fix")
        icon.name: AppIcons.pick("wizard")
        highlighted: true
        onClicked: {
          sudoFixDialog.close()
          root.settingsDialogObject.runPrinterFix(true)
        }
      }
    }
  }
}
